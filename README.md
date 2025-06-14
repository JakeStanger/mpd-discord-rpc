# MPD Discord RPC

![crates.io](https://img.shields.io/crates/v/mpd-discord-rpc)
![aur](https://img.shields.io/aur/version/mpd-discord-rpc-git)

Displays your currently playing song / album / artist from MPD in Discord using Rich Presence,
along with the album art.

It includes support for multiple MPD hosts if, like me, you have more than one server you alternate between.

The program does not require MPD or Discord to be running in order to run.

Once installed just run `mpd-discord-rpc`.

![status image](https://user-images.githubusercontent.com/5057870/174365384-f8ce4aae-cd99-4177-9304-da757206015a.png)

## Installation

### Cargo

The cargo package can be found [here](https://crates.io/crates/mpd-discord-rpc).

```
cargo install mpd-discord-rpc
```

### Arch Linux

Two AUR packages are available:

- [mpd-discord-rpc](https://aur.archlinux.org/packages/mpd-discord-rpc)
- [mpd-discord-rpc-git](https://aur.archlinux.org/packages/mpd-discord-rpc-git)

The systemd unit is included and can be started with:

```
systemctl --user enable --now mpd-discord-rpc 
```

### NixOS

You can use the [`mpd-discord-rpc`](https://search.nixos.org/packages?channel=unstable&show=mpd-discord-rpc&from=0&size=50&sort=relevance&type=packages&query=mpd-discord-rpc) package in nixpkgs. Users of home-manager can also use the [`services.mpd-discord-rpc.enable`](https://github.com/nix-community/home-manager/blob/master/modules/services/mpd-discord-rpc.nix) option.

Many thanks to [Ilan Joselevich](https://github.com/Kranzes) for maintaining both of those.

## Configuration

Running the program once will generate a default configuration file. On Linux this will be at `~/.config/discord-rpc/config.toml`

- **id** - The Discord application ID to run through. 
- **hosts** - An array of MPD server host socket addresses. 
    Each one will be tried in order until a playing server is found.
- **format** - Format strings. Tokens are listed below.
    - **details** - A format string for the top line. 
        This is the song title by default.
    - **state** - A format string for the second line. 
        This is the artist / album by default.
    - **timestamp** - The timestamp mode for the third line. 
        This is 'elapsed' by default.
        Can be one of `both`, `elapsed`, `left` or `off`. Falls back to `both`.
    - **large_image** - The name of the rich presence asset that gets displayed as the large image. This is `"notes"` by default. Setting this to `""` disables the large image.
    - **small_image** - The name of the rich presence asset that gets displayed as the small image. This is `"notes"` by default. Setting this to `""` disables the small image.
    - **large_text** - A format string that is displayed upon hovering the large image. Setting this to `""` disables the hover.
    - **small_text** - A format string that is displayed upon hovering the small image. Setting this to `""` disables the hover.

### Formatting Tokens

Any part of the format string that does not match one of these tokens will be displayed as is.
The following will automatically be replaced with their value from MPD:

- `$title`
- `$album`
- `$artist`
- `$albumartist`
- `$date`
- `$track`
- `$disc`
- `$genre`
- `$duration`
- `$elapsed`

### Default Configuration

This configuration file is automatically generated if one does not exist. 
It is included here for reference.

```toml
id = 677226551607033903
hosts = ["localhost:6600"]

[format]
details = "$title"
state = "$artist / $album"
timestamp = "both"
large_image = "notes"
small_image = "notes"
large_text = ""
small_text = ""
```

## Album art

Album art is pulled from the MusicBrainz database and Album Art Archive automatically. 
You'll only get a cover if it can be found though; there's a couple of things you can do to help this:

- Make sure your music is sensibly tagged. 
    In most cases MusicBrainz will be searched for releases matching the album/artist name.
- Add MusicBrainz release tags to your tracks. 
    This is officially supported by MPD and can be done automatically using MusicBrainz Picard.
- Add missing album art to MusicBrainz. 
    Many albums are missing covers, and you can upload your own to the database to contribute these for everyone.

