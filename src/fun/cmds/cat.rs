use std::error::Error;
use std::fs;
use colored::*;
use anyhow::{Context,Result};

pub fn cat(files:Vec<String>)->Result<(),Box<dyn Error>>{
            for i in files{
                let content=fs::read_to_string(&i).with_context(|| format!("could not read file `{}`", i))?;
                println!("Document {} :",i.purple());
                if let Err(ref e) = show_file(&content){
		            return Err(e.to_string().into());
	            }
                println!("");
            }
            Ok(())    
}

pub fn show_file(content: &String)->Result<(),Box<dyn Error>>{
    println!("{}",content);
    println!("");
    Ok(())
}