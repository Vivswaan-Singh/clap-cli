use std::error::Error;

pub fn show_file(content: &String)->Result<(),Box<dyn Error>>{
    println!("{}",content);
    println!("");
    Ok(())
}