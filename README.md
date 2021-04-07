# rust-minecraft-proxy

Updated fork of [kuro46/minecraft-proxy-rs](https://github.com/kuro46/minecraft-proxy-rs) with the goal of adding additional features

## Goals
- Unknown host MOTD
- Custom online & offline MOTD overrides for defined hosts
- Load balancing between defined hosts
- Proxy Protocol support (both incoming and outgoing)
- TCPShield compatibility & passthrough
- Forge Support
- IP based filtering
- Optional Redis support
- Basic anti-bot system

## Disclaimer
This is a relatively new fork and is not yet ready for production. You can follow the below instructions to get an instance running, but there is a high chance of the structure of the project and configuration changing. Once the project has reached a point where I feel it is ready for a release, such release will be created on this repository. 

## Usage

1. Download binary from releases or build with `cargo build --release` and run.
1. `config.toml` will be created. Please edit if you need.

### Configuration example

```toml
listen_addr = "0.0.0.0:25565"
unknown_host_message = "{\"text\":\"Invalid address.\",\"color\":\"red\"}"

[hosts]
hub.example.com = "127.0.0.1:35560"
minigame.example.com = "127.0.0.1:25561"
```

If connect with `hub.example.com`, it will connect to `127.0.0.1:35560`.  
If connect with `minigame.example.com`, it will connect to `127.0.0.1:35561`.  
If connect with other hosts, it will be disconnected and shows `Invalid address.` message.
