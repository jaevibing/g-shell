use std::{path::Path, env, fs::{self, File, OpenOptions}, io::Write, };

pub fn saveLastCheck(cacheData: String) -> std::io::Result<()>{
    let cachePath = Path::new(&env::var("HOME").unwrap()).join(".gsh/.gsh_cache");
    match fs::metadata(cachePath.clone()) { // check the history file exists or not
        Ok(_) => (),
        Err(_) => {
            let _ = File::create(Path::new(&env::var("HOME").unwrap()).join(".gsh/.gsh_cache"));
        },
    }

    let mut cacheFile = OpenOptions::new().append(true).open(cachePath)?;
    cacheFile.write(cacheData.as_bytes())?;

    Ok(())
}