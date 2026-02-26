use chrono::Local;
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

fn main() {
    println!("Starting globex monitor with LOAD (Interval: 5s)...");

    // --- CPU Load Thread ---
    thread::spawn(|| {
        println!("CPU load thread started...");
        let mut x: u64 = 0;
        loop {
            // Intensive calculation to keep the CPU busy
            x = x.wrapping_add(1);
            if x % 1_000_000_000 == 0 {
                // Prevent the compiler from optimizing the loop away entirely
                std::hint::black_box(x);
            }
        }
    });

    // --- Memory Load Thread ---
    thread::spawn(|| {
        println!("Memory load thread started...");
        // Allocate approx 500MB of data
        let size = 500 * 1024 * 1024; // 500MB
        let mut _data: Vec<u8> = vec![0u8; size];
        
        // Touch the memory to ensure it's actually committed
        for i in (0..size).step_by(4096) {
            _data[i] = 1;
        }

        loop {
            // Keep the memory allocated
            thread::sleep(Duration::from_secs(60));
            std::hint::black_box(&_data);
        }
    });

    // --- Reporter Loop ---
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    loop {
        sys.refresh_all();

        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        let cpu_usage = sys.global_cpu_usage();
        let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

        println!(
            "[{}] CPU: {:.2}% | Mem: {:.2}GB / {:.2}GB",
            timestamp, cpu_usage, used_mem, total_mem
        );

        thread::sleep(Duration::from_secs(5));
    }
}
