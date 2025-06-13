use clap::Parser;
pub mod fun;
use anyhow::{Result};

fn main()-> Result<(), Box<dyn std::error::Error>>  {
    
    let cli = fun::Cli::parse();

    fun::run(cli)?;
    
    Ok(())

}