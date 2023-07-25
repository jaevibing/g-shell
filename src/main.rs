mod autoupdate;
mod download;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::io::stdout;
use std::io::stdin;
use std::process;
use tokio::runtime::Runtime;
use colored::Colorize;

pub fn setToHomeDir(gsh: bool, pathFromHome: &str){
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
    let version = include_str!("VERSION");
    // let version = "v0.0.3"; // debug version number for testing autoupdate
    let mut gitversion = String::new();
    let rt = Runtime::new().unwrap();
    setToHomeDir(false, ".gsh");
    let mut history = File::open(".gsh_history");
    match rt.block_on(autoupdate::checkForUpdate()) {
        Ok(r) => gitversion = r,
        Err(e) => (),
    }
    if gitversion != version {
        println!("You are not currently running the latest version of g-shell.");
        println!("{} -> {}", version.red(), gitversion.green());
        println!("Do you wish to update? (Y/n)");
        let mut choice = String::new();
        stdin().read_line(&mut choice)
            .expect("Could not read input command.");
        if choice == "Y" {
            setToHomeDir(true, ".gsh");
            download::update(gitversion.as_str());
        }
        else {
            println!("Avoiding update, you will be prompted again on next startup.\nYou can update in the terminal with the gsh-update command.\n")
        }
    }
    loop {
        let current_dir = env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();
        print!("gsh [{}] > ", current_dir);
        let _ = stdout().flush();

        let mut command = String::new();
        stdin().read_line(&mut command)
            .expect("Could not read input command.");
        
        /* 
        get ready for some high quality autism in 
        the form of ansi text commands
        */

        print!("\u{1b}[1;A"); // move cursor back to beginning of output
        print!("\r\x1b[K"); // delete output 
        print!("> {command}"); // reprint command without bells and whistles

        command = command.replace("\n",""); // replace new line character in command, just cause

        let command_split : Vec<&str> = command.split(' ').collect();

        let keyword = command_split[0];
        let args = &command_split[1..];

        match keyword{
            "cwd" => {
                match env::current_dir() {
                    Ok(current_dir) => {
                        let current_dir_str = current_dir
                            .to_string_lossy()
                            .into_owned();
                        println!("{}", current_dir_str);
                    }
                    Err(e) => {
                        println!("error getting working directory: {}", e);
                    }
                }
            },
            "help" => {
                let helpfile = include_str!("HELPFILE");
                let helpheader = "--- g-shell ".to_string() + version + " pre-alpha ---";
                let helpend = "--- end help ---\n";
                println!("\n{helpheader}\n\n{helpfile}\n\n{helpend}");
            },
            "end" => {
                println!("exiting with code 0x0100");
                process::exit(0x0100);
            },
            "cd" => {
                let root = Path::new(args[0]);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("error running cd: {}", e);
                }
            },
            "gsh-update" => {
                if gitversion == version{
                    println!("You are already on the latest version of g-shell.\n")
                }
                else{
                    setToHomeDir(true, ".gsh");
                    download::update(gitversion.as_str());
                }
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