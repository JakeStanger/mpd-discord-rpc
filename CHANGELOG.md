# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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