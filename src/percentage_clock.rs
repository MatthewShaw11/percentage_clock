use chrono::{DateTime, Local, Timelike};


fn time_string_to_tuple(range_start_option: &String) -> Result<(u32,u32), String>
{
    let range_start = range_start_option
        .trim()
        .to_string();
    
    let time_segments: Vec<&str> = range_start.split(':').collect(); 
    
    if time_segments.len() != 2 {
        return Result::Err("the provided time string is not in the format number:number, value = \"{range_start}\"".to_string());
    }
    
    let hour_str: &str = time_segments.first().unwrap().trim();
    let hour: u32 = hour_str.parse().map_err(|e| format!(
        "the first segment of the provided time string could not be parsed into u32. value = \"{hour_str}\"\n{e}"))?;
    
    let minute_str: &str = time_segments.last().unwrap().trim();
    let minute: u32 = minute_str.parse().map_err(|e| format!(
        "the second segment of the provided time string could not be parsed into u32. value = \"{minute_str}\"\n{e}"))?;
    
    Ok((hour,minute))
}

fn seconds_since_midnight_from_time_string(time_string: &Option<String>) -> Result<u32, String>
{
    if time_string == &Option::None {
        return Ok(seconds_from_midnight());
    }

    let range_start = time_string
        .as_ref()
        .unwrap()
        .trim()
        .to_string();

    let range_start_hour_minute: (u32, u32) = time_string_to_tuple(&range_start)
        .map_err( |e|format!("Invalid time string provided, expected number:number, got \"{range_start}\" \n{}", e))?;

    Ok(range_start_hour_minute.0 * 3600 + range_start_hour_minute.1 * 60)
}

fn seconds_from_midnight() -> u32
{
    let dt1: DateTime<Local> = Local::now();
    dt1.num_seconds_from_midnight()
}

fn get_percentage_of_day() -> f64 
{
    let sec: f64 = seconds_from_midnight().into();
    let percentage_of_day: f64 = sec / 86_400_f64;
    percentage_of_day  
}

fn get_percent_and_percenties() -> (u32, u32)
{
    let perc = get_percentage_of_day() * 100_f64;

    let percentage: u32 = perc.trunc() as u32;
    let percenties: u32 = seconds_from_midnight() % 864 as u32;

    (percentage, percenties)
}

fn get_percent_and_percenties_with_range(
    input_range_start_option: &Option<String>,
    input_range_end_option: &Option<String>
) -> Result<(i32, i32), String>
{
    let mut range_start_option: &Option<String> = &input_range_start_option.clone();
    let mut range_end_option: &Option<String> = &input_range_end_option.clone();

    if range_start_option == &Option::None && range_end_option == &Option::None {
        let default_clock = get_percent_and_percenties();
        return Ok((default_clock.0 as i32, default_clock.1 as i32));
    }
    else if range_start_option == &Option::None || range_end_option == &Option::None {
        //make the option which is None to have the same value, were going to offset midnight point of the clock
        if range_start_option == &Option::None {
            range_start_option = range_end_option;
        }
        else {
            range_end_option = range_start_option;
        }
    }

    let range_start_seconds_from_midnight: u32 = seconds_since_midnight_from_time_string(range_start_option)
        .map_err(|e| format!("Failed to parse range start time string \n{}", e))?;
    
    let range_end_seconds_from_midnight: u32  = seconds_since_midnight_from_time_string(range_end_option)
        .map_err(|e| format!("Failed to parse range end time string \n{}", e))?;

    let range_one_percentie_in_seconds: f64 =  one_percentie_in_seconds(range_start_seconds_from_midnight, range_end_seconds_from_midnight);
    

    if range_start_seconds_from_midnight > range_end_seconds_from_midnight 
    {  //have to calculate this across midnight 
        let range_seconds_before_midnight: u32 = 864_00 - range_start_seconds_from_midnight;
        let time_seconds_from_midnight: f64 = seconds_from_midnight() as f64;
        let time_seconds_from_range_start: f64 = time_seconds_from_midnight + range_seconds_before_midnight as f64; 

        let time_percenties: i32 = (time_seconds_from_range_start / range_one_percentie_in_seconds).trunc() as i32;
        let time_seconds: i32 = (time_seconds_from_range_start % range_one_percentie_in_seconds).trunc() as i32;

        return Ok((time_percenties, time_seconds));
    }
    else 
    {  //recalculate range where 0 is some time after midnight yesterday and 100 is sometime before todays midnight 
        let time_seconds_from_midnight: f64 = seconds_from_midnight() as f64 - range_start_seconds_from_midnight as f64;
        let time_percenties: i32 = (time_seconds_from_midnight / range_one_percentie_in_seconds).trunc() as i32;
        let time_seconds: i32 = (time_seconds_from_midnight % range_one_percentie_in_seconds).trunc() as i32;
        
        return Ok((time_percenties, time_seconds));
    }
}


pub fn one_percentie_in_seconds(
    range_start_seconds_from_midnight: u32,
    range_end_seconds_from_midnight: u32 
) -> f64
{
    if range_start_seconds_from_midnight == range_end_seconds_from_midnight {
        return 864_f64;
    }

    let range_total_seconds: u32; 
    if range_end_seconds_from_midnight > range_start_seconds_from_midnight {
        range_total_seconds = range_end_seconds_from_midnight - range_start_seconds_from_midnight;
    }
    else { //this time range cross over the midnight barrior (86400 = seconds in 24 hours)
        let seconds_before_midnight: u32 = (86_400) - range_start_seconds_from_midnight;
        range_total_seconds = seconds_before_midnight + range_end_seconds_from_midnight;
    }
    let range_percentie_in_seconds: f64 = range_total_seconds as f64 / 100_f64;

    range_percentie_in_seconds
}

pub fn one_percentie_in_seconds_from_range_strings(
    range_start_option: &Option<String>,
    range_end_option: &Option<String>
) -> Result<f64, String>
{
    let range_start_seconds_from_midnight: u32 = seconds_since_midnight_from_time_string(range_start_option)
        .map_err(|e| format!("Failed to parse range start time string to calculate length of a percentie \n{}", e))?;
    
    let range_end_seconds_from_midnight: u32  = seconds_since_midnight_from_time_string(range_end_option)
        .map_err(|e| format!("Failed to parse range end time string to calculate length of a percentie \n{}", e))?;

    Ok(one_percentie_in_seconds(range_start_seconds_from_midnight, range_end_seconds_from_midnight))
}

pub fn get_time() -> String {
    let p = get_percent_and_percenties();
    format!("{:0>2}:{:0>3}", p.0, p.1)
}


pub fn get_time_with_range(
    range_start_option: &Option<String>,
    range_end_option: &Option<String>
) -> Result<String, String> {
    let p = get_percent_and_percenties_with_range (
            range_start_option,
            range_end_option
        ).map_err(|e|{ format!("Unable to produce the time based on the provided range!\n {e}") })?;
        
    Ok(format!("{:0>2}:{:0>3}", p.0, p.1))
}

