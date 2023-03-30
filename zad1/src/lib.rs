use std::collections::{VecDeque};

use zad0::GraphList;

pub mod print;

type PopFn = dyn Fn(&mut VecDeque<(Option<usize>, usize)>) -> Option<(Option<usize>, usize)>;

fn search_visit(graph: &GraphList, visited: &mut [bool], popfn: &PopFn, list: &mut Vec<(Option<usize>, usize)>, v: usize) {
    let mut vecd = VecDeque::new();
    
    vecd.push_back((None, v));

    while let Some((p, v)) = popfn(&mut vecd) {
        if visited[v] {
            continue;
        }
        visited[v] = true;
        
        for &u in graph.neigh(v) {
            vecd.push_back((Some(v),u));
        }
        
        list.push((p,v));
    }
}

fn search(graph: &GraphList, popfn: &PopFn) -> Vec<(Option<usize>, usize)>{
    let mut list = Vec::new();
    let mut visited = vec![false; graph.v_size() + 1];

    for (v, _) in graph.list() {
        search_visit(graph, &mut visited, popfn, &mut list, v);
    }

    list
}

pub fn dfs(graph: &GraphList) -> Vec<(Option<usize>, usize)> {
    search(graph, &|vec| vec.pop_back())
}

pub fn bfs(graph: &GraphList) -> Vec<(Option<usize>, usize)> {
    search(graph, &|vec| vec.pop_front())
}
