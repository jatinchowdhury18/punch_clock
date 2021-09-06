use std::io::prelude::*;
use std::fs;
use std::fs::OpenOptions;
use chrono::{Datelike,Timelike,Local};

const IN_PUNCH: &str = "[IN]";
const OUT_PUNCH: &str = "[OUT]";

// punch in or out
pub fn punch(tag: &str, time_file: &str, punch_in: bool) -> Result<(), &'static str> {
    let mut file = match OpenOptions::new()
        .read(true)
        .append(true)
        .open(&time_file) {
        Ok(args) => args,
        Err(_) => return Err("Unable to load tracker file!"),
    };

    let punch_type = if punch_in {IN_PUNCH} else {OUT_PUNCH};

    let last_punch = get_last_punch(tag, &mut file);
    if last_punch == punch_type {
        if punch_in {
            return Err("You are already punched in for this project!")
        } else {
            return Err("You have already punched out for this project!")
        }
    }

    let now = Local::now();
    let time_str = format!("{}-{}-{}-{}-{}", now.year(), now.month(), now.day(), now.hour(), now.minute());

    let line = format!("{}, {} {}\n", tag, time_str, punch_type);

    match file.write(line.as_bytes()) {
        Err(_) => return Err("Unable to print to file!"),
        _ => (),
    };

    Ok(())
}

// determine if previous punch for this project
// was a punch in or out
fn get_last_punch(tag: &str, file: &mut fs::File) -> String {
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or_else(|_err| {0});

    let mut last_state = String::new();
    for line in contents.lines().rev() {
        if line.contains(tag) {
            let line_split: Vec<&str> = line.split(' ').collect();
            last_state = match line_split.last() {
                Some(s) => s.to_string(),
                None => continue,
            };
            break;
        }
    }

    last_state
}
