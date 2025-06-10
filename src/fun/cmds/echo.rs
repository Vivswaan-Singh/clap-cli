use colored::*;
use std::error::Error;

pub fn show(ips: &Vec<String>) -> Result<(), Box<dyn Error>> {
	for i in ips{
        print!("{} ",i.red().on_yellow());
    }
    println!("");
	Ok(())
}