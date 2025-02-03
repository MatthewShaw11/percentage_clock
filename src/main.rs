use std::{thread::sleep, time::Duration};

mod percentage_clock;

fn main() {
    
    let mut last_time = String::new();
    loop {
        let time = percentage_clock::get_time();
        if last_time == time {
            sleep(Duration::new(0,250_000_000));
            continue;
        }

        println!("{time}");  
        last_time = time;  
    
    }
    
}

