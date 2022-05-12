extern crate ping;

use std::time::{Duration,Instant};

fn main() {
    let addr = "8.8.8.8".parse().unwrap();
    let timeout = Duration::from_secs(1);
    let now = Instant::now();
    ping::ping(addr, Some(timeout), Some(166), Some(3), Some(5), None).unwrap();
    let new_now = Instant::now();
    println!("{:?}", new_now.duration_since(now));
}
