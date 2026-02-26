use chrono::Local;
use local_ip_address::local_ip;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting globex monitor (Interval: 5s)...");

    loop {
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        match local_ip() {
            Ok(ip) => {
                println!("[{}] Current IP: {}", timestamp, ip);
            }
            Err(e) => {
                eprintln!("[{}] Error getting IP: {}", timestamp, e);
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}
