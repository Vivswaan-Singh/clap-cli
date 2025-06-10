use std::fs;
use std::path::Path;
use std::error::Error;
use colored::*;
use clap::{Parser,command,Subcommand}; 
use anyhow::{Context,Result};
pub mod cmds;





#[derive(Debug,Subcommand, Clone)]
pub enum Cmnds{
    Echo{
        text: Vec<String>
    },
    Ls,
    Find{
        pattern: String
    },
    Cat{
        files:Vec<String>
    },
    Grep{
        pattern: String,
        files:Vec<String>
    }
}



#[derive(Debug,Parser)]
#[command(propagate_version=true)]
#[command(version)]
pub struct Cli{
    #[command(subcommand)]
    pub cmnd: Cmnds
}


pub fn run(cli: Cli)->Result<(),Box<dyn Error>>{
    match cli.cmnd {
        Cmnds::Echo { text } => {
            if let Err(ref e) = cmds::echo::show(&text){
		        return Err(e.to_string().into()); 
	        } 
        },
        Cmnds::Ls => {
            if let Err(ref e) = cmds::ls::see_folder(Path::new(".")) {
                return Err(e.to_string().into()); 
	        }       
        },
        Cmnds::Grep { pattern, files } => {
            
            let word=&pattern[..];
            for i in files{
                let content=fs::read_to_string(&i)?;
                let ans = cmds::grep::search(word,&content);
                let mut cnt=1;
                println!("Word {} occurs in Document {} in following lines ",word.red(),&i.yellow());
                for line in ans{
                    println!("Line {cnt}. {:?}",line);
                    cnt+=1;
                }
                println!("");
            }
        },
        Cmnds::Cat { files } => {
            for i in files{
                let content=fs::read_to_string(i.clone()).with_context(|| format!("could not read file `{}`", i))?;
                println!("Document {} :",i.purple());
                if let Err(ref e) = cmds::cat::show_file(&content){
		            return Err(e.to_string().into());
	            }
                println!("");
            }
        },
        Cmnds::Find { pattern }=> {
            if let Err(ref e) = cmds::find::find_out(Path::new("."), &pattern) {
		        return Err(e.to_string().into()); 
	        } 
        },
        
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn check_echo(){
        let v=vec![String::from("1")];
        if let Err(ref e) = cmds::echo::show(&v){
		        println!("{}", e);
	        }
    }
}