use std::io::{self, Error};
use std::fs;
fn write_to_file() -> Result<String, Error> {
    let mut file_name = String::new();
    let mut file_content = String::new();
    println!("Enter the file name: ");
    io::stdin().read_line(&mut file_name)?;
    println!("Enter the contents: ");
    io::stdin().read_line(&mut file_content)?;
    
    fs::write(file_name.trim(), file_content)?;
    Ok(file_name)

}
fn main() {
    match write_to_file() {
        Ok(filename) => println!("Successfully wrote to {:?}", filename),
        Err(error) => println!("Error in writing to file {:?}", error),
    }
}
