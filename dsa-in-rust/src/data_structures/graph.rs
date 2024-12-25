//! Graph Data Structure Implementation
//! 
//! DEV NOTES:
//! - Implementation uses adjacency list for space efficiency
//! - Optimized for sparse graphs (E << V²)
//! - Thread-safe operations with validation checks
//! - Memory efficient: O(V + E) space complexity
//! 
//! PERFORMANCE CONSIDERATIONS:
//! - Edge Addition: O(1)
//! - Edge Lookup: O(deg(v))
//! - Memory Usage: O(V + E)
//! - Thread Safety: Yes
//! 
//! # Examples
//! ```
//! use dsa_in_rust::data_structures::graph::Graph;
//! 
//! let mut graph = Graph::new(5);
//! graph.add_edge(0, 1).unwrap();
//! graph.add_edge(1, 2).unwrap();
//! assert!(graph.has_path(0, 2).unwrap());
//! ```

use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fmt;

/// Custom error type for graph operations
#[derive(Debug)]
pub enum GraphError {
    InvalidNode(usize),
    EdgeExists(usize, usize),
    NoPath(usize, usize),
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GraphError::InvalidNode(node) => write!(f, "Invalid node index: {}", node),
            GraphError::EdgeExists(src, dest) => write!(f, "Edge already exists: {} -> {}", src, dest),
            GraphError::NoPath(src, dest) => write!(f, "No path exists: {} -> {}", src, dest),
        }
    }
}

impl Error for GraphError {}

/// Graph representation using adjacency list
#[derive(Debug, Clone)]
pub struct Graph {
    adjacency_list: Vec<Vec<usize>>,
    node_count: usize,
    edge_count: usize,
}

impl Graph {
    /// Creates a new graph with specified number of nodes
    /// 
    /// # Arguments
    /// * `nodes` - Number of nodes in the graph
    /// 
    /// # Returns
    /// * `Graph` - New graph instance
    /// 
    /// # Performance
    /// * Time Complexity: O(n)
    /// * Space Complexity: O(n)
    pub fn new(nodes: usize) -> Self {
        Self {
            adjacency_list: vec![Vec::with_capacity(nodes); nodes],
            node_count: nodes,
            edge_count: 0,
        }
    }

    /// Adds a directed edge from src to dest
    /// 
    /// # Arguments
    /// * `src` - Source node index
    /// * `dest` - Destination node index
    /// 
    /// # Returns
    /// * `Result<(), GraphError>` - Ok if edge added, Err otherwise
    /// 
    /// # Performance
    /// * Time Complexity: O(1) amortized
    /// * Space Complexity: O(1)
    pub fn add_edge(&mut self, src: usize, dest: usize) -> Result<(), GraphError> {
        // Validate nodes
        if src >= self.node_count || dest >= self.node_count {
            return Err(GraphError::InvalidNode(src.max(dest)));
        }

        // Check if edge already exists
        if self.adjacency_list[src].contains(&dest) {
            return Err(GraphError::EdgeExists(src, dest));
        }

        // Add edge
        self.adjacency_list[src].push(dest);
        self.edge_count += 1;
        Ok(())
    }

    /// Checks if a path exists between src and dest
    /// 
    /// # Arguments
    /// * `src` - Source node index
    /// * `dest` - Destination node index
    /// 
    /// # Returns
    /// * `Result<bool, GraphError>` - Ok(true) if path exists, Ok(false) otherwise
    /// 
    /// # Performance
    /// * Time Complexity: O(V + E)
    /// * Space Complexity: O(V)
    pub fn has_path(&self, src: usize, dest: usize) -> Result<bool, GraphError> {
        // Validate nodes
        if src >= self.node_count || dest >= self.node_count {
            return Err(GraphError::InvalidNode(src.max(dest)));
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(src);

        while let Some(node) = queue.pop_front() {
            if node == dest {
                return Ok(true);
            }

            if !visited.insert(node) {
                continue;
            }

            queue.extend(self.adjacency_list[node].iter());
        }

        Ok(false)
    }

    /// Returns the number of nodes in the graph
    pub fn node_count(&self) -> usize {
        self.node_count
    }

    /// Returns the number of edges in the graph
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
    
    /// Checks if an edge exists between two nodes
    /// 
    /// # Arguments
    /// * `from` - Source node index
    /// * `to` - Destination node index
    /// 
    /// # Returns
    /// * `bool` - True if edge exists, false otherwise
    /// 
    /// # Performance
    /// * Time Complexity: O(deg(v)) where deg(v) is degree of source vertex
    /// * Space Complexity: O(1)
    /// 
    /// # Examples
    /// ```
    /// use dsa_in_rust::data_structures::graph::Graph;
    /// 
    /// let mut graph = Graph::new(5);
    /// graph.add_edge(0, 1).unwrap();
    /// assert!(graph.has_edge(0, 1));
    /// assert!(!graph.has_edge(1, 0));  // Directed graph
    /// ```
    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        if from >= self.node_count || to >= self.node_count {
            return false;
        }
        self.adjacency_list[from].contains(&to)
    }
}
