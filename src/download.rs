use std::io::Cursor;

use crate::setToHomeDir;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
 
fn downloadToFile(url: String, file_name: String) -> Result<()> {
    let response = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes()?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
 
pub fn update(ver: &str) {
    setToHomeDir(true, ".gsh/bin");
    let binaryurl = "https://github.com/jaevibing/g-shell/releases/download/".to_string() + ver + "/gsh";
    downloadToFile(binaryurl, "gsh".to_string()).unwrap();
    println!("New binary downloaded, restarting...");
}