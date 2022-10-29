use std::error::Error;
use std::fmt::format;
use std::io;
use clap::{Parser};
use std::fs::{read_to_string};

#[derive(Parser, Debug)]
#[command(author = "Jakub Stiburek", version = "0.1.0", about = "Concatenate to standard output.")]
struct Args {
    #[arg(short, long)]
    number: bool,
    
    #[arg(short, long)]
    end: bool,
    
    file: Option<String>,
}

#[derive(Eq, PartialEq)]
enum Flag {
    NUMBER,
    END,
}

fn main() {
    let args = Args::parse();
    
    let mut flags: Vec<Flag> = vec![];
    
    if args.number {
        flags.push(Flag::NUMBER)
    }
    
    if args.end {
        flags.push(Flag::END)
    }
    
    match args.file {
        Some(file) => dump_file(file, flags).unwrap(),
        None => default()
    }
}

fn dump_file(input_file: String, flags: Vec<Flag>) -> Result<(), Box<dyn Error>> {
    if flags.is_empty() {
        println!("{}", read_to_string(input_file).unwrap());
    } else {
        let content = read_to_string(input_file).unwrap();
        let lines = content.split("\n");
        for (i, line) in lines.enumerate() {
            let mut prefix: String = "".to_string();
            let mut suffix: String = "".to_string();
            
            if flags.contains(&Flag::NUMBER) {
                prefix = format!("{}", i).to_string();
            }
            if flags.contains(&Flag::END) {
                suffix = "$".to_string();
            }
            println!("{} {} {}", prefix, line, suffix );
        }
    }
    
    
    
    
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
