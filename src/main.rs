use std::error::Error;
use std::io;
use clap::{Parser};
use std::fs::{read_to_string};

#[derive(Parser, Debug)]
#[command(author = "Jakub Stiburek", version = "0.1.0", about = "Concatenate to standard output.")]
struct Args {
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    
    match args.file {
        Some(file) => dump_file(file).unwrap(),
        None => default()
    }
}

fn dump_file(input_file: String) -> Result<(), Box<dyn Error>> {
    println!("{}", read_to_string(input_file).unwrap());
    
    Ok(())
}

fn default() {
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        println!("{input}");
    }
}
