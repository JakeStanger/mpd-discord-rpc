use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use rand::seq::SliceRandom;
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;
use tokio::task::spawn_blocking;
use tokio::time::interval;

use tracing::{debug, warn};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
static CACHE_FILENAME: &str = "mpd-discord-rpc-uploader.cache";
const CACHE_DURATION: Duration = Duration::from_secs(3600);
const PING_TIMEOUT: Duration = Duration::from_secs(5);
const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);
const UPLOAD_TIMEOUT: Duration = Duration::from_secs(10);

const UPLOADERS: &[(&str, u64)] = &[
    ("litterbox", 200 * 1024 * 1024),
    ("uguu", 128 * 1024 * 1024),
    ("tmpfiles", 100 * 1024 * 1024),
];

#[derive(Debug, Clone)]
struct UploaderHealth {
    is_healthy: bool,
    last_latency: Duration,
}

type HealthMap = Arc<Mutex<HashMap<String, UploaderHealth>>>;

pub struct ImageUploader {
    cache: Mutex<HashMap<PathBuf, (String, SystemTime)>>,
    cache_file: PathBuf,
    user_agents: Vec<String>,
    current_ua: Mutex<usize>,
    health: HealthMap,
}

impl ImageUploader {
    pub fn new() -> Self {
        let user_agents = vec![
            APP_USER_AGENT.to_string(),
            // "curl/8.13.0".to_string(),
            // "PostmanRuntime/7.32.0".to_string(),
            // "Wget/1.21.3".to_string(),
            // "HTTPie/3.2.2".to_string(),
            // "insomnia/2023.5.8".to_string(),
            // "Python-urllib/3.11".to_string(),
        ];
        let mut shuffled = user_agents.clone();
        shuffled.shuffle(&mut rand::thread_rng());

        // Initialise health map
        let health = Arc::new(Mutex::new(HashMap::new()));
        {
            let mut h = health.lock().unwrap();
            for (name, _) in UPLOADERS {
                h.insert(
                    name.to_string(),
                    UploaderHealth {
                        is_healthy: true,
                        last_latency: Duration::ZERO,
                    },
                );
            }
        }

        // Spawn background health checker
        let health_clone = health.clone();
        tokio::spawn(async move {
            let mut ticker = interval(HEALTH_CHECK_INTERVAL);
            loop {
                ticker.tick().await;
                Self::update_health(health_clone.clone()).await;
            }
        });

        // Load persistent cache from temp directory
        let cache_file = std::env::temp_dir().join(CACHE_FILENAME);
        let cache = Self::load_cache(&cache_file).unwrap_or_default();

        Self {
            cache: Mutex::new(cache),
            cache_file,
            user_agents: shuffled,
            current_ua: Mutex::new(0),
            health,
        }
    }

    fn load_cache(path: &Path) -> Option<HashMap<PathBuf, (String, SystemTime)>> {
        let file = File::open(path).ok()?;
        let reader = BufReader::new(file);
        let mut map = HashMap::new();

        for line in reader.lines() {
            let line = line.ok()?;
            let mut parts = line.split('\t');
            let path_encoded = parts.next()?;
            let url_encoded = parts.next()?;
            let ts_str = parts.next()?;

            let path_bytes = BASE64.decode(path_encoded).ok()?;
            let url_bytes = BASE64.decode(url_encoded).ok()?;
            let path_str = String::from_utf8(path_bytes).ok()?;
            let url = String::from_utf8(url_bytes).ok()?;
            let secs: u64 = ts_str.parse().ok()?;

            let dur = Duration::from_secs(secs);
            let timestamp = SystemTime::UNIX_EPOCH.checked_add(dur)?;
            map.insert(PathBuf::from(path_str), (url, timestamp));
        }
        Some(map)
    }

    fn save_cache(&self) {
        let cache = self.cache.lock().unwrap();
        if let Ok(file) = File::create(&self.cache_file) {
            let mut writer = BufWriter::new(file);
            for (path, (url, timestamp)) in cache.iter() {
                if let Ok(dur) = timestamp.duration_since(SystemTime::UNIX_EPOCH) {
                    let secs = dur.as_secs();
                    // Encoding to prevent formatting issues
                    let path_encoded = BASE64.encode(path.to_string_lossy().as_bytes());
                    let url_encoded = BASE64.encode(url.as_bytes());
                    let line = format!("{}\t{}\t{}\n", path_encoded, url_encoded, secs);
                    let _ = writer.write_all(line.as_bytes());
                }
            }
        }
    }

