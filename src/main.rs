use std::{
    io::{stdout, Write}, thread::sleep, time::Duration, process::Command, io
};

mod percentage_clock;
mod command_arguments;

fn main() {
    
    let args = command_arguments::CommandArgs::new();

    let mut run_clock = false;
    let mut run_clock_in_one_line = false;
    let mut figlet_font_option: Option<String> = None;

    let flag_full_run = "--run";
    if args.contains(flag_full_run) {
        run_clock = true;
        let value = args.get(flag_full_run).unwrap();
        match value {
            Some(run_option) => {
                match run_option.as_str() {
                    "oneline" => { 
                        run_clock_in_one_line = true;
                    },
                    _ => { println!("Unknown option for run flag of \"{run_option}\""); }
                }
            },
            None => {}
        }
    }

    let flag_full_figlet_font = "--figlet";
    if args.contains(flag_full_figlet_font) {
        let value = args.get(flag_full_figlet_font).unwrap();
        figlet_font_option = value;
    }


    if run_clock == false {
        println!("{}", get_time_string(&figlet_font_option));
    }
    else {
        clock(&run_clock_in_one_line, &figlet_font_option);
    }
}


fn clock(one_line: &bool, figlet_font_option: &Option<String>) {
    let mut last_time = String::new();
    let mut first_multi_print = true;
    loop {
        let time = get_time_string(figlet_font_option);
        if last_time == time {
            sleep(Duration::new(0,100_000_000)); //100 milliseconds
            continue;
        }

        match figlet_font_option {
            None => {
                match one_line {
                    true => {
                        print!("\r{time}");
                    },
                    false => {
                        println!("{time}");
                    }
                }
                stdout().flush().unwrap();
            }
            Some(_) => {
                match one_line {
                    true => {
                        if first_multi_print 
                        {
                            first_multi_print = false;
                        }
                        else {
                            clear_multiline_output(&time);    
                        }
                        print!("{time}");
                    },
                    false => {
                        println!("{time}");
                    }
                }
                stdout().flush().unwrap();
            }
        }

        last_time = time;  
    }
}


fn clear_multiline_output(output: &str) {
    let lines = output.matches('\n').count(); // Count number of newlines
    if lines > 0 {
        print!("\x1B[{}A", lines); // Move up X lines
    }
    print!("\x1B[J"); // Clear everything below
    io::stdout().flush().unwrap();
}


fn get_time_string(figlet_font_option: &Option<String>) -> String
{
    let time_string = percentage_clock::get_time();
    match figlet_font_option {
        None => {
            time_string
        }
        Some(font) => {
            let output = Command::new("figlet")
                .arg("-f")
                .arg(font)
                .arg(time_string)
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                
                //add three spaces to the end of each newline
                //so if we update the string in-place there wont be missing characters
                let output = stdout
                    .to_string()
                    .replace('\n', "   \n");
                output
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                panic!("Command failed:\n{}", stderr);
            }
        }
    }
}