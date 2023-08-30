extern crate reqwest;
extern crate tokio;
use std::{fs, path::Path, env, time::{SystemTime, UNIX_EPOCH}};

pub async fn checkForUpdate() -> Result<String, reqwest::Error> {
    let mut cachePassed: bool = true;
    match fs::metadata(Path::new(&env::var("HOME").unwrap()).join(".gsh/.gsh_cache")){
        Ok(_) => {
            let cachePath = Path::new(&env::var("HOME").unwrap()).join(".gsh/.gsh_cache");
            let cachedContents = fs::read_to_string(cachePath).expect("oops");
            let cachedUnix: u32 = cachedContents.split(":").last().unwrap().parse().unwrap();
            let currentUnix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            if (currentUnix - u64::from(cachedUnix)) > 432000 { // 5 days in seconds
                ()
            }
            else {
                cachePassed = false;
            }
        },
        Err(_) => (),
    }

    if !cachePassed{
        return Ok::<String, reqwest::Error>(include_str!("docs/VERSION").to_string()); // return OK if cache isnt old enough yet
    }

    let url = "https://raw.githubusercontent.com/jaevibing/g-shell/master/src/docs/VERSION";
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        Err(response.error_for_status_ref().err().unwrap())
    }
}