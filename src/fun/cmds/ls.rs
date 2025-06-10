use std::path::Path;
use std::error::Error;
use colored::*;
use std::fs;

pub fn see_folder(dir: &Path) -> Result<(), Box<dyn Error>> {
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
				let entry = entry?;
				let file_name = entry
						.file_name()
						.into_string()
						.or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
				print!("{} ", file_name.truecolor(40, 200, 200));
		}
	}
	Ok(())
}