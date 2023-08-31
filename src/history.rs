use std::{fs::{self, File, OpenOptions}, path::Path, env, io::Write};

pub fn writeToHistory(command: &str) -> std::io::Result<()>{
    let historyPath = Path::new(&env::var("HOME").unwrap()).join(".gsh/.gsh_history");
    match fs::metadata(historyPath.clone()) { // check the history file exists or not
        Ok(_) => (),
        Err(_) => {
            let _ = File::create(Path::new(&env::var("HOME").unwrap()).join(".gsh/.gsh_history"));
        },
    }

    let mut historyFile = OpenOptions::new().append(true).open(historyPath)?;
    historyFile.write_all(command.as_bytes())?;

    Ok(())
}