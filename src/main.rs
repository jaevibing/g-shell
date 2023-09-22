#![allow(non_snake_case, unused_must_use)]
mod autoupdate;
mod download;
mod history;
mod saveLastCheck;

use termion::{raw::IntoRawMode, input::TermRead, event::Key};
use colored::Colorize;
use std::{env, io::{Write, stdin}, path::Path, process::{Command, self}, time::{SystemTime, UNIX_EPOCH}};

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
    //let mut termout = std::io::stdout().into_raw_mode().unwrap();
    let mut termout = std::io::stdout().into_raw_mode().unwrap();

    let version = include_str!("docs/VERSION");
    // let version = "v0.0.3"; // debug version number for testing autoupdate
    setToHomeDir(false, ".gsh");
    let gitversion = match autoupdate::checkForUpdate() {
        Ok(r) => r,
        Err(_) => version.to_string(), // if check fails assume latest version
    };

    if gitversion != version {
        writeln!(termout, "You are not currently running the latest version of g-shell.");
        writeln!(termout, "{} -> {}", version.red(), gitversion.green());
        writeln!(termout, "Do you wish to update? (Y/n)");
        let mut choice = String::new();
        stdin().read_line(&mut choice)
            .expect("Could not read input command.");
        write!(termout, "{choice}");
        if choice.contains("Y") {
            setToHomeDir(true, ".gsh");
            download::update(gitversion.as_str());
        }
        else {
            writeln!(termout, "Avoiding update, you will be prompted again on next startup.\nYou can update in the terminal with the gsh-update command.\n");
            let value = gitversion.clone() + ":" + &SystemTime::now().duration_since(UNIX_EPOCH)
                                                    .expect("oopsie poopsie a friggin time error beitch")
                                                    .as_secs()
                                                    .to_string();
            let _ = saveLastCheck::saveLastCheck(value);
        }
    }
    
    loop {
        let current_dir = env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();
        write!(termout, "gsh [{}] $ ", current_dir);
        termout.flush();

        let mut cursor_pos = 0;
        let mut command = String::new();

        loop {
            if let Some(Ok(key_event)) = stdin().keys().next() {
                match key_event {
                    Key::Char('\n') => {
                        write!(termout, "\n");
                        break;
                    }
                    Key::Left => {
                        if cursor_pos == 0 {
                            write!(termout, "\x07");
                        }
                        else {
                            write!(termout, "\x1b[D");
                            cursor_pos -= 1;
                        }
                    }
                    Key::Right => {
                        if cursor_pos == command.as_str().len() {
                            write!(termout, "\x07");
                        }
                        else {
                            write!(termout, "\x1b[C");
                            cursor_pos += 1;
                        }
                    }
                    Key::Up => {
                        write!(termout, "\x07");
                    }
                    Key::Down => {
                        write!(termout, "\x07");
                    }
                    Key::Char(n) => {
                        command.push_str(n.to_string().as_str());
                        cursor_pos += 1;
                        write!(termout, "{}", n);
                    }
                    Key::Backspace | Key::Delete => {
                        command.truncate(command.as_str().len()-1);
                        write!(termout, "\r\x1b[K");
                        write!(termout, "gsh [{}] $ {}", env::current_dir().unwrap().to_string_lossy().to_string(), command);
                        cursor_pos -= 1;
                    }
                    _ => todo!(),
                }
            }
    
            termout.flush();
        }
        
        /* 
        get ready for some high quality autism in 
        the form of ansi text commands
        */

        write!(termout, "\u{1b}[1;A"); // move cursor back to beginning of output
        write!(termout, "\r\x1b[K"); // delete output 
        write!(termout, "$ {command}\n\r"); // reprint command without bells and whistles
        termout.flush();

        history::writeToHistory(&command);

        command = command.replace("\n",""); // replace new line character in command, just cause

        let command_split : Vec<&str> = command.split(' ').collect();

        let keyword = command_split.clone()[0];
        let args = &command_split.clone()[1..];

        drop(command_split);
        let mut clear = false;

        match keyword{
            "cwd" => {
                match env::current_dir() {
                    Ok(current_dir) => {
                        let current_dir_str = current_dir
                            .to_string_lossy()
                            .into_owned();
                        drop(current_dir);
                        writeln!(termout, "{}", current_dir_str);
                    }
                    Err(e) => {
                        writeln!(termout, "error getting working directory: {}", e);
                    }
                }
            },
            "help" => {
                let helpfile = include_str!("docs/HELPFILE");
                let helpheader = "--- g-shell ".to_string() + version + " pre-alpha ---";
                let helpend = "--- end help ---\n";
                writeln!(termout, "\n{helpheader}\n\n{helpfile}\n\n{helpend}");
            },
            "exit" => {
                writeln!(termout, "exiting with code 0x0100");
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
                    writeln!(termout, "You are already on the latest version of g-shell.\n");
                }
                else{
                    setToHomeDir(true, ".gsh");
                    download::update(gitversion.as_str());
                }
            },
            "gsh-info" => {
                write!(termout, "gsh pre-alpha {}\n", version);
            },
            "clear-history" =>{
                history::clearHistory();
            },
            keyword => {
                if keyword == "clear"{
                    clear = true;
                }
                let child = Command::new(keyword)
                    .args(args)
                    .output()
                    .expect("Failed to execute command");

                let stdout_child = String::from_utf8_lossy(&child.stdout).replace("\n", "\n\r");

                write!(termout, "{}", stdout_child);

                termout.flush();
            }
        }
        if !clear {
            write!(termout, "\n\r");
        }
    }
}