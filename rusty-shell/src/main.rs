
use std::io::{stdin, stdout, Write};

use std::process::Command; //process builder

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


        //implementing 'cd' command
        match command {
            "cd" => {

            },
            _ => println!("error")
        }

        let mut child = Command::new(command)
                                .spawn()
                                .unwrap();

        child.wait(); //dont accept another command until the current one is complete

    }

    
}
