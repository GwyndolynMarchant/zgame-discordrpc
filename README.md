## zgame-discordrpc
![license](https://img.shields.io/badge/license-public%20domain-green)

A discordrpc client for ZDoom and Raze written in Rust.

Possibly cross-platform support? It works on Windows. I haven't personally tested it otherwise.

I forked this because:

1. DiscordRPC for Raze and GZDoom is still WIP
2. I play a lot of WADs for friends in Discord.
3. This was a fun way to learn Rust.

## How it works
1. Program reads first argument
2. Program connects to Discord via RPC
3. It then looks for a zgame process based on supplied argument, and parses it's window title ("level - game/mod")
4. It is separated into an &str vector
5. The icon will be the logo for the game/mod, the status will be the game and level, and the hover text will be the engine
6. Program loops every 15 secs, looping every second would be overkill

### Supported Engines
- GZDoom
- LZDoom
- Raze

## Running
1. First make sure that a supported engine is running.
2. Download a prebuilt binary from the releases section.
3. Open a terminal (on Windows, CMD or PowerShell should work just fine) then...

ENGINE = `gzdoom` or `lzdoom` or `raze`

Linux: `chmod +x gzdoom-discordrpc`, then `./gzdoom-discordrpc ENGINE`

Windows: `.\gzdoom-discordrpc.exe ENGINE`

From source:
1. `git clone https://github.com/Phate6660/gzdoom-discordrpc`
2. `cd gzdoom-discordrpc`
3. `cargo run -- ENGINE`

## Screenshots

DOOM:

![DOOM](images/doom.png?raw=true "DOOM")

DOOM (with LZDoom):

![DOOM](images/doom_lz.png?raw=true "DOOM (with LZDoom)")

DOOM II:

![DOOM II](images/doom_ii.png?raw=true "DOOM II")

Project Brutality:

![Project Brutality](images/pb.png?raw=true "Project Brutality")