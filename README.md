# minecraft-proxy-rs

minecraft-proxy-rs is a lightweight proxy server for Minecraft.

# Usage

1. Download binary from releases or build with `cargo build --release` and run.
1. `config.toml` will be created. Please edit if you need.

## Configuration example

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
