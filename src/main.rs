use std::{process::Command, fs, io::{self, Read}};

fn main() {
    let addresses = match fs::read_to_string(".addresses.txt") {
        Ok(addresses) => addresses,
        Err(_) => populate_new_file(),
    };

    for address in addresses.lines() {
        let output = if cfg!(target_os = "windows") {
            Command::new("ping")
                .args(&["/n", "1", address])
                .output()
                .expect("failed to execute process")            
        } else {
            Command::new("ping")
                .arg("-c")
                .arg("1")
                .arg(address)
                .output()
                .expect("failed to execute process")
        };
        
        if output.status.success() {
            println!("{}is up \x1b[5;32m•\x1b[0m", format!("{0:15}", address));
        } else {
            println!("{}is \x1b[1mdn\x1b[0m \x1b[2;31m•\x1b[0m", format!("{0:15}", address));
        }
    }
}

fn populate_new_file() -> String {
    println!("Could not read .addresses.txt\nCreating file...");
    println!("Please enter one address per line (send EOF when done):");

    let mut new_addresses = String::new();
    io::stdin()
        .read_to_string(&mut new_addresses)
        .expect("Failed to read line");

    fs::write(".addresses.txt", &new_addresses)
        .expect("Failed to write file");

    println!("File created successfully\n");
    new_addresses
}