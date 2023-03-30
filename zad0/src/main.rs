use std::io::{BufReader, stdin};

use zad0::{parse::read, print::PrintDot};

fn main() {
    let graph = read( &mut BufReader::new(stdin()));
        
    let dot = graph.to_dot();
    
    println!("{dot}");
}