use clap::Clap;
use punch_clock::{Opts,project::Project,punching,reporting};
use punch_clock as pc;
use std::process;
use std::process::Command;
use serde_json::{json};

// @TODO: get these for the user system
const PROJ_JSON: &str = "C:\\Users\\jatin\\AppData\\Local\\Jatin_Chowdhury\\punch_clock\\projects.json";
const TIME_FILE: &str = "C:\\Users\\jatin\\AppData\\Local\\Jatin_Chowdhury\\punch_clock\\tracker.log";
const CODE_EXE: &str = "C:\\Program Files\\Microsoft VS Code\\Code.exe";

fn main() {
    let opts = Opts::parse();

    if opts.projects {
        Command::new(&CODE_EXE)
            .arg(&PROJ_JSON)
            .spawn()
            .expect("VSCode failed to open projects json!");
        return;
    }

    if opts.tracker {
        Command::new(&CODE_EXE)
            .arg(&TIME_FILE)
            .spawn()
            .expect("VSCode failed to open tracking log!");
        return;
    }

    // load projects from file
    let mut projects = pc::get_projects_from_json(&PROJ_JSON).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if opts.project == "LIST" {
        pc::list_projects(&projects);
    } else if opts.add {
        let new_proj = Project::new(&opts.project);
        projects.push(json!(new_proj));
    } else {
        // get the project to operate on
        let proj = match pc::get_project(&projects, &opts.project) {
            Some(p) => p,
            None => {
                println!("Project not found!");
                pc::list_projects(&projects);
                process::exit(1);
            }
        };

        if opts.punch {
            punching::punch(proj.get_tag(), &TIME_FILE, true).unwrap_or_else(|err| {
                eprintln!("Problem punching in: {}", err);
                process::exit(1);
            });
        } else if opts.punch_out {
            punching::punch(proj.get_tag(), &TIME_FILE, false).unwrap_or_else(|err| {
                eprintln!("Problem punching in: {}", err);
                process::exit(1);
            });
        } else if opts.report {
            reporting::create_report(&proj, &TIME_FILE);
        } else {
            println!("No action chosen!");
        }
    }

    // write projects to file
    pc::write_projects_to_json(&projects, &PROJ_JSON);
}
