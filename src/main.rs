use std::fs; 
use std::io; 


fn main() {
    std::process::exit(decompress()); 
}

fn decompress() -> i32 {
    let _args: Vec<_> = std::env::args().collect();
    if _args.len() < 2 {
        println!("Usage: {} <filename>", _args[0]); 
        return 1; 
    }       
}
