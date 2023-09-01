#![allow(non_snake_case, unused_must_use)]
mod autoupdate;
mod download;
mod history;
mod saveLastCheck;
mod keyFunctions;

use termion::raw::IntoRawMode;
use tokio::runtime::Runtime;
use colored::Colorize;
use std::{env, io::{Write, stdout, stdin}, path::Path, process::{Command, self}, time::{SystemTime, UNIX_EPOCH}, thread, sync::{Arc, Mutex}};

pub fn setToHomeDir(gsh: bool, pathFromHome: &str){
    #[allow(deprecated)]
    match env::home_dir() {
        Some(mut home_dir) => {
            if gsh{
                home_dir.push(pathFromHome);
            }
            if let Err(e) = env::set_current_dir(home_dir) {
                eprintln!("Error setting current directory: {}", e);
            } else {
                ()
            }
        }
        None => {
            eprintln!("Failed to get home directory.");
        }
    }
}

fn main(){
    //let mut termout_lock = std::io::stdout().into_raw_mode().unwrap();
    let termout = Arc::new(Mutex::new(std::io::stdout().into_raw_mode().unwrap()));
    let mut termout_lock = termout.lock().unwrap();

    let version = include_str!("docs/VERSION");
    // let version = "v0.0.3"; // debug version number for testing autoupdate
    let rt = Runtime::new().unwrap();
    setToHomeDir(false, ".gsh");
    let gitversion = match rt.block_on(autoupdate::checkForUpdate()) {
        Ok(r) => r,
        Err(_) => version.to_string(), // if check fails assume latest version
    };

    drop(rt);

    if gitversion != version {
        writeln!(termout_lock, "You are not currently running the latest version of g-shell.");
        writeln!(termout_lock, "{} -> {}", version.red(), gitversion.green());
        writeln!(termout_lock, "Do you wish to update? (Y/n)");
        let mut choice = String::new();
        stdin().read_line(&mut choice)
            .expect("Could not read input command.");
        write!(termout_lock, "{choice}");
        if choice.contains("Y") {
            setToHomeDir(true, ".gsh");
            download::update(gitversion.as_str());
        }
        else {
            writeln!(termout_lock, "Avoiding update, you will be prompted again on next startup.\nYou can update in the terminal with the gsh-update command.\n");
            let value = gitversion.clone() + ":" + &SystemTime::now().duration_since(UNIX_EPOCH)
                                                    .expect("oopsie poopsie a friggin time error beitch")
                                                    .as_secs()
                                                    .to_string();
            let _ = saveLastCheck::saveLastCheck(value);
        }
    }

    thread::spawn({
        let termout = Arc::clone(&termout); // share termout with thread
        move || {
            keyFunctions::getKeyPresses(termout);
        }
    });
    
    loop {
        let current_dir = env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();
        write!(termout_lock, "gsh [{}] $ ", current_dir);
        let _ = stdout().flush();

        let mut command = String::new();
        stdin().read_line(&mut command)
            .expect("Could not read input command.");
        
        /* 
        get ready for some high quality autism in 
        the form of ansi text commands
        */

        write!(termout_lock, "\u{1b}[1;A"); // move cursor back to beginning of output
        write!(termout_lock, "\r\x1b[K"); // delete output 
        write!(termout_lock, "$ {command}"); // reprint command without bells and whistles

        let _ = history::writeToHistory(&command);

        command = command.replace("\n",""); // replace new line character in command, just cause

        let command_split : Vec<&str> = command.split(' ').collect();

        let keyword = command_split.clone()[0];
        let args = &command_split.clone()[1..];

        drop(command_split);

        match keyword{
            "cwd" => {
                match env::current_dir() {
                    Ok(current_dir) => {
                        let current_dir_str = current_dir
                            .to_string_lossy()
                            .into_owned();
                        drop(current_dir);
                        writeln!(termout_lock, "{}", current_dir_str);
                    }
                    Err(e) => {
                        writeln!(termout_lock, "error getting working directory: {}", e);
                    }
                }
            },
            "help" => {
                let helpfile = include_str!("docs/HELPFILE");
                let helpheader = "--- g-shell ".to_string() + version + " pre-alpha ---";
                let helpend = "--- end help ---\n";
                writeln!(termout_lock, "\n{helpheader}\n\n{helpfile}\n\n{helpend}");
            },
            "exit" => {
                writeln!(termout_lock, "exiting with code 0x0100");
                process::exit(0x0100);
            },
            #[allow(deprecated)]
            "cd" => {
                let directory = args[0].replace("~", &env::home_dir().unwrap().to_string_lossy().into_owned());
                let root = Path::new(&directory);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("error running cd: {}", e);
                }
            },
            "gsh-update" => {
                if gitversion == version{
                    writeln!(termout_lock, "You are already on the latest version of g-shell.\n");
                }
                else{
                    setToHomeDir(true, ".gsh");
                    download::update(gitversion.as_str());
                }
            },
            "gsh-info" => {
                write!(termout_lock, "gsh pre-alpha {}\n", version);
            },
            keyword => {
                let mut child = Command::new(keyword)
                    .args(args)
                    .spawn()
                    .unwrap();

                child.wait().expect("Error encountered, be concerned!");
            }
        }
    }
}