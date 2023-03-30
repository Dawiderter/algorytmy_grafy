use zad0::GraphList;

pub fn topological_sort(graph : &GraphList) -> Option<Vec<usize>> {
    let mut indegrees = vec![0;graph.v_size() + 1];
    
    for (_, neigh) in graph.list() {
        for &u in neigh {
            indegrees[u] += 1;
        }
    }

    let mut independ = Vec::new();

    for (v, &i) in indegrees.iter().enumerate().skip(1) {
        if i == 0 {
            independ.push(v);
        }
    }

    let mut sorted = Vec::new();

    while let Some(v) = independ.pop() {
        sorted.push(v);
        for &u in graph.neigh(v) {
            indegrees[u] -= 1;
            if indegrees[u] == 0 {
                independ.push(u);
            }
        }
    }

    if sorted.len() == graph.v_size() {
        Some(sorted)
    }
    else {
        None
    }
}
