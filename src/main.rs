use std::io::Write;
use std::io::stdout;
use std::io::stdin;

fn main(){
    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut command = String::new();
        stdin().read_line(&mut command)
            .expect("Failed to read in command");

        let command_split : Vec<&str> = command.replace("\n", "").split(' ').collect();

        let keyword = command_split[0];
        let args = &command_split[1..];

        println!("{:?}", command_split);
    }
}

