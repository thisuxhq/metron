use sysinfo::{DiskExt, NetworkExt, ProcessorExt, System, SystemExt};
use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let webhook_url = ""; // Replace with your Discord webhook URL
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
        let total_disk = system.disks().iter().map(|d| d.total_space()).sum::<u64>() as f64 / 1024.0 / 1024.0;
        let used_disk = total_disk - system.disks().iter().map(|d| d.available_space()).sum::<u64>() as f64 / 1024.0 / 1024.0;
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

        // Construct the message with emojis and better formatting
   let message = format!(
    "**Server Metrics Summary**\n\
    \n\
    :bar_chart: **CPU Usage**\n\
    {:.1}%\n\
    \n\
    :computer: **RAM Usage**\n\
    {:.1}% (Used: {:.2} MB / Total: {:.2} GB)\n\
    \n\
    :floppy_disk: **Disk Usage**\n\
    {:.1}% (Used: {:.2} MB / Total: {:.2} GB)\n\
    \n\
    :signal_strength: **Network Traffic**\n\
    Sent: {:.2} MB | Received: {:.2} MB\n\
    \n\
    :gear: **Total Processes**\n\
    {}\n\
    \n\
    :chart_with_upwards_trend: **Server Load**\n\
    {:.2}, {:.2}, {:.2}\n\
    \n\
    :loudspeaker: **System Messages**\n\
    {}",
    cpu_usage,                    // CPU Usage
    memory_usage,                 // RAM Usage Percentage
    used_memory,                  // Used RAM in MB
    total_memory / 1024.0,        // Total RAM in GB
    disk_usage_percentage,        // Disk Usage Percentage
    used_disk,                    // Used Disk in MB
    total_disk / 1024.0,          // Total Disk in GB
    total_transmitted,            // Network Traffic Sent
    total_received,               // Network Traffic Received
    total_processes,              // Total Processes
    load_average.one,             // Server Load 1 minute average
    load_average.five,            // Server Load 5 minute average
    load_average.fifteen,         // Server Load 15 minute average
    "No important system messages" // System Messages
);



        // Send the message to Discord
        match client.post(webhook_url)
            .json(&json!({ "content": message }))
            .send()
            .await {
                Ok(_) => println!("Message sent successfully"),
                Err(e) => eprintln!("Failed to send message to Discord: {}", e),
            }

        // Wait for 30 seconds
        sleep(Duration::from_secs(30)).await;
    }
}
