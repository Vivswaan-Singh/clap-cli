use std::fs;
use std::error::Error;
use std::path::Path;
use colored::*;

pub fn find_out(dir: &Path, pattern: &str) -> Result<(), Box<dyn Error>> {
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
				let entry = entry?;
				let file_name = entry
						.file_name()
						.into_string()
						.or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
                if file_name.contains(pattern) {
                    print!("{} ",file_name.truecolor(40, 200, 200));
                }
                let way=entry.path();
                if way.is_dir() && way!=dir {
                    if let Err(ref e) = find_out(&way,pattern) {
		            return Err(e.to_string().into()); 
	                }
                }
		}
	}
	Ok(())
}