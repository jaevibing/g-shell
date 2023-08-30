use std::io::Cursor;

use crate::setToHomeDir;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
 
async fn fetch_url(url: String, file_name: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
 
pub async fn update(ver: &str) {
    setToHomeDir(true, ".gsh/bin");
    let binaryurl = "https://github.com/jaevibing/g-shell/releases/download/".to_string() + ver + "/gsh";
    fetch_url(binaryurl, "gsh".to_string()).await.unwrap();
    println!("New binary downloaded, restarting...");
}