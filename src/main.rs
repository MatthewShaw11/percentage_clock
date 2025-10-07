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

    
    let flag_full_help = "--help";
    if args.contains(flag_full_help) {
        println!("Percentage Clock");
        println!("Outputs the time of day using your system time.");
        println!("The time will be formatted [PercentOfDay]:[secondsUntilOnePercentGoesBy]");
        println!("There are 86,400 seconds in a day so there are 864 seconds in one percent.");
        println!("");
        println!("Flags");
        println!("  --run [option]");
        println!("    Causes the program to output continiously until the you press Ctrl+C");
        println!("    Options:");
        println!("      oneline (updates the text in-place rather than outputting new text each second");
        println!("  --figlet [figlet_font_here]");
        println!("    outputs the time using the provided font using the figlet command.");
        return;
    }
    
    
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
        println!("{}", pretty_print_time(&percentage_clock::get_time(), &figlet_font_option));
    }
    else {
        clock(&run_clock_in_one_line, &figlet_font_option);
    }
}


fn clock(one_line: &bool, figlet_font_option: &Option<String>) {
    let mut last_time = String::new();
    let mut sleep_time: u128;
    let mut start = Instant::now();
    
    let start_time = percentage_clock::get_time();
    clock_print_time(
        one_line, 
        figlet_font_option, 
        &pretty_print_time(&start_time, figlet_font_option), 
        &true
    );

    loop {
        let sync_time = percentage_clock::get_time();
        if sync_time != start_time {
            break;
        }
    }

    loop {
        let time = percentage_clock::get_time();
        let elapsed: u128 = start.elapsed().as_millis();

        if last_time == time {  
            //need to sleep, we got a result of a duplicate time
            let time_left: i32 = 1_000 - elapsed as i32;

            if time_left <= 0 {
                //more than a second has passed since loop began, restart to pull time again
                continue;
            }

            sleep_time = (0.9 * time_left as f64) as u128;

            if sleep_time < 50 {
                sleep_time = 10;
            }

            let sleep_nano: u128 = sleep_time * 1_000_000;
            let sleep_nano: u32 = sleep_nano
                                    .try_into()
                                    .unwrap_or(10_000_000); //10 milliseconds

            println!("sleep {sleep_nano} nano seconds");

            sleep(Duration::new(0,sleep_nano)); 
            
            continue;
        
            //sleep(Duration::new(0,25_000_000)); //25 milliseconds 
        } 
        
        //print the time
        clock_print_time(
            one_line, 
            figlet_font_option, 
            &pretty_print_time(&time, figlet_font_option), 
            &false
        );
        start = Instant::now();
        last_time = time;

    }
}

fn clock_print_time(
    one_line: &bool, 
    figlet_font_option: &Option<String>, 
    string_to_print: &String, 
    clear_prior_console_text: &bool) 
{
    match figlet_font_option {
        None => {
            match one_line {
                true => {
                    print!("\r{string_to_print}");
                },
                false => {
                    println!("{string_to_print}");
                }
            }
            stdout().flush().unwrap();
        }
        Some(_) => {
            if one_line == &true {
                if clear_prior_console_text == &false {
                    clear_multiline_output(&string_to_print);
                }
            }
            println!("{string_to_print}");
            stdout().flush().unwrap();
        }
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


fn pretty_print_time(time_input: &String, figlet_font_option: &Option<String>) -> String
{
    let time_string: String = time_input.to_string();
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