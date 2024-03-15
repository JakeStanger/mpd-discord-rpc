# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v1.7.2] - 2024-03-15
### :bug: Bug Fixes
- [`bba3015`](https://github.com/JakeStanger/mpd-discord-rpc/commit/bba3015092bcc2ad82cb18e325ab734f051d4b95) - not retrying discord connection *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`558c3e1`](https://github.com/JakeStanger/mpd-discord-rpc/commit/558c3e19b0964a8165b1360763ff5d73f44a8c1f) - crashes when duration/elapsed missing *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`6e7d1e9`](https://github.com/JakeStanger/mpd-discord-rpc/commit/6e7d1e92a1fb3b8cec33ea86f674130be3c3a532) - error when setting activity with strings over 128 bytes *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :recycle: Refactors
- [`3b46dcc`](https://github.com/JakeStanger/mpd-discord-rpc/commit/3b46dcc6bb454ecc0a8da44aec2bb4e86ac3c7bf) - fix clippy warnings *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`a31e88d`](https://github.com/JakeStanger/mpd-discord-rpc/commit/a31e88d6da8754bac93f30c28659252061dfbbc8) - update dependencies to latest versions *(commit by [@JakeStanger](https://github.com/JakeStanger))*


## [v1.7.1] - 2023-07-09
### :recycle: Refactors
- [`36ac27f`](https://github.com/JakeStanger/mpd-discord-rpc/commit/36ac27f7e4a4ad5f6961ab8f7932fcfa03991323) - use `discord_presence` crate *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :memo: Documentation Changes
- [`4bf1415`](https://github.com/JakeStanger/mpd-discord-rpc/commit/4bf1415a7648c21754e0bc2ab2c3125f983fbb37) - **readme**: add systemd unit info *(commit by [@JakeStanger](https://github.com/JakeStanger))*


## [v1.7.0] - 2023-06-14
### :sparkles: New Features
- [`23791be`](https://github.com/JakeStanger/mpd-discord-rpc/commit/23791be7ad5f33a92e15d3d142aef2e325ecf6a4) - support for album artist tags *(PR [#41](https://github.com/JakeStanger/mpd-discord-rpc/pull/41) by [@derinsh](https://github.com/derinsh))*
- [`55c397f`](https://github.com/JakeStanger/mpd-discord-rpc/commit/55c397f2638341b2732e170ffd70beda0968297c) - add systemd service *(commit by [@Serial-ATA](https://github.com/Serial-ATA))*


## [v1.6.0] - 2023-05-26
### :sparkles: New Features
- [`22803a1`](https://github.com/JakeStanger/mpd-discord-rpc/commit/22803a10b916d3b2e603e602939908fac71846fe) - **album art**: improve algorithm for finding covers *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :recycle: Refactors
- [`b88467f`](https://github.com/JakeStanger/mpd-discord-rpc/commit/b88467f48e193e10d0c8ca7ded84b7d112febf35) - replace thread sleeps with tokio sleeps *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`a0421a8`](https://github.com/JakeStanger/mpd-discord-rpc/commit/a0421a8d8dab16193c337d0d103d9b29290ef8a1) - replace config impl with `universal-config`, better serde standards *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`4c22c0e`](https://github.com/JakeStanger/mpd-discord-rpc/commit/4c22c0ebc1c6382832566efb9c58498fe086ba75) - fix various clippy warnigns *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`0066565`](https://github.com/JakeStanger/mpd-discord-rpc/commit/006656572f679dab18691db4b014d626f3ab5029) - replace mpd code with `mpd-utils` crate *(commit by [@JakeStanger](https://github.com/JakeStanger))*


[v1.6.0]: https://github.com/JakeStanger/mpd-discord-rpc/compare/v1.5.4b...v1.6.0
[v1.7.0]: https://github.com/JakeStanger/mpd-discord-rpc/compare/v1.6.0...v1.7.0
[v1.7.1]: https://github.com/JakeStanger/mpd-discord-rpc/compare/v1.7.0...v1.7.1
[v1.7.2]: https://github.com/JakeStanger/mpd-discord-rpc/compare/v1.7.1...v1.7.2