    async fn update_health(health: HealthMap) {
        let client = match Client::builder().timeout(PING_TIMEOUT).build() {
            Ok(c) => c,
            Err(e) => {
                warn!("Failed to build health-check client: {}", e);
                return;
            }
        };

        for (name, _max_size) in UPLOADERS {
            let url = match *name {
                "litterbox" => "https://litterbox.catbox.moe/resources/internals/api.php",
                "uguu" => "https://uguu.se",
                "tmpfiles" => "https://tmpfiles.org",
                _ => continue,
            };

            let start = Instant::now();
            let result = client.head(url).send().await;
            let latency = start.elapsed();
            let is_healthy = result.is_ok();

            if let Err(e) = result {
                debug!("Health check for {name} failed: {e}");
            }

            let mut h = health.lock().unwrap();
            h.insert(
                name.to_string(),
                UploaderHealth {
                    is_healthy,
                    last_latency: latency,
                },
            );
        }
    }

    pub async fn upload_local_file(&self, path: &Path) -> Option<String> {
        let now = SystemTime::now();
        let cached_url = {
            let mut cache = self.cache.lock().unwrap();
            if let Some((url, timestamp)) = cache.get(path) {
                if let Ok(elapsed) = now.duration_since(*timestamp) {
                    if elapsed < CACHE_DURATION {
                        Some(url.clone())
                    } else {
                        tracing::debug!("Removing {} from cache", path.display());
                        cache.remove(path);
                        drop(cache); // release lock before save
                        self.save_cache();
                        None
                    }
                } else {
                    tracing::debug!("Removing {} from cache", path.display());
                    cache.remove(path);
                    drop(cache);
                    self.save_cache();
                    None
                }
            } else {
                None
            }
        };

        if let Some(url) = cached_url {
            tracing::debug!("Cache hit for {} (URL: {})", path.display(), url);
            return Some(url);
        }

        debug!("Preparing to upload {}", path.display());

        let (bytes, mime) = spawn_blocking({
            let path = path.to_path_buf();
            move || extract_picture_from_file(&path)
        })
        .await
        .ok()??;

        let ext = mime_to_extension(&mime).unwrap_or("jpg");
        let filename = format!(
            "cover_{}.{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ext
        );

        let url = self.upload_bytes(&bytes, &filename).await?;

        // Insert into cache with current time
        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(path.to_path_buf(), (url.clone(), SystemTime::now()));
        }
        self.save_cache();

