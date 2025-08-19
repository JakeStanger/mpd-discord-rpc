{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
}: let
  fs = lib.fileset;
in
  rustPlatform.buildRustPackage (finalAttrs: {
    pname = "mpd-discord-rpc";
    version = (builtins.fromTOML (builtins.readFile ../Cargo.toml)).package.version;

    src = fs.toSource {
      root = ../.;
      fileset = fs.unions [
        (fs.fileFilter (file: builtins.any file.hasExt ["rs"]) ../src)
        ../Cargo.lock
        ../Cargo.toml
      ];
    };

    nativeBuildInputs = [pkg-config];
    buildInputs = [openssl];

    cargoLock.lockFile = "${finalAttrs.src}/Cargo.lock";
    enableParallelBuilding = true;

    meta = {
      description = "Displays your currently playing song / album / artist from MPD in Discord";
      homepage = "https://github.com/JakeStanger/mpd-discord-rpc";
      license = lib.licenses.mit;
      platforms = lib.platforms.linux;
      maintainers = with lib.maintainers; [
        jakestanger
        fazzi
      ];
      mainProgram = "mpd-discord-rpc";
    };
  })
