# MPD Discord RPC

![crates.io](https://img.shields.io/crates/v/mpd-discord-rpc)
![aur](https://img.shields.io/aur/version/mpd-discord-rpc=git)

Displays your currently playing song / album / artist from MPD in Discord using Rich Presence. It includes support for multiple MPD hosts if, like me, you have more than one server you alternate between.

The program does not require MPD or Discord to be running in order to run.

Once installed just run `mpd-discord-rpc`.

![status](https://f.jstanger.dev/github/mpd-discord-rpc/status.png)

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

I have a derivation on the way. The `[replace]` tag is causing some issues right now.

## Configuration

Running the program once will generate a default configuration file. On Linux this will be at `~/.config/discord-rpc/config.toml`

- **id** - The Discord application ID to run through. 
- **hosts** - An array of MPD server host socket addresses. Each one will be tried in order until a playing server is found.
- **format** - Format strings. Tokens are listed below.
    - **details** - A format string for the top line. This is the song title by default.
    - **state** - A format string for the second line. This is the artist / album by default.

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
```
