use tabbycat::*;

pub fn search_tree(edges: &[(Option<usize>, usize)], n: usize) -> String {
    let mut stmt_list = StmtList::new();

    for v in 1..=n {
        stmt_list = stmt_list.add_node(Identity::from(v), None, None);
    }

    let edges = edges.iter().filter_map(|(p, v)| p.map(|p| (p,v)));

    for (n, (p, u)) in edges.enumerate() {
        let e = Edge::head_node(Identity::from(p), None)
            .arrow_to_node(Identity::from(*u), None)
            .add_attribute(Identity::id("label").unwrap(), Identity::from(n));
        stmt_list = stmt_list.add_edge(e);
    }

    let dot_graph = GraphBuilder::default()
        .graph_type(GraphType::DiGraph)
        .strict(false)
        .id(Identity::id("g").unwrap())
        .stmts(stmt_list)
        .build()
        .unwrap();

    dot_graph.to_string()
}
