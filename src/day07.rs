use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{Bfs, DfsPostOrder, EdgeRef};

/// Used to help build our graph of Bags piece by piece.
/// It stores the index each color String is stored at in the Graph
/// and implements helper methods
pub struct BagGraph {
    graph: Graph<String, usize>,
    node_map: HashMap<String, NodeIndex>,
}

impl BagGraph {
    pub fn new() -> Self {
        BagGraph {
            graph: Graph::new(),
            node_map: HashMap::new(),
        }
    }

    /// return a reference to the graph
    pub fn graph(&self) -> &Graph<String, usize> {
        &self.graph
    }

    /// Makes no change if the bag is already in the graph
    /// Adds a bag to the graph and records its index
    /// Always returns the NodeIndex of the bag
    pub fn add_bag(&mut self, color: String) -> NodeIndex {
        match self.lookup_node_index(&color) {
            Some(idx) => idx,
            None => {
                let idx = self.graph.add_node(color.clone());
                self.node_map.insert(color, idx);
                idx
            }
        }
    }

    /// Adds an edge to the graph between bags color and contained_color
    /// Adds nodes for the bags as necessary
    /// The edges are directed from bag down to its contents
    pub fn contains_bag(&mut self, color: String, number: usize, contained_color: String) {
        let a = self.add_bag(color);
        let b = self.add_bag(contained_color);
        self.graph.add_edge(a, b, number);
    }

    fn lookup_node_index(&self, color: &str) -> Option<NodeIndex> {
        self.node_map.get(color).cloned()
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> BagGraph {
    let mut bag_graph = BagGraph::new();

    input.lines().for_each(|line| {
        let mut iter = line.split("contain ");
        let color = iter
            .next()
            .unwrap()
            .split(' ')
            .take(2)
            .collect::<Vec<&str>>()
            .join(" ");
        let contains = iter.next().unwrap();
        if contains == "no other bags." {
            // a bag that contains no other bags
            bag_graph.add_bag(color.to_string());
        } else {
            // a bag containing other bags
            contains.split(", ").for_each(|string| {
                let mut contents = string.split(' ');
                let count = contents.next().unwrap().parse::<usize>().unwrap();
                let contained_color = contents.take(2).collect::<Vec<&str>>().join(" ");
                bag_graph.contains_bag(color.to_string(), count, contained_color.to_string());
            });
        };
    });

    bag_graph
}

#[aoc(day7, part1)]
pub fn part1(graph: &BagGraph) -> usize {
    let idx = graph.lookup_node_index("shiny gold").unwrap();
    let graph_ref = &graph.graph();
    let mut bfs = Bfs::new(graph_ref, idx);
    let mut count = 0;
    while let Some(_idx) = bfs.next(graph_ref) {
        count += 1;
    }
    // adjusted to not include shiny gold bags themselves!
    count - 1
}

#[aoc(day7, part2)]
pub fn part2(graph: &BagGraph) -> usize {
    let idx = graph.lookup_node_index("shiny gold").unwrap();
    let graph_ref = graph.graph();

    // we need post order, i.e. bottom to top.
    // to calculate the count of a bag, we need know the count of all of the bags inside it first
    let mut dfs = DfsPostOrder::new(graph_ref, idx);

    // store the counts of each bag as we traverse the graph
    let mut counts: HashMap<NodeIndex, usize> = HashMap::new();

    // Depth first search where we need to track all the weights as we come up and multiply them together
    // each bag will add its "count" plus its count multiplied by its total contents
    while let Some(idx) = dfs.next(graph_ref) {
        for e in graph_ref.edges_directed(idx, petgraph::Direction::Outgoing) {
            //     outer (source)
            //    /
            //   / e
            //  /
            // inner (target)
            let count = counts.entry(e.source()).or_insert(0);
            *count += e.weight();

            // dbg!(&graph.graph()[e.source()]);
            // dbg!(&graph.graph()[e.target()]);

            if let Some(multiplier) = counts.get(&e.target()).cloned() {
                let count = counts.entry(e.source()).or_insert(0);
                *count += e.weight() * multiplier;
            }

            // dbg!(&counts
                // .iter()
                // .map(|(idx, count)| (graph.graph()[*idx].clone(), count))
                // .collect::<HashMap<_, _>>());
        }
    }
    *counts.get(&idx).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_1: &'static str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const TEST_INPUT_2: &'static str = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

const d: &'static str = "\
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";


    #[test]
    fn part1_works() {
        assert_eq!(4, part1(&input_generator(TEST_INPUT_1)));
    }

    #[test]
    fn part2_works() {
        assert_eq!(126, part2(&input_generator(TEST_INPUT_2)));
        
        assert_eq!(32, part2(&input_generator(TEST_INPUT_1)));
    }
}
