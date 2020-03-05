# MPD Discord RPC

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

The AUR package can be found [here](https://aur.archlinux.org/packages/mpd-discord-rpc).

```
yay -S mpd-discord-rpc
```

### NixOS

I have a derivation on the way. The `[replace]` tag is causing some issues right now.

## Configuration

Running the program once will generate a default configuration file. On Linux this will be at `~/.config/discord-rpc/config.toml`

- **id** - The Discord application ID to run through. 
- **hosts** - An array of MPD server host socket addresses. Each one will be tried in order until a playing server isfound.
