use std::collections::HashSet;

struct Graph {
    adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with a given number of nodes
    fn new(nodes: usize) -> Self {
        Self {
            adjacency_list: vec![vec![]; nodes],
        }
    }

    // Add an edge from `src` to `dest`
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adjacency_list[src].push(dest);
    }
}
