use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use util::db_path;
use super::util;

// Function to extract data from a URL and save it to a file
pub fn extract(url: &str, file_name: &str) -> Result<&'static str, Box<dyn std::error::Error>> {
    // Define the full file path
    // let file_path = Path::new("./db_path").join(file_name);  // Adjust db_path if necessary
    
    // Fetch the URL content
    let response = get(url)?;
    
    // Open the file to write the response content
    // let mut file = File::create(file_path)?;
    // let mut file = File::create(db_path(file_name))?;
    let mut file = File::create(file_name)?;


    // Write the content to the file
    file.write_all(&response.bytes()?)?;
    
    Ok("Extract Successful")
}



fn main() {
    match extract("https://data.cityofnewyork.us/resource/c3uy-2p5r.csv?$limit=200000", "air_quality.csv") {
        Ok(message) => println!("{}", message),
        Err(e) => eprintln!("Failed to extract data: {}", e),
    }
}
