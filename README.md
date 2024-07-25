# Metron

**Metron** is a Rust-based server monitoring tool that sends server metrics to a Discord channel via a webhook. It provides insights into CPU usage, memory usage, disk usage, network traffic, and system load.

## Features

- 📊 **CPU Usage**: Monitor CPU usage.
- 🧠 **Memory Usage**: Track RAM usage and available memory.
- 💾 **Disk Usage**: Check disk space used and available.
- 🌐 **Network Traffic**: Measure data sent and received.
- 📈 **System Load**: Report server load averages.

## Prerequisites

- A Hetzner server or any Linux-based server.
- Rust installed on the server.
- `systemd` for service management.
- A Discord webhook URL for sending metrics.

## Installation

### 1. Install Rust

If Rust is not installed, use these commands:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Clone the Repository

Clone this repository to your server:

```sh
git clone https://github.com/THISUXHQ/metron.git
cd metron
```

### 3. Build the Project

Build the Rust project:

```sh
cargo build --release
```

### 4. Configure the Service

Create and set up the `metron.service` file:

```sh
sudo nano /etc/systemd/system/metron.service
```

Add this content, replacing placeholders with your actual paths and username:

```ini
[Unit]
Description=Server Metrics Monitor
After=network.target

[Service]
ExecStart=/home/yourusername/metron/target/release/metron
Restart=always
User=yourusername
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

### 5. Enable and Start the Service

Reload `systemd`, enable, and start the service:

```sh
sudo systemctl daemon-reload
sudo systemctl enable metron.service
sudo systemctl start metron.service
```

### 6. Verify the Service

Check if the service is running:

```sh
sudo systemctl status metron.service
```

See real-time logs:

```sh
journalctl -u metron.service -f
```

### 7. Stopping the Service

To stop the service, use:

```sh
sudo systemctl stop metron.service
```

### 8. Disabling the Service

To stop it from starting at boot:

```sh
sudo systemctl disable metron.service
```

## Configuration

Edit the `src/main.rs` file to set your Discord webhook URL:

```rust
let webhook_url = "YOUR_DISCORD_WEBHOOK_URL"; // Replace with your Discord webhook URL
```

## Contributing

Feel free to open issues and submit pull requests. We welcome contributions!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For questions or feedback, email [sanju](mailto:work@sanju.sh)
