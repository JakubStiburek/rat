use std::error::Error;
use std::io;
use clap::{Parser};
use std::fs::{read_to_string};

#[derive(Parser, Debug)]
#[command(author = "Jakub Stiburek", version = "0.1.0", about = "Concatenate to standard output.")]
struct Args {
    #[arg(short, long, help = "number all output lines")]
    number: bool,
    
    #[arg(short, long = "show-ends", help = "display $ at end of each line")]
    end: bool,
    
    #[arg(short = 'b', long, help = "number nonempty output lines, overrides -n")]
    number_nonblank: bool,
    
    file: Option<String>,
}

#[derive(Eq, PartialEq)]
enum Flag {
    NUMBER,
    END,
    NONBLANK
}

fn main() {
    let args = Args::parse();
    
    let mut flags: Vec<Flag> = vec![];
    
    if args.number_nonblank {
        flags.push(Flag::NONBLANK)
    } else if args.number {
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
        let mut lines_skipped = 0;
        let content = read_to_string(input_file).unwrap();
        let lines: Vec<&str> = content.split("\n").collect();
        for (i, line) in lines.iter().enumerate() {
            let mut output: String = line.to_string();
            
            if flags.contains(&Flag::END) {
                output = format!("{} $", line);
            }
            
            if flags.contains(&Flag::NUMBER) {
                output = format!("{} {}", i, output);
            }
            
            if flags.contains(&Flag::NONBLANK) && output.len() > 1 {
                output = format!("{} {}", i - lines_skipped, output);
            } else {
                lines_skipped += 1;
            }
            
            println!("{}", output)
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
