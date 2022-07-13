use std::io::{stdin, stdout, Write};

use std::env;
use std::path::Path; //supports file path operations
use std::process::Command; //process builder //helps inspect process environment

fn main() {
    println!("Starting shell...");

    loop {
        print!("> "); //prompt char
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        stdout().flush();

        //handling multiple args
        //everything after 1st whitespace counts towards args
        let mut parts = input.trim().split_whitespace(); //trim removes trailing \n left by readline
        let command = parts.next().unwrap();
        let args = parts;

        
        match command {
            //implementing 'cd' command in shell
            "cd" => {
                //default to home dir '/' if no dir is specified, or dereference the dir in 'x'
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(err) = env::set_current_dir(&root) {
                    eprintln!("Error in cd: {}", err);
                }
                else {
                    print!("{}/ ", new_dir);
                }
            }
            command => {
                let mut child = Command::new(command)
                                        .spawn()
                                        .unwrap();

                //dont accept another command until the current one is complete
                child.wait(); 
            }

        }

        
    }
}
