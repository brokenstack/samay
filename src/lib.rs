use std::error::Error;

use chrono::{DateTime, Local};
use progress::*;

pub mod progress;

#[derive(Debug)]
pub struct Args {
    pub flag: Option<String>,
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        match args.len() {
            1 => return Ok(Args { flag: None }),
            2 => {
                //TODO:  match valid flags here
                let flag = args[1].clone();

                Ok(Args { flag: Some(flag) })
            }
            _ => return Err("invalid arguments..."),
        }
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let flag = args.flag;
    let time = chrono::offset::Local::now();

    match flag {
        Some(_) => todo!(),
        None => {
            print_all(time)?;
        }
    }
    Ok(())
}

pub fn print_all(time: DateTime<Local>) -> Result<(), Box<dyn Error>> {
    let day_progress = calc_day(&time);
    println!("Day Progress: {:.2}%", day_progress);
    println!(
        "Week Progress: {:.2}%",
        calc_week(&time, FirstDay::Sunday, true)
    );
    println!("Month Progress: {:.2}%", calc_month(&time));
    println!("Year Progress: {:.0}%", calc_year(&time));
    Ok(())
}
