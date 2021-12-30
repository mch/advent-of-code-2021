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

    // #[test]
    // fn day12_no_path_through_empty_cave() {
    //     let mut graph = Vec::new();

    //     let paths = find_paths(&graph, &"start", &"end");

    //     assert_eq!(0, paths.len())
    // }

    // #[test]
    // fn day12_cave_paths() {
    //     let mut graph = Vec::new();
    //     graph.push(Edge::new("start", "A"));
    //     graph.push(Edge::new("A", "end"));

    //     let paths = find_paths(&graph, &"start", &"end");

    //     assert_eq!(1, paths.len())
    // }


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
        let paths = find_paths(&graph, String::from("start"), String::from("end"));
        assert_eq!(1, paths.len());
        assert_eq!(vec!["start", "end"], paths[0]);
    }

    #[test]
    fn day12_test_diamond_shape_caves() {
        //  start
        // /    \
        // a     b
        // \    /
        //   end
        // paths: [start, a], [start, b]

        let mut graph = Vec::new();
        graph.push(("start", "a"));
        graph.push(("a", "end"));

        graph.push(("start", "b"));
        graph.push(("b", "end"));
        let paths = find_paths(&graph, String::from("start"), String::from("end"));
        assert_eq!(2, paths.len());
        assert!(paths.contains(&vec![String::from("start"), String::from("a"), String::from("end")]));
        assert!(paths.contains(&vec![String::from("start"), String::from("b"), String::from("end")]));
        // rust retro: how can we avoid String::from everywhere
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
    // | \    \ /
    // d  f   g
    //  \ /  /
    //   e
    // Paths:
    // s,a,e
    // s,b,e
    // s,b,c,b,e

}

fn find_paths(graph: &Vec::<(&str, &str)>, start: String, end: String) -> Vec<Vec<String>>
{
    let mut final_list_of_paths: Vec<Vec<String>> = Vec::new();

    let mut paths: Vec<Vec<String>> = Vec::new();

    let mut unvisited_nodes = Vec::new();
    let mut discovered_nodes = Vec::new();

    unvisited_nodes.push((start.clone(), start.clone()));
    paths.push(vec![start.clone()]);

    println!("paths: {:?}", paths);
    println!("unvisited_nodes: {:?}", unvisited_nodes);
    println!("discovered_node: {:?}", discovered_nodes);

    while unvisited_nodes.len() > 0 {
        let (previous_node, discovered_node) = unvisited_nodes.pop().unwrap();
        if !discovered_nodes.contains(&discovered_node) {
            discovered_nodes.push(discovered_node.clone());

            for path in paths.iter_mut() {
                if path[path.len() - 1] == previous_node {
                    path.push(discovered_node.clone());
                }
            }

            if discovered_node == end {
                // if current_path has start and end, and find path inside paths that is the same as
                // current path, remove current path from paths and assign current path to be the
                // next path in the path list
                let mut new_paths: Vec<Vec<String>> = Vec::new();
                for path in paths.iter() {
                    if !(path[0] == start && path[path.len() - 1] == end) {
                        new_paths.push(path.clone());
                    } else {
                        final_list_of_paths.push(path.clone());
                    }
                }
                paths = new_paths;
            }
            let adj_nodes = adjacent_edges(&graph, discovered_node.clone());
            for adj_node in adj_nodes {
                unvisited_nodes.push((discovered_node.clone(), adj_node.clone()));
                // find paths that end with discovered node and duplicate those paths for every
                // adjacent node.
                let mut new_paths: Vec<Vec<String>> = Vec::new();
                for path in paths.iter() {
                    if path[path.len() - 1] == discovered_node {
                        let mut new_path = path.clone();
                        new_path.push(adj_node.clone());
                        new_paths.push(new_path);
                    } else {
                        new_paths.push(path.clone());
                    }
                }
                paths = new_paths;
            }
        }
        println!("paths: {:?}", paths);
        println!("unvisited_nodes: {:?}", unvisited_nodes);
        println!("discovered_node: {:?}", discovered_nodes);
    }

    // Start
    // /   \
    // a    b
    // |    |
    // c    |
    // \    /
    //  end

    // Start
    // /
    // a
    // |
    // c----b
    // \    /
    //  end

    // "start", "A", "B", "C", "end"
    // iter 1 -> ["start"]
    // iter 2 -> ["start", "A"], ["start", "B"]
    // iter 3 -> ["start", "A", "C"], ["start", "B"]
    // iter 4 -> ["start", "A", "C", "end"], ["start", "B"]
    // iter 5 -> ["start", "B", "end"]

    // current path
    // "start"
    // "start" "a"
    // "start" "c"
    // "start" "c" "end"
    // current path added to paths
    // backtrack along current path to find branch



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

    final_list_of_paths
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
