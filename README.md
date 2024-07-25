# Metron

**Metron** is a Rust-based server monitoring tool that sends server metrics to a Discord channel via a webhook. It provides insights into CPU usage, memory usage, disk usage, network traffic, and system load.

## Features

- **CPU Usage**: Monitor CPU utilization.
- **Memory Usage**: Track RAM usage and available memory.
- **Disk Usage**: Observe disk space used and available.
- **Network Traffic**: Measure sent and received data.
- **System Load**: Report server load averages.

## Prerequisites

- A Hetzner server or any Linux-based server.
- Rust installed on the server.
- `systemd` for service management.
- A Discord webhook URL for sending metrics.

## Installation

### 1. Install Rust

If Rust is not installed, you can install it using the following commands:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Clone the Repository

Clone this repository to your server:

```sh
git clone https://github.com/yourusername/metron.git
cd metron
```

### 3. Build the Project

Build the Rust project with the release profile:

```sh
cargo build --release
```

### 4. Configure the Service

Create and configure the `metron.service` file:

```sh
sudo nano /etc/systemd/system/metron.service
```

Add the following content to the service file, replacing placeholders with your actual paths and username:

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

Reload the `systemd` manager configuration, enable the service, and start it:

```sh
sudo systemctl daemon-reload
sudo systemctl enable metron.service
sudo systemctl start metron.service
```

### 6. Verify the Service

Check the status of the service to ensure it’s running correctly:

```sh
sudo systemctl status metron.service
```

View the logs for real-time updates:

```sh
journalctl -u metron.service -f
```

### 7. Stopping the Service

To stop the service, use the following command:

```sh
sudo systemctl stop metron.service
```

## Configuration

Edit the `src/main.rs` file to set your Discord webhook URL:

```rust
let webhook_url = "YOUR_DISCORD_WEBHOOK_URL"; // Replace with your Discord webhook URL
```

## Contributing

Feel free to open issues and submit pull requests. Contributions are welcome!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For any questions or feedback, you can reach out to [your email](mailto:hi@sanju.sh).
```
