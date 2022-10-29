use std::io;

fn main() {
    default();
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
