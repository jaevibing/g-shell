use std::io::Write;
use std::process::Command;
use std::io::stdout;
use std::io::stdin;
use std::fs;
use std::process;

fn main(){
    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut command = String::new();
        stdin().read_line(&mut command)
            .expect("Failed to read in command");

        command = command.replace("\n",""); // replace new line character in command, just cause

        let command_split : Vec<&str> = command.split(' ').collect();

        let keyword = command_split[0];
        let args = &command_split[1..];

        match keyword{
            "help" => {
                let helpfile = fs::read_to_string("src/HELPFILE").expect("Failed to read the file.");
                println!("{}",helpfile);
            }
            "end" => {
                println!("exiting with code 0x0100");
                process::exit(0x0100);
            }
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