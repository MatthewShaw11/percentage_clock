
use chrono::{DateTime, Local, Timelike};

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

fn get_percent_and_percenties() -> (u8, u32)
{
    let perc = get_percentage_of_day() * 100_f64;

    let percentage: u8 = perc.trunc() as u8;
    let percenties: u32 = (perc.fract() * 864_f64) as u32;

    (percentage, percenties)
}

pub fn get_time() -> String {
    let p = get_percent_and_percenties();
    format!("{}:{}", p.0, p.1)
}