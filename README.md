# MPD Discord RPC

Displays your currently playing song / album / artist from MPD in Discord using Rich Presence.

Includes support for multiple MPD hosts if, like me, you have more than one server you alternate between.
These can be configured from `.config/discord-rpc/config.toml`. 
The appliation ID can also be edited from here.

Should smoothly handle situations where a connection to MPD or Discord cannot be established, and
will retry until connections are made.

If nothing is playing, nothing is displayed.

Install with `cargo install mpd-discord-rpc`.