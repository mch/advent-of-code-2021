use petgraph::{Graph, Undirected};
use petgraph::visit::{IntoEdges, Visitable};
use std::hash::Hash;
use petgraph::algo::dijkstra::dijkstra;
use petgraph::algo::simple_paths::all_simple_paths;
use std::collections::HashSet;

pub fn puzzle() {
    // Load cave connectivity graph
    // Find all routes through the graph
    // Visit small caves only once
}

fn find_paths(graph: &Vec<Edge>, start: &str, end: &str) -> Vec<Path> {
    let mut paths: Vec<Path> = Vec::new();

    // Find the number of paths distinct paths from 'start' to 'end
    // Don't visit small caves more than once.
    // Big caves are upper case
    // Small caves are lowercase

    // Keep a list of paths
    // The current path

    // Save list of nodes to visit next
    // Keep track of visited nodes
    // for each adjacent node {
    // if small, skip
    // add connected nodes to list of nodes to visit next
    //


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

    #[test]
    fn day12_test_two_connected_caves() {
        let mut graph = Vec::new();
        graph.push(("start", "end"));
        let num_paths = number_of_distinct_paths(&graph, String::from("start"), String::from("end"));
        assert_eq!(1, num_paths);
    }

    #[test]
    fn day12_test_diamond_shape_caves() {
        let mut graph = Vec::new();
        graph.push(("start", "a"));
        graph.push(("a", "end"));

        graph.push(("start", "b"));
        graph.push(("b", "end"));
        let num_paths = number_of_distinct_paths(&graph, String::from("start"), String::from("end"));
        assert_eq!(2, num_paths);
    }

    //   s---
    //  / \  \
    // a   b  c
    //  \ /  /
    //   e---
    //
    // s a e
    // s (already visited a) b e
    //
    //

    //   s
    //  / \
    // a   b--c
    //  \ /
    //   e
    //
    // Paths:
    // s,a,e
    // s,b,e
    // s,b,c,b,e

}

fn number_of_distinct_paths(graph: &Vec::<(&str, &str)>, start: String, end: String) -> usize
{
    let mut final_list_of_paths: Vec<Vec<String>> = Vec::new();

    let mut paths: Vec<Vec<String>> = Vec::new();
    let mut current_path: Vec<String> = Vec::new();

    let mut unvisited_nodes = Vec::new();
    let mut discovered_nodes = Vec::new();

    unvisited_nodes.push(start);
    //add_single_path_single_node(start);
    //current_path.push(start);
    while unvisited_nodes.len() > 0 {
        let discovered_node = unvisited_nodes.pop().unwrap();
        if !discovered_nodes.contains(&discovered_node) {
            discovered_nodes.push(discovered_node.clone());
            //add_this_discovered_node_to_all_of_the_paths
            current_path.push(discovered_node.clone());
            if discovered_node == end {
                // copy all paths to final list of paths
                paths.push(current_path);
                current_path
            }
            let adj_nodes = adjacent_edges(&graph, discovered_node);
            for node in adj_nodes {
                unvisited_nodes.push(node)
            }
        }
    }

    final_list_of_paths.len()
}

fn adjacent_edges(graph: &Vec::<(&str, &str)>, node: String) -> HashSet<String> {
    let mut adj_edges = HashSet::new();
    for (node_a, node_b) in graph {
        if *node_a == node {
            adj_edges.insert(String::from(*node_b));
        }
        if *node_b == node {
            adj_edges.insert(String::from(*node_a));
        }
    }
    adj_edges
}
