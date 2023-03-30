use std::{io::{stdin, BufReader}, env::args};

use zad0::parse::read;
use zad1::{dfs, bfs, print::search_tree};

enum SType {
    Dfs, Bfs, UnDef
}

fn main() {

    let mut search_type = SType::UnDef;
    let mut tree = false;

    for arg in args() {
        if arg == "dfs" {
            search_type = SType::Dfs;
        }
        else if arg == "bfs" {
            search_type = SType::Bfs;
        }
        else if arg == "tree" {
            tree = true;
        }
    }

    let graph = read(&mut BufReader::new(stdin()));
    
    let search = match search_type {
        SType::Dfs => dfs(&graph),
        SType::Bfs => bfs(&graph),
        SType::UnDef => panic!("Need to specify search method (dfs/bfs)"),
    };

    if tree {
        let tree = search_tree(&search, graph.v_size());
        println!("{tree}");
    }
    else {
        let order : Vec<usize> = search.into_iter().map(|(_,v)| v).collect(); 
        println!("{order:?}"); 
    }
}
