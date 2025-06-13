use std::path::Path;
use std::error::Error;
use clap::{command, Parser, Subcommand}; 
use anyhow::{Result};
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
#[command(author, version="1.0")]
#[command(name = "Command Line Functionalities", long_about=None)]
///Can perfrom linux cli commands like ls, echo, cat, find & grep
pub struct Cli{
    #[command(subcommand)]
    pub cmnd: Cmnds
}


pub fn run(cli: Cli)->Result<(),Box<dyn Error>>{
    match cli.cmnd {
        Cmnds::Echo { text } => {
            return cmds::echo::show(&text);
        },
        Cmnds::Ls => {
            return cmds::ls::see_folder(Path::new("."));
        },
        Cmnds::Grep { pattern, files } => {
            return cmds::grep::grep_parallel(pattern, files);
        },
        Cmnds::Cat { files } => {
            return cmds::cat::cat(files);
        },
        Cmnds::Find { pattern }=> {
            return cmds::find::find_out(Path::new("."), &pattern);
        },
    }
    
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_search(){
        let q="you";
        let f="\
        Hi Hello how are you ?
        I am fine. Thnak You for asking. ";
        assert_eq!(vec!["Line 1: Hi Hello how are you ?"],cmds::grep::search(q, f));
    }
}