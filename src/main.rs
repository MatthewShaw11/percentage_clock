use std::{
    io::{stdout, Write}, thread::sleep, time::Duration
};

mod percentage_clock;
mod command_arguments;

fn main() {
    
    let args = command_arguments::CommandArgs::new();
    
    if args.contains("--run") {
        let review = args.get("--run").unwrap();
        match review {
            Some(run_option) => {
                if run_option == "oneline" {
                    clock(true);
                } else {
                    println!("Unknown option for run flag of \"{run_option}\"");
                }
            },
            None => { 
                clock(false); 
            }
        }
        
    }
    else if args.count() > 1
    {
        println!("Received unknown command");
        println!("Known commands are \"--run\" to have a continiously running clock");
        println!("  to have only a single line update in place you can specify \"--run oneline\"");
    }
    else {
        let time = percentage_clock::get_time();
        println!("{time}");  
    }
}


fn clock(one_line: bool) {
    let mut last_time = String::new();
    loop {
        let time = percentage_clock::get_time();
        if last_time == time {
            sleep(Duration::new(0,100_000_000)); //100 milliseconds
            continue;
        }

        if one_line
        {
            print!("\r{time}");
        }
        else {
            println!("{time}");
        }
        stdout().flush().unwrap();
        last_time = time;  
    }
}
