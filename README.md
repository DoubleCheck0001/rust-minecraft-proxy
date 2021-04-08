# rust-minecraft-proxy
<a href="https://doublecheck.gg/discord">
    <img alt="Discord" src="https://img.shields.io/discord/752389778636406934?color=7289da&label=Discord">
</a>

Updated fork of [kuro46/minecraft-proxy-rs](https://github.com/kuro46/minecraft-proxy-rs) with the goal of adding additional features

## Goals
- ~~Unknown host MOTD~~ ✔️
- ~~Forge Support~~ ✔️
- Custom online & offline MOTD overrides for defined hosts
- Load balancing between defined hosts
- Proxy Protocol support (both incoming and outgoing)
- TCPShield compatibility & passthrough
- IP based filtering
- Optional Redis support
- Basic anti-bot system

## Disclaimer
This is a relatively new fork and is not yet ready for production. You can follow the instructions below to get an instance running, but there is a high chance of the structure of the project and configuration changing. Once the project has reached a point where I feel it is ready for a release, such release will be created on this repository. This project also serves as a way for me to learn Rust, so I am by no means a Rust developer yet. If there's anything that can be done better, feel free to fork and submit a pull request as well as add me on discord (DoubleCheck#0001) for code suggestions. 

## Usage

1. Download binary from releases or build with `cargo build --release` and run.
1. `config.yml` will be created. Please edit if you need.

### Configuration example

```yaml
listen_addr: "0.0.0.0:25565"
unknown_host:
  kick_message: "§bRust Minecraft Proxy\n\n§cInvalid Address"
  motd:
    text: "§cUnknown host!\n§7Please use a valid address to connect."
    protocol_name: §crust-minecraft-proxy
host:
  hub.example.com:
    ip: "127.0.0.1:35560"
  minigame.example.com:
    ip: "127.0.0.1:25561"
```
