use chrono::Local;
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

fn main() {
    println!("Starting globex monitor (Interval: 5s)...");

    // Initialize system monitor
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    loop {
        // Refresh system information
        sys.refresh_all();

        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        // Get CPU and Memory info
        let cpu_usage = sys.global_cpu_usage();
        let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0; // GB
        let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;   // GB

        println!(
            "[{}] CPU: {:.2}% | Mem: {:.2}GB / {:.2}GB",
            timestamp, cpu_usage, used_mem, total_mem
        );

        thread::sleep(Duration::from_secs(5));
    }
}
