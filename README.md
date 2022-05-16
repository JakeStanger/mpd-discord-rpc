# MPD Discord RPC

![crates.io](https://img.shields.io/crates/v/mpd-discord-rpc)
![aur](https://img.shields.io/aur/version/mpd-discord-rpc-git)

Displays your currently playing song / album / artist from MPD in Discord using Rich Presence. It includes support for multiple MPD hosts if, like me, you have more than one server you alternate between.

The program does not require MPD or Discord to be running in order to run.

Once installed just run `mpd-discord-rpc`.

![status image](https://user-images.githubusercontent.com/5057870/168690393-ca28f55b-0e1a-4c30-ab09-e38754178aa7.png)


## Installation

### Cargo

The cargo package can be found [here](https://crates.io/crates/mpd-discord-rpc).

```
cargo install mpd-discord-rpc
```

### Arch Linux

The AUR package can be found [here](https://aur.archlinux.org/packages/mpd-discord-rpc-git).

```
yay -S mpd-discord-rpc-git
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
        Can be one of `elapsed`, `left` or `off`. Falls back to `elapsed`.
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
timestamp = "elapsed"
large_image = "notes"
small_image = "notes"
large_text = ""
small_text = ""
```
