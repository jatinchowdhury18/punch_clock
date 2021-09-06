use super::project::Project;
use chrono::{Datelike,Local,DateTime,NaiveDateTime,NaiveDate,NaiveTime};
use std::io;
use std::fs;
use std::ops::Add;

// struct to store report entry
#[derive(Debug)]
struct Entry {
    pub date: DateTime<Local>,
    pub hours_worked: f32,
}

impl Entry {
    pub fn add_time(&mut self, new_time: f32) {
        self.hours_worked += new_time;
    }
}

// create a report for a project
pub fn create_report(proj : &Project, time_file: &str) {
    println!("Enter start date (\"{{month}}-{{date}}\", or \"{{year}}-{{month}}-{{date}}\"):");
    let start = get_time();

    println!("Enter end date (\"{{month}}-{{date}}\", or \"{{year}}-{{month}}-{{date}}\"):");
    let end = get_time();

    let contents = match fs::read_to_string(time_file) {
        Ok(s) => s,
        Err(_) => return,
    };

    let mut lines = contents.lines()
        .filter(|line| {
            let tag_com = String::from(proj.get_tag()).add(",");
            line.contains(&tag_com)
        })
        .rev();

    let mut entries: Vec<Entry> = vec![];
    while let Some((st, ed)) = get_next_in_out_pair(&mut lines, &start, &end) {
        let start_time = string_to_datetime(st);
        let end_time = string_to_datetime(ed);

        let dur = end_time - start_time;
        let hours_worked = dur.num_minutes() as f32 / 60.0;

        let mut found = false;
        for entry in entries.iter_mut() {
            if entry.date == start_time {
                found = true;
                entry.add_time(hours_worked);
            }
        }

        if ! found {
            entries.push(Entry { date: start_time, hours_worked: hours_worked });
        }
    }

    print_header(&proj);
    print_entry_list(&entries);
}

// print company name and address to report header
fn print_header(proj : &Project) {
    println!("{}", proj.get_name());
    println!("{}", proj.get_address().get_string());
}

// print list of entries to CLI
fn print_entry_list(list: &Vec<Entry>) {
    for entry in list {
        println!("\t{}: {} hours", entry.date.date(), entry.hours_worked);
    }
}

// Get the next pair of [IN]/[OUT] times in range
fn get_next_in_out_pair<'a, I>(lines: &mut I,
                               range_start: &DateTime<Local>,
                               range_end: &DateTime<Local>) -> Option<(&'a str, &'a str)>
where
    I: Iterator<Item = &'a str>
{
    let next_line = match lines.next() {
        Some(l) => l,
        None => return None,
    };

    let line_split: Vec<&str> = next_line.split(' ').collect();
    let end_str = line_split[1];

    let next_line = match lines.next() {
        Some(l) => l,
        None => return None,
    };

    let line_split: Vec<&str> = next_line.split(' ').collect();
    let start_str = line_split[1];
    let start_time = string_to_datetime(start_str);

    if start_time < *range_start {
        return None; // too early!
    }

    if start_time > *range_end {
        return get_next_in_out_pair(lines, range_start, range_end);
    }

    Some((start_str, end_str))
}

// get a datetime object from a punch string
fn string_to_datetime(dt_str: &str) -> DateTime<Local> {
    let mut str_split: Vec<&str> = dt_str.split('-').collect();

    let pop_val = |x: &mut Vec<&str>| -> u32 {
        match &mut x.pop() {
        Some(s) => {
            match s.trim().parse() {
                Ok(val) => val,
                Err(_) => 0,
            }
        }
        None => 0,
    }};

    let min = pop_val(&mut str_split);
    let hour = pop_val(&mut str_split);
    let day = pop_val(&mut str_split);
    let month = pop_val(&mut str_split);
    let year = pop_val(&mut str_split) as i32;

    let day = NaiveDate::from_ymd(year, month, day);
    let time = NaiveTime::from_hms(hour, min, 0);
    DateTime::<Local>::from_utc(NaiveDateTime::new(day, time), chrono::offset::FixedOffset::east(0))
}

// get a datetime from user input
fn get_time() -> DateTime<Local> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut in_split: Vec<&str> = input.split('-').collect();

    if in_split.len() < 2 || in_split.len() > 3 {
        return Local::now();
    }

    let date: u32 = match &mut in_split.pop() {
        Some(s) => {
            match s.trim().parse() {
                Ok(val) => val,
                Err(_) => return Local::now(),
            }
        }
        None => return Local::now(),
    };

    let month: u32 = match &mut in_split.pop() {
        Some(s) => {
            match s.trim().parse() {
                Ok(val) => val,
                Err(_) => return Local::now(),
            }
        }
        None => return Local::now(),
    };

    let year: i32 = match &mut in_split.pop() {
        Some(s) => {
            match s.trim().parse() {
                Ok(val) => val,
                Err(_) => return Local::now(),
            }
        }
        None => Local::now().year(),
    };

    let day = NaiveDate::from_ymd(year, month, date);
    let time = NaiveTime::from_hms(0, 0, 0);
    DateTime::<Local>::from_utc(NaiveDateTime::new(day, time), chrono::offset::FixedOffset::east(0))
}
