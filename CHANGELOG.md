# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v1.11.0] - 2026-01-24
### :sparkles: New Features
- [`828b94a`](https://github.com/JakeStanger/mpd-discord-rpc/commit/828b94a2fa2b05c97f466f361451c27817042084) - configurable activity button for searching active song on another platform *(commit by [@ReallyWeirdCat](https://github.com/ReallyWeirdCat))*

### :recycle: Refactors
- [`7ec612d`](https://github.com/JakeStanger/mpd-discord-rpc/commit/7ec612dcd8561de6500f9b59d35ea6c1d74f3693) - remove button_enabled parameter and implement support for two buttons *(commit by [@ReallyWeirdCat](https://github.com/ReallyWeirdCat))*

### :memo: Documentation Changes
- [`b1920e8`](https://github.com/JakeStanger/mpd-discord-rpc/commit/b1920e81df223ff590cd98c42a4e80681f97ae36) - update readme to document button configuration *(commit by [@ReallyWeirdCat](https://github.com/ReallyWeirdCat))*


## [v1.10.0] - 2026-01-06
### :sparkles: New Features
- [`6de3816`](https://github.com/JakeStanger/mpd-discord-rpc/commit/6de38164f8b903f6ec17b19c4b607ce7f9f80a69) - add `originaldate` tag/formatting token *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :bug: Bug Fixes
- [`c0f6c6c`](https://github.com/JakeStanger/mpd-discord-rpc/commit/c0f6c6c629d8e86d2345126eb852dcd347fa2aca) - **systemd**: remove mpd requirement *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :recycle: Refactors
- [`bb0900b`](https://github.com/JakeStanger/mpd-discord-rpc/commit/bb0900bb0adf0bb7c1598ad66587c9b0004a5183) - fix new clippy warnings *(commit by [@JakeStanger](https://github.com/JakeStanger))*


## [v1.9.0] - 2025-08-21
### :sparkles: New Features
- [`c0cb562`](https://github.com/JakeStanger/mpd-discord-rpc/commit/c0cb5625c818c4ebc827ed4460bfadd019a5d343) - allow configuring status display *(commit by [@fxzzi](https://github.com/fxzzi))*
- [`7491c72`](https://github.com/JakeStanger/mpd-discord-rpc/commit/7491c72a7ce5e5eb689e3d4e267bae733e9bafd1) - nix packaging *(commit by [@fxzzi](https://github.com/fxzzi))*

### :bug: Bug Fixes
- [`a82faf0`](https://github.com/JakeStanger/mpd-discord-rpc/commit/a82faf0f12109536b78016ff3faace46b370929a) - create drpc client with error config *(commit by [@koffydrop](https://github.com/koffydrop))*
- [`e50d778`](https://github.com/JakeStanger/mpd-discord-rpc/commit/e50d778b59abb3be836db9ef153f611313c28b08) - use empty char for extending details *(commit by [@fxzzi](https://github.com/fxzzi))*


## [v1.8.1] - 2025-06-22
### :sparkles: New Features
- [`0697293`](https://github.com/JakeStanger/mpd-discord-rpc/commit/0697293b2315cc7cee08ec0fb0d8a1e264d59517) - timestamp "both" mode, progress bar *(commit by [@fxzzi](https://github.com/fxzzi))*

### :bug: Bug Fixes
- [`f395b30`](https://github.com/JakeStanger/mpd-discord-rpc/commit/f395b30f75a51a9d8ec131981f170e31f15f7313) - error if song title is one char long *(commit by [@JakeStanger](https://github.com/JakeStanger))*
- [`865621c`](https://github.com/JakeStanger/mpd-discord-rpc/commit/865621c3a221dca9a7db231a51811b3c413bbd65) - not sending ready event on connect *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :recycle: Refactors
- [`e759103`](https://github.com/JakeStanger/mpd-discord-rpc/commit/e7591037792cb6db220d46afa0076319ce3820f6) - reduce idle check time to 3 seconds *(commit by [@JakeStanger](https://github.com/JakeStanger))*


## [v1.8.0] - 2025-02-13
### :sparkles: New Features
- [`07eec2b`](https://github.com/JakeStanger/mpd-discord-rpc/commit/07eec2b699d0d52f6417ee88ed9c6a447258f345) - change status message from "playing" to "listening to" *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :bug: Bug Fixes
- [`59f7ddf`](https://github.com/JakeStanger/mpd-discord-rpc/commit/59f7ddf291ad54515e9f5199cf289f7489a3dbf1) - not reconnecting if discord restarted *(commit by [@JakeStanger](https://github.com/JakeStanger))*


## [v1.7.3] - 2024-06-13
### :recycle: Refactors
- [`9e153e4`](https://github.com/JakeStanger/mpd-discord-rpc/commit/9e153e4c2efb453c8ef22f2f94e1804fee88d6e5) - reduce calls to mpd server *(commit by [@JakeStanger](https://github.com/JakeStanger))*

### :memo: Documentation Changes
- [`0fd70aa`](https://github.com/JakeStanger/mpd-discord-rpc/commit/0fd70aa776906e44eba1e686d9f64d944e18efd0) - **readme**: update arch packages *(commit by [@JakeStanger](https://github.com/JakeStanger))*


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
[v1.7.3]: https://github.com/JakeStanger/mpd-discord-rpc/compare/v1.7.2...v1.7.3
[v1.8.0]: https://github.com/JakeStanger/mpd-discord-rpc/compare/v1.7.3...v1.8.0
[v1.8.1]: https://github.com/JakeStanger/mpd-discord-rpc/compare/v1.8.0...v1.8.1
[v1.9.0]: https://github.com/JakeStanger/mpd-discord-rpc/compare/v1.8.1...v1.9.0
[v1.10.0]: https://github.com/JakeStanger/mpd-discord-rpc/compare/v1.9.0...v1.10.0
[v1.11.0]: https://github.com/JakeStanger/mpd-discord-rpc/compare/v1.10.0...v1.11.0
