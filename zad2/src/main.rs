use std::io::{stdin, BufReader};

use zad0::parse::read;
use zad2::topological_sort;

fn main() {
    let graph = read(&mut BufReader::new(stdin()));

    if let Some(sorted) = topological_sort(&graph) {
        println!("Acyclic graph");
        if graph.v_size() <= 200 {
            println!("Ordering: {sorted:?}");
        }
    }
    else {
        println!("Cyclic graph");
    }
}
