
use std::io::{stdin, stdout, Write};

use std::process::Command; //process builder

fn main() {
    println!("Starting shell...");


    loop {

        print!("> "); //prompt char
        

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        stdout().flush();
        
        //only takes in one word commands at the moment
        let command = input.trim();//removes the trailing newline left by read_line

        let mut child = Command::new(command)
                                .spawn()
                                .unwrap();

        child.wait(); //dont accept another command until the current one is complete

    }

    
}
