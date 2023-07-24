use std::env;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::io::stdout;
use std::io::stdin;
use std::fs;
use std::process;

fn main(){
    loop {
        print!("gsh> ");
        let _ = stdout().flush();

        let mut command = String::new();
        stdin().read_line(&mut command)
            .expect("Failed to read in command");

        command = command.replace("\n",""); // replace new line character in command, just cause

        let command_split : Vec<&str> = command.split(' ').collect();

        let keyword = command_split[0];
        let args = &command_split[1..];

        match keyword{
            "cwd" => {
                match env::current_dir() {
                    Ok(current_dir) => {
                        let current_dir_str = current_dir.to_string_lossy().into_owned();
                        println!("{}", current_dir_str);
                    }
                    Err(e) => {
                        eprintln!("error getting working directory: {}", e);
                    }
                }
            },
            "help" => {
                let helpfile = include_str!("HELPFILE");
                println!("{helpfile}");
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
            keyword => {
                let mut child = Command::new(keyword)
                    .args(args)
                    .spawn()
                    .unwrap();

                child.wait();
            }
        }
    }
}