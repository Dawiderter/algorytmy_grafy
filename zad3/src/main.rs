use std::io::{BufReader, stdin};

use zad0::parse::read;
use zad3::scc;

fn main() {
    let graph = read(&mut BufReader::new(stdin()));

    let components = scc(&graph);

    println!("Components: {}", components.len());

    for (&root, comp) in components.iter() {
        println!("Root: {} || Size: {}", root, comp.len());
        if graph.v_size() <= 200 {
            println!("{comp:?}");
        }
    }
}
