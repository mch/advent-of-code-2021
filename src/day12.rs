use petgraph::{Graph, Undirected};
use petgraph::algo::dijkstra::dijkstra;
use petgraph::algo::simple_paths::all_simple_paths;

pub fn puzzle() {
    // Load cave connectivity graph
    // Find all routes through the graph
    // Visit small caves only once
}

fn find_paths(graph: &Vec<Edge>, start: &str, end: &str) -> Vec<Path> {
    let mut paths: Vec<Path> = Vec::new();

    // Dijkstra's algorithm


    paths
}

struct Path {
}

struct Edge<'a> {
    node_a: &'a str,
    node_b: &'a str,
}

impl<'a> Edge<'a> {
    pub fn new(node_a: &'a str, node_b: &'a str) -> Edge<'a> {
        Edge{node_a, node_b}
    }
}

mod tests {
    use super::*;

    #[test]
    fn day12_no_path_through_empty_cave() {
        let mut graph = Vec::new();

        let paths = find_paths(&graph, &"start", &"end");

        assert_eq!(0, paths.len())
    }

    #[test]
    fn day12_cave_paths() {
        let mut graph = Vec::new();
        graph.push(Edge::new("start", "A"));
        graph.push(Edge::new("A", "end"));

        let paths = find_paths(&graph, &"start", &"end");

        assert_eq!(1, paths.len())
    }


    #[test]
    fn day12_test_petgraph() {
        let mut graph = Graph::<&str, ()>::new();
        let start_node = graph.add_node("start");
        let c_node = graph.add_node("c");
        let A_node = graph.add_node("A");
        let b_node = graph.add_node("b");
        let d_node = graph.add_node("d");
        let end_node = graph.add_node("end");
        graph.add_edge(start_node, A_node, ());
        graph.add_edge(start_node, b_node, ());
        graph.add_edge(A_node, c_node, ());
        graph.add_edge(A_node, b_node, ());
        graph.add_edge(b_node, d_node, ());
        graph.add_edge(A_node, end_node, ());
        graph.add_edge(b_node, end_node, ());
        let result = all_simple_paths::<Vec<_>, _>(&graph, start_node, end_node, 1, None);
        for item in result {
            println!("{:?}", item);
        }
    }
}
