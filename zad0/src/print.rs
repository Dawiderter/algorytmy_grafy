use crate::GraphList;
use tabbycat::*;

pub trait PrintDot {
    fn to_dot(&self) -> String;
}

impl PrintDot for GraphList {
    fn to_dot(&self) -> String {
        let mut stmt_list = StmtList::new();

        for v in 1..=(self.n) {
            stmt_list = stmt_list.add_node(Identity::from(v), None, None);
        }

        let undir = |(v, u)| v < u;
        let dir = |(_, _)| true;

        let h: &dyn Fn((usize, usize)) -> bool = match self.gtype {
            crate::GraphType::Directed => &dir,
            crate::GraphType::Undirected => &undir,
        };

        let edges: Vec<_> = self
            .list
            .iter()
            .enumerate()
            .flat_map(|(v, vec)| {
                vec.iter()
                    .filter_map(move |&u| Some((v, u)).filter(|&e| h(e)))
            })
            .collect();

        for (v, u) in edges {
            let mut e = Edge::head_node(Identity::from(v), None);
            match self.gtype {
                crate::GraphType::Directed => e = e.arrow_to_node(Identity::from(u), None),
                crate::GraphType::Undirected => e = e.line_to_node(Identity::from(u), None),
            }
            stmt_list = stmt_list.add_edge(e);
        }

        let gtype = match self.gtype {
            crate::GraphType::Directed => GraphType::DiGraph,
            crate::GraphType::Undirected => GraphType::Graph,
        };

        let dot_graph = GraphBuilder::default()
            .graph_type(gtype)
            .strict(false)
            .id(Identity::id("g").unwrap())
            .stmts(stmt_list)
            .build()
            .unwrap();

        dot_graph.to_string()
    }
}
