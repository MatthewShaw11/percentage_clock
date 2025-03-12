use std::{
    env, io::{stdout, Write}, thread::sleep, time::Duration
};

mod percentage_clock;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    
    if args.iter().count() > 1 {
        let first_opt = args
            .get(1)
            .unwrap()
            .trim()
            .to_ascii_lowercase();

        if first_opt == "--run"
        {
            clock();
        }
        else {
            println!("The only valid option you can provide is \"--run\" to make this run as a continuously running clock synced with your computers time.");
        }
    }
    else {
        let time = percentage_clock::get_time();
        println!("{time}");  
    }
}


fn clock() {
    let mut last_time = String::new();
    loop {
        let time = percentage_clock::get_time();
        if last_time == time {
            sleep(Duration::new(0,100_000_000)); //100 milliseconds
            continue;
        }


        println!("{time}");  
        stdout().flush().unwrap();

        last_time = time;  
    }
}