        debug!("Cached {} ({})", path.display(), url);
        Some(url)
    }

    async fn upload_bytes(&self, bytes: &[u8], filename: &str) -> Option<String> {
        let ua = self.get_next_user_agent();
        let client = Client::builder()
            .user_agent(&ua)
            .timeout(UPLOAD_TIMEOUT)
            .build()
            .ok()?;

        // Collect healthy uploaders
        let health_snapshot: Vec<(String, UploaderHealth)> = {
            let h = self.health.lock().unwrap();
            UPLOADERS
                .iter()
                .filter_map(|&(name, max_size)| {
                    if bytes.len() as u64 > max_size {
                        return None;
                    }
                    h.get(name).map(|health| (name.to_string(), health.clone()))
                })
                .collect()
        };

        // Filter healthy ones and order by latency
        let mut candidates: Vec<_> = health_snapshot
            .into_iter()
            .filter(|(_, h)| h.is_healthy)
            .collect();
        candidates.sort_by_key(|(_, h)| h.last_latency);

        debug!(
            "Candidates for upload ({} bytes): {:?}",
            bytes.len(),
            candidates
                .iter()
                .map(|(n, h)| format!("{n}({}ms)", h.last_latency.as_millis()))
                .collect::<Vec<_>>()
        );

        for (name, _) in &candidates {
            let result = match name.as_str() {
                "litterbox" => Self::upload_to_litterbox(&client, bytes, filename, "1h").await,
                "uguu" => Self::upload_to_uguu(&client, bytes, filename).await,
                "tmpfiles" => Self::upload_to_tmpfiles(&client, bytes, filename, "3600").await,
                _ => None,
            };

            if let Some(url) = result {
                return Some(url);
            }

            // Mark as unhealthy
            if let Ok(mut h) = self.health.lock()
                && let Some(health) = h.get_mut(name)
            {
                health.is_healthy = false;
                debug!("Marked {name} as unhealthy due to upload failure");
            }
        }

        None
    }

    async fn upload_to_litterbox(
        client: &Client,
        bytes: &[u8],
        filename: &str,
        time: &str,
    ) -> Option<String> {
        let form = Form::new()
            .text("reqtype", "fileupload")
            .text("time", time.to_string())
            .text("fileNameLength", filename.len().to_string())
            .part(
                "fileToUpload",
                Part::bytes(bytes.to_vec()).file_name(filename.to_string()),
            );

        let resp = client
            .post("https://litterbox.catbox.moe/resources/internals/api.php")
            .multipart(form)
            .send()
            .await
            .ok()?;
        debug!("litterbox.catbox.moe response: {:?}", resp);

        let text = resp.text().await.ok()?;

        if text.starts_with("http") {
            let url = Some(text.trim().to_string());
            debug!("Using URL from litter.catbox.moe: {:?}", url);
            url
        } else {
            None
        }
    }

    async fn upload_to_uguu(client: &Client, bytes: &[u8], filename: &str) -> Option<String> {
        let form = Form::new().part(
            "files[]",
            Part::bytes(bytes.to_vec()).file_name(filename.to_string()),
        );
        let resp = client
            .post("https://uguu.se/upload")
            .multipart(form)
            .send()
            .await
            .ok()?;
        debug!("uguu.se response: {:?}", resp);
        #[derive(Deserialize)]
        struct UguuResponse {
            success: bool,
            files: Vec<UguuFile>,
        }
        #[derive(Deserialize)]
        struct UguuFile {
            url: String,
        }
        let json: UguuResponse = resp.json().await.ok()?;
        if json.success && !json.files.is_empty() {
            let url = json.files[0].url.clone();
            debug!("Using URL from uguu.se: {:?}", url);
            Some(url)
        } else {
            None
        }
    }

    async fn upload_to_tmpfiles(
        client: &Client,
        bytes: &[u8],
        filename: &str,
        time: &str,
    ) -> Option<String> {
        let form = Form::new()
            .part(
                "file",
                Part::bytes(bytes.to_vec()).file_name(filename.to_string()),
            )
            .text("expire", time.to_string());

        let resp = client
            .post("https://tmpfiles.org/api/v1/upload")
            .multipart(form)
            .send()
            .await
            .ok()?;

        debug!("tmpfiles.org response: {:?}", resp);

        #[derive(Deserialize)]
        struct TmpResponse {
            status: String,
            data: Option<TmpData>,
        }

        #[derive(Deserialize)]
        struct TmpData {
            url: String,
        }

        let json: TmpResponse = resp.json().await.ok()?;

        if json.status == "success" {
            let url = json.data.map(|d| {
                d.url
                    .replace("https://tmpfiles.org/", "https://tmpfiles.org/dl/")
            });
            debug!("Using URL from tmpfiles.org: {:?}", url);
            url
        } else {
            None
        }
    }

    fn get_next_user_agent(&self) -> String {
        let mut idx = self.current_ua.lock().unwrap();
        let ua = self.user_agents[*idx].clone();
        *idx = (*idx + 1) % self.user_agents.len();
        ua
    }
}

fn mime_to_extension(mime: &str) -> Option<&'static str> {
    match mime {
        "image/jpeg" | "image/jpg" => Some("jpg"),
        "image/png" => Some("png"),
        "image/gif" => Some("gif"),
        "image/webp" => Some("webp"),
        "image/bmp" => Some("bmp"),
        _ => None,
    }
}

fn extract_picture_from_file(path: &Path) -> Option<(Vec<u8>, String)> {
    use lofty::file::TaggedFileExt;
    let tagged_file = lofty::read_from_path(path).ok()?;
    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag())?;

    let picture = tag.pictures().first()?;
    let mime = picture.mime_type()?;
    let mime_str = mime.as_str().to_string();
    Some((picture.data().to_vec(), mime_str))
}
