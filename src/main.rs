use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use sysinfo::{DiskExt, NetworkExt, ProcessorExt, System, SystemExt};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let webhook_url = "YOUR_DISCORD_WEBHOOK_URL"; // Replace with your Discord webhook URL
    let client = Client::new();

    loop {
        // Initialize the system struct
        let mut system = System::new_all();
        system.refresh_all();

        // Gather metrics
        let cpu_usage = system.global_processor_info().cpu_usage();
        let total_memory = system.total_memory() as f64 / 1024.0;
        let used_memory = system.used_memory() as f64 / 1024.0;
        let memory_usage = if total_memory > 0.0 {
            used_memory / total_memory * 100.0
        } else {
            0.0
        };
        let total_disk =
            system.disks().iter().map(|d| d.total_space()).sum::<u64>() as f64 / 1024.0 / 1024.0;
        let used_disk = total_disk
            - system
                .disks()
                .iter()
                .map(|d| d.available_space())
                .sum::<u64>() as f64
                / 1024.0
                / 1024.0;
        let disk_usage_percentage = if total_disk > 0.0 {
            (used_disk / total_disk) * 100.0
        } else {
            0.0
        };

        let mut total_received = 0;
        let mut total_transmitted = 0;
        for (_, data) in system.networks() {
            total_received += data.received();
            total_transmitted += data.transmitted();
        }
        let total_received = total_received as f64 / 1024.0 / 1024.0;
        let total_transmitted = total_transmitted as f64 / 1024.0 / 1024.0;

        let total_processes = system.processes().len();
        let load_average = system.load_average();
        let total_swap = system.total_swap() as f64 / 1024.0;
        let used_swap = system.used_swap() as f64 / 1024.0;
        let swap_usage = if total_swap > 0.0 {
            used_swap / total_swap * 100.0
        } else {
            0.0
        };

        let system_uptime = format_duration(Duration::from_secs(system.uptime()));

        // high level details of the system
        let embed = json!({
            "title": "Server Metrics Summary",
            "color": 3447003,
            "fields": [
                    { "name": "📊 CPU Usage", "value": format!("{:.1}%", cpu_usage), "inline": true },
                    { "name": "🧠 RAM Usage", "value": format!("{:.1}% (Used: {:.2} MB / Total: {:.2} GB)", memory_usage, used_memory, total_memory / 1024.0), "inline": true },
                    { "name": "💾 Disk Usage", "value": format!("{:.1}% (Used: {:.2} MB / Total: {:.2} GB)", disk_usage_percentage, used_disk, total_disk / 1024.0), "inline": true },
                    { "name": "🌐 Network Traffic", "value": format!("Sent: {:.2} MB | Received: {:.2} MB", total_transmitted, total_received), "inline": true },
                    { "name": "📈 Total Processes", "value": total_processes.to_string(), "inline": true },
                    { "name": "⚙️ Server Load", "value": format!("{:.2}, {:.2}, {:.2}", load_average.one, load_average.five, load_average.fifteen), "inline": true },
                    { "name": "🌀 Swap Usage", "value": format!("{:.1}% (Used: {:.2} MB / Total: {:.2} MB)", swap_usage, used_swap, total_swap), "inline": true },
                    { "name": "📝 System Uptime", "value": format!("{:?}", system_uptime), "inline": true },
                    { "name": "📢 System Messages", "value": "No important system messages", "inline": true }
            ]
        });

        // Send the embed message to Discord
        match client
            .post(webhook_url)
            .json(&json!({ "embeds": [embed] }))
            .send()
            .await
        {
            Ok(_) => println!("Embed message sent successfully"),
            Err(e) => eprintln!("Failed to send embed message to Discord: {}", e),
        }

        // Wait for 30 minutes
        sleep(Duration::from_secs(1800)).await;
    }
}

fn format_duration(duration: std::time::Duration) -> String {
    let mut seconds = duration.as_secs();
    let days = seconds / 86400;
    seconds %= 86400;
    let hours = seconds / 3600;
    seconds %= 3600;
    let minutes = seconds / 60;
    seconds %= 60;

    format!(
        "{} days, {} hours, {} minutes, {} seconds",
        days, hours, minutes, seconds
    )
}
