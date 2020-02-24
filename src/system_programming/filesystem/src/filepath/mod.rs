use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn replace_in_file(file_path: &str, from: &str, to: &str) -> Result<(), std::io::Error> {
	// Open and read the file entirely
	let mut src = File::open(&Path::new(&file_path))?;
	let mut data = String::new();
	src.read_to_string(&mut data)?;
	drop(src); // Close the file early

	// Run the replace operation in memory
	let new_data = data.replace(from, to);

	// Recreate the file and dump the processed contents to it
	let mut dst = File::create(&file_path)?;
	dst.write_all(new_data.as_bytes())?;
	Ok(())
}

pub fn read_file_ok(file_path: &str) -> Result<(), std::io::Error> {
    let mut src = File::open(&Path::new(&file_path))?;
    let mut data = String::new();
    src.read_to_string(&mut data)?;
    Ok(())
}

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut src = File::open(&Path::new(&file_path))?;
    let mut data = String::new();
    src.read_to_string(&mut data)?;
    Ok(data)
}
