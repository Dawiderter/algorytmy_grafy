use std::{io::BufRead, num::ParseIntError};

use crate::{GraphList, GraphType};

pub fn read(reader: &mut dyn BufRead) -> GraphList {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    parse(&buffer).unwrap()
}

pub fn parse(text: &str) -> Result<GraphList, GraphParseError> {
    let mut lines = text.lines().enumerate();

    let flag : GraphType = lines.next().and_parse()?;

    let n : usize = lines.next().and_parse()?;

    let _m: usize = lines.next().and_parse()?;

    let mut g = GraphList::new(n, flag);

    for line in lines {
        let (v, u) = line.and_parse()?;
        g.add_edge(v, u);
    }

    Ok(g)
}

#[derive(Debug)]
pub enum GraphParseError {
    AtEnd,
    AtLine(usize),
}

trait GraphParse<T> {
    fn and_parse(&self) -> Result<T, GraphParseError>;
}

impl GraphParse<usize> for Option<(usize, &str)> {
    fn and_parse(&self) -> Result<usize, GraphParseError> {
        let (line_number, text) = self.ok_or(GraphParseError::AtEnd)?;
        text.parse::<usize>()
            .map_err(|_| GraphParseError::AtLine(line_number))
    }
}

impl GraphParse<GraphType> for Option<(usize, &str)> {
    fn and_parse(&self) -> Result<GraphType, GraphParseError> {
        let (line_number, text) = self.ok_or(GraphParseError::AtEnd)?;
        match text.trim() {
            "D" => Ok(GraphType::Directed),
            "U" => Ok(GraphType::Undirected),
            _ => Err(GraphParseError::AtLine(line_number)),
        }
    }
}

impl GraphParse<(usize, usize)> for (usize, &str) {
    fn and_parse(&self) -> Result<(usize, usize), GraphParseError> {
        let line_number = self.0;
        let line = self.1;

        let split: Result<Vec<usize>, ParseIntError> = line
            .split_whitespace()
            .map(|t| t.parse::<usize>())
            .collect();

        let split = split.map_err(|_| GraphParseError::AtLine(line_number))?;
        if split.len() == 2 {
            Ok((split[0], split[1]))
        } else {
            Err(GraphParseError::AtLine(line_number))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_test() {
        let mut test_graph = GraphList::new(2, GraphType::Undirected);
        test_graph.add_undirected(1, 2);

        let test_text = "U\n2\n1\n1 2\n";

        let graph = parse(test_text).unwrap();

        assert_eq!(graph, test_graph);
    }
}
