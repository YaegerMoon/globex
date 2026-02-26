use chrono::Local;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting globex monitor (Interval: 5s)...");

    loop {
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        println!("[{}] Current Time", timestamp);

        thread::sleep(Duration::from_secs(5));
    }
}
