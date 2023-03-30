use std::collections::{BTreeMap, VecDeque};

use zad0::GraphList;

pub fn invert_graph(graph: &GraphList) -> GraphList {
    let mut inverted = GraphList::new(graph.v_size(), graph.g_type());

    for (v, list) in graph.list() {
        for &u in list {
            inverted.add_edge(u, v);
        }
    }

    inverted
}

fn dfs_visit(visited: &mut [bool], cur_arc: &mut [usize], list: &mut VecDeque<usize>, graph: &GraphList, v: usize) {
    if visited[v] {
        return;
    }
    let mut stack = Vec::new();
    stack.push(v);
    visited[v] = true;
    while let Some(&v) = stack.last() {
        if let Some(u) = find_adm_arc(visited, cur_arc, graph, v) {
            visited[u] = true;
            stack.push(u);
        }
        else {
            stack.pop();
            list.push_front(v);
        }
    }
}

fn find_adm_arc(visited: &mut [bool], cur_arc: &mut [usize], graph: &GraphList, v: usize) -> Option<usize> {
    while let Some(&u) = graph.neigh(v).get(cur_arc[v]) {
        cur_arc[v] += 1;
        if !visited[u] {
            return Some(u);
        }
    }
    None
}

fn dfs_assign(
    assigned: &mut [bool],
    components: &mut BTreeMap<usize, Vec<usize>>,
    inverted: &GraphList,
    root: usize,
) {
    if assigned[root] {
        return;
    }

    components.insert(root, Vec::new());

    let mut stack = Vec::new();
    stack.push(root);

    while let Some(v) = stack.pop() {
        if assigned[v] {
            continue;
        }
        assigned[v] = true;
        components.get_mut(&root).unwrap().push(v);
        for &u in inverted.neigh(v) {
            stack.push(u);
        }
    }

}

pub fn scc(graph: &GraphList) -> BTreeMap<usize, Vec<usize>> {
    let mut list = VecDeque::new();
    let mut cur_arc = vec![0; graph.v_size() + 1];
    let mut visited = vec![false; graph.v_size() + 1];

    for (v, _) in graph.list() {
        dfs_visit(&mut visited, &mut cur_arc, &mut list, graph, v);
    }
    visited.fill(false);

    let inverted = invert_graph(graph);
    let mut components = BTreeMap::new();
    let mut assigned = visited;

    for v in list {
        dfs_assign(&mut assigned, &mut components, &inverted, v);
    }

    components
}
