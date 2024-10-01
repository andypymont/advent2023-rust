use std::collections::BTreeMap;
use std::str::FromStr;

advent_of_code::solution!(25);

#[derive(Debug, PartialEq)]
struct Graph {
    nodes: Vec<Vec<bool>>,
}

impl Graph {
    fn group_sizes_with_three_removed(&self) -> (usize, usize) {
        let mut retained = Vec::new();
        for _node in &self.nodes {
            retained.push(true);
        }

        loop {
            let external_neighbours: Vec<usize> = retained
                .iter()
                .enumerate()
                .map(|(i, kept)| {
                    self.nodes[i]
                        .iter()
                        .enumerate()
                        .map(|(j, neighbour)| usize::from(*kept && *neighbour && !retained[j]))
                        .sum()
                })
                .collect();
            let total: usize = external_neighbours.iter().sum();
            if total == 3 {
                break;
            }

            let (candidate, _max) = external_neighbours
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.cmp(b.1))
                .unwrap_or((0, &0));
            retained[candidate] = false;
        }

        let retained_count = retained.iter().map(|r| usize::from(*r)).sum();
        (retained_count, self.nodes.len() - retained_count)
    }
}

#[derive(Debug, PartialEq)]
struct ParseGraphError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct NodeName([char; 3]);

impl FromStr for NodeName {
    type Err = ParseGraphError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let mut a: Result<char, Self::Err> = Err(ParseGraphError);
        let mut b: Result<char, Self::Err> = Err(ParseGraphError);
        let mut c: Result<char, Self::Err> = Err(ParseGraphError);

        for (ix, ch) in name.chars().enumerate() {
            match ix {
                0 => a = Ok(ch),
                1 => b = Ok(ch),
                2 => c = Ok(ch),
                _ => return Err(ParseGraphError),
            }
        }

        let a = a?;
        let b = b?;
        let c = c?;
        Ok(Self([a, b, c]))
    }
}

#[derive(Debug, PartialEq)]
struct GraphBuilder {
    names: BTreeMap<NodeName, usize>,
    nodes: Vec<Vec<bool>>,
}

impl GraphBuilder {
    fn new() -> Self {
        let names = BTreeMap::new();
        let nodes = Vec::new();
        Self { names, nodes }
    }

    fn insert_node(&mut self, name: NodeName) -> usize {
        // assign latest index
        let ix = self.names.len();

        // insert the name into the name mapping
        self.names.insert(name, ix);

        // create the new node and extend all existing nodes
        let mut new_node = Vec::new();
        new_node.push(false);
        self.nodes.iter_mut().for_each(|node| {
            node.push(false);
            new_node.push(false);
        });
        self.nodes.push(new_node);

        // return the index of the newly-created node
        ix
    }

    fn node_ix(&mut self, name: NodeName) -> usize {
        match self.names.get(&name) {
            Some(existing) => *existing,
            None => self.insert_node(name),
        }
    }

    fn connect(&mut self, first: NodeName, second: NodeName) {
        let i = self.node_ix(first);
        let j = self.node_ix(second);
        self.nodes[i][j] = true;
        self.nodes[j][i] = true;
    }
}

impl FromStr for Graph {
    type Err = ParseGraphError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut builder = GraphBuilder::new();

        for line in text.lines() {
            let Some((first, others)) = line.split_once(": ") else {
                return Err(ParseGraphError);
            };

            let first = first.parse()?;

            for other in others.split_whitespace() {
                let other = other.parse()?;
                builder.connect(first, other);
            }
        }

        Ok(Self {
            nodes: builder.nodes,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    if let Ok(graph) = input.parse::<Graph>() {
        let (a, b) = graph.group_sizes_with_three_removed();
        Some(a * b)
    } else {
        None
    }
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_graph() -> Graph {
        Graph {
            nodes: vec![
                vec![
                    false, true, true, true, false, false, false, false, false, false, false,
                    false, false, true, false,
                ], // jqt
                vec![
                    true, false, true, false, false, false, false, false, true, false, false,
                    false, true, false, false,
                ], // rhn
                vec![
                    true, true, false, false, false, false, false, false, true, false, false,
                    false, true, true, false,
                ], // xhk
                vec![
                    true, false, false, false, false, false, true, false, false, true, true, true,
                    false, false, false,
                ], // nvd
                vec![
                    false, false, false, false, false, true, true, true, false, false, false,
                    false, false, false, true,
                ], // rsh
                vec![
                    false, false, false, false, true, false, false, true, false, false, true, true,
                    false, false, false,
                ], // frs
                vec![
                    false, false, false, true, true, false, false, true, true, false, false, false,
                    false, false, false,
                ], // pzl
                vec![
                    false, false, false, false, true, true, true, false, false, false, false, true,
                    false, false, true,
                ], // lsr
                vec![
                    false, true, true, false, false, false, true, false, false, false, false,
                    false, true, true, false,
                ], // hfx
                vec![
                    false, false, false, true, false, false, false, false, false, false, true,
                    true, true, false, true,
                ], // cmg
                vec![
                    false, false, false, true, false, true, false, false, false, true, false,
                    false, false, false, true,
                ], // qnr
                vec![
                    false, false, false, true, false, true, false, true, false, true, false, false,
                    false, false, false,
                ], // lhk
                vec![
                    false, true, true, false, false, false, false, false, true, true, false, false,
                    false, true, false,
                ], // bvb
                vec![
                    true, false, true, false, false, false, false, false, true, false, false,
                    false, true, false, false,
                ], // ntq
                vec![
                    false, false, false, false, true, false, false, true, false, true, true, false,
                    false, false, false,
                ], // rzs
            ],
        }
    }

    #[test]
    fn test_parse_graph() {
        assert_eq!(
            advent_of_code::template::read_file("examples", DAY).parse(),
            Ok(example_graph())
        );
    }

    #[test]
    fn test_group_sizes_with_three_removed() {
        assert_eq!(example_graph().group_sizes_with_three_removed(), (6, 9),);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }
}
