use std::io;
use serde::{Deserialize,Serialize};

// function to get string value from CLI
fn get_val(prompt : &str) -> String {
    println!("{}", prompt);
    let mut val = String::new();
    io::stdin().read_line(&mut val).expect("Invalid!");

    val.trim().to_string()
}

// Address struct
#[derive(Serialize,Deserialize)]
pub struct Address {
    line1: String,
    line2: String,
    city: String,
    state: String,
    zip: String,
    country: String,
}

impl Address {
    pub fn new() -> Address {
        let line1   = get_val("Address Line 1: ");
        let line2   = get_val("Address line 2: ");
        let city    = get_val("City: ");
        let state   = get_val("State: ");
        let zip     = get_val("Zip: ");
        let country = get_val("Country: ");

        Address { line1, line2, city, state, zip, country }
    }

    pub fn get_string(&self) -> String {
        if self.line2.is_empty() {
            let addr_str = format!("{}\n{}, {}, {}, {}", &self.line1, &self.city, &self.state, &self.zip, &self.country);
            return addr_str;
        } else {
            let addr_str = format!("{}\n{}\n{}, {}, {}, {}", &self.line1, &self.line2, &self.city, &self.state, &self.zip, &self.country);
            return addr_str;
        }
    }
}

// project struct
#[derive(Serialize,Deserialize)]
pub struct Project {
    tag: String,
    name: String,
    address: Address,
    // phone: String,
}

impl Project {
    pub fn new(tag : &str) -> Project {
        let tag = String::from(tag);

        println!("Creating new project with tag: {}", tag);

        let name = get_val("Project name: ");
        let address = Address::new();

        Project { tag, name, address }
    }

    pub fn get_tag(&self) -> &String {
        &self.tag
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_address(&self) -> &Address {
        &self.address
    }
}
