use clap::Clap;
use std::fs;
use serde_json::Value;
use std::vec::Vec;

pub mod project;
use project::Project;

pub mod punching;
pub mod reporting;

// CLI options
#[derive(Clap)]
#[clap(version="1.0")]
pub struct Opts {
    #[clap(short, long, about="Project to use. Pass \"LIST\" to list options")]
    pub project: String,

    #[clap(long, about="Add new project to config")]
    pub add: bool,

    #[clap(long, about="Punch in")]
    pub punch: bool,

    #[clap(long, about="Punch out")]
    pub punch_out: bool,

    #[clap(long, about="Create report for this project")]
    pub report: bool,

    #[clap(long, about="Open projects data file")]
    pub projects: bool,

    #[clap(long, about="Open tracking data file")]
    pub tracker: bool,
}

// write projects vector to json file
pub fn write_projects_to_json(projects : &Vec<Value>, path : &str) {
    let proj_str = serde_json::to_string_pretty(&projects).unwrap_or_else(|err| {
        eprintln!("Unable to save projects to file! {}",  err);
        return String::new();
    });

    match fs::write(&path, &proj_str) {
        Ok(_) => return,
        Err(_) => return,
    };
}

// load vector of projects from json file
pub fn get_projects_from_json(path : &str) -> Result<Vec<Value>, &'static str> {
    let projects_string = match fs::read_to_string(path) {
        Ok(arg) => arg,
        Err(_) => return Err("Unable to load projects file!"),
    };

    let projects_json: Value = match serde_json::from_str(&projects_string) {
        Ok(arg) => arg,
        Err(_) => return Err("Unable to parse projects file!"),
    };

    if let Value::Array(arr) = projects_json {
        Ok(arr)
    } else {
        Err("Unable to load projects as array")
    }
}

// get project from vector of projects
pub  fn get_project(projects : &Vec<Value>, tag : &str) -> Option<Project> {
    for p in projects.iter() {
        let proj: Project = serde_json::from_str(&p.to_string()).unwrap();
        if proj.get_tag() == tag {
            return Some(proj)
        }
    }

    None
}

// list all available projects
pub fn list_projects(projects : &Vec<Value>) {
    println!("These are the available projects:");

    for p in projects.iter() {
        let proj: Project = serde_json::from_str(&p.to_string()).unwrap();
        println!("\t {}: {}", proj.get_tag(), proj.get_name());
    }
}
