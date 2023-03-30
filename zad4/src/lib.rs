use zad0::GraphList;

fn undirect(graph : &GraphList) -> GraphList {
    let mut undir = GraphList::new(graph.v_size(), zad0::GraphType::Undirected);

    for (v, list)in graph.list() {
        for &u in list {
            undir.add_edge(v, u);
        } 
    }

    for (_, list)in undir.list_mut() {
        list.sort_unstable();
        list.dedup();
    }

    undir
}

fn dfs_partition(colors : &mut [i8],  graph : &GraphList, v : usize) -> bool {
    if colors[v] != -1 {
        return true;
    }
    let mut stack = Vec::new();
    stack.push(v);
    colors[v] = 0;
    while let Some(v) = stack.pop() {
        for &u in graph.neigh(v) {
            if colors[u] == -1 {
                colors[u] = 1 - colors[v];
                stack.push(u);
            }
            else if colors[u] == colors[v] {
                return false;
            }
        }
    }
    true
}

pub fn partition(graph : &GraphList) -> Option<(Vec<usize>, Vec<usize>)> {
    let mut colors = vec![-1; graph.v_size() + 1];

    let undir = undirect(graph);

    for (v, _) in undir.list() {
        if !dfs_partition(&mut colors, &undir, v) {
            return None;
        }
    }

    let mut black = Vec::new();
    let mut white = Vec::new();

    for (v, &c) in colors.iter().enumerate().skip(1) {
        if c == 1 {
            white.push(v);
        }
        else if c == 0 {
            black.push(v);
        }
    }

    Some((black, white))
}