use std::{
    io::{self, stdout, Write}, process::Command, thread::sleep, time::Duration
};
use std::time::Instant;

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
    let mut sleep_time: u128;
    let mut start = Instant::now();
    
    loop {
        let time = get_time_string(figlet_font_option);
        let elapsed: u128 = start.elapsed().as_millis();

        if last_time == time {

            let time_left = 1_000 - elapsed;
            sleep_time = (0.9 * time_left as f64) as u128;

            if sleep_time < 50 {
                sleep_time = 10;
            }

            //println!("Sleep time {sleep_time}");
            let ten_milli_as_nano = 10_000_000; //10 milliseconds
            let sleep_nano: u32 = (sleep_time * 1_000_000).try_into().unwrap_or(ten_milli_as_nano); 
            sleep(Duration::new(0,sleep_nano)); 
            
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
        //println!("elapsed: {elapsed}");
        start = Instant::now();
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
                .arg("-w")
                .arg("500")
                .arg("-f")
                .arg(font)
                .arg(time_string)
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                
                //add three spaces to the end of each newline
                //so if we update the string in-place there wont be missing characters
                let output = stdout.trim_end().to_string() + "\n";

                let has_new_line = output.contains('\n');

                if has_new_line {
                    let s: &str = output.as_str();

                    let index_of_first_not_whitespace = 
                        s.char_indices()
                        .find(|&(_i, c)| 
                        {
                            !c.is_whitespace()
                    }); 

                    if index_of_first_not_whitespace == None {
                        return output;
                    }

                    let index_of_first_not_whitespace = index_of_first_not_whitespace.unwrap().0;
                    let leading_whitespace: String = output[..index_of_first_not_whitespace].to_string();
                    let last_newline_in_leading_whitespace = leading_whitespace.rfind('\n');

                    if last_newline_in_leading_whitespace == None {
                        //no newlines in leading whitespace = no added lines
                        return output;
                    }

                    let index_of_start_of_first_real_line = last_newline_in_leading_whitespace.unwrap(); 
                    if index_of_start_of_first_real_line + 1 > output.chars().count() - 1 {
                        //if the index of the start of the first real line plus one
                        // is outside the bounds of the array 
                        return output;
                    }

                    //return output with a single newline character, then crop out all the leading whitespace
                    //not apart of the first valid line
                    let output= "\n".to_string() + &output[index_of_start_of_first_real_line+1..];

                    output
                }
                else {
                    //font is single line, just output
                    output
                }
                
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                panic!("Command failed:\n{}", stderr);
            }
        }
    }
}