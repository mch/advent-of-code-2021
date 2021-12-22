use petgraph::Graph;

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
        let mut graph = Graph::<&str, &str>::new();

    }
}
