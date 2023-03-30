use std::io::{BufReader, stdin};

use zad0::parse::read;
use zad4::partition;

fn main() {
    let graph = read(&mut BufReader::new(stdin()));

    let part = partition(&graph);

    if let Some((black, white)) = part {
        println!("Bipartite graph");
        if graph.v_size() <= 200 {
            println!("Black: {black:?}");
            println!("White: {white:?}");
        }
    }
    else {
        println!("Not bipartite graph");
    }
}
