extern crate chrono;
extern crate ping;
extern crate sqlite;

use std::thread;
use std::time::{Duration,Instant};
use chrono::prelude::*;

fn main() {
    let connection = sqlite::open(":memory:").unwrap();

    connection.execute(
        "
        CREATE TABLE pings (ip TEXT, timestamp TEXT, ping REAL);
        ",
    ).unwrap();

    let addrstr = "8.8.8.8";
    let addr = addrstr.parse().unwrap();
    let timeout = Duration::from_secs(1);

    for i in 1..10 {
        let now = Instant::now();
        ping::ping(addr, Some(timeout), Some(166), Some(3), Some(i), None).unwrap();
        let new_now = Instant::now();
        let local: DateTime<Local> = Local::now();

        connection.execute(
            format!("INSERT INTO pings VALUES ('{}', '{}', {});", addrstr, local, new_now.duration_since(now).as_micros()),
        ).unwrap();
        println!("{} {} {}", addrstr, local, new_now.duration_since(now).as_micros());
        thread::sleep(Duration::from_secs(1));
    }
    
}
