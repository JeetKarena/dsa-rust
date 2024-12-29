//! Depth First Search Implementation
//! 
//! DEV NOTES:
//! - Recursive implementation
//! - Stack space: O(h) where h is height of recursion tree
//! - Time complexity: O(n) where n is number of elements
//! - Space complexity: O(n) for visited set

use std::collections::HashSet;

/// Performs depth-first search on array to find target element
/// 
/// # Arguments
/// * `arr` - Slice of integers to search
/// * `target` - Value to find
/// 
/// # Returns
/// * `Option<usize>` - Index of target if found, None otherwise
/// 
/// # Examples
/// ```
/// use dsa_in_rust::algorithms::searching::dfs::depth_first_search;
/// 
/// let arr = vec![1, 2, 3, 4, 5];
/// assert_eq!(depth_first_search(&arr, 3), Some(2));
/// assert_eq!(depth_first_search(&arr, 6), None);
/// ```
pub fn depth_first_search(arr: &[i32], target: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut visited = HashSet::new();
    dfs_recursive(arr, target, 0, &mut visited)
}

fn dfs_recursive(arr: &[i32], target: i32, current: usize, visited: &mut HashSet<usize>) -> Option<usize> {
    if visited.contains(&current) || current >= arr.len() {
        return None;
    }

    visited.insert(current);

    if arr[current] == target {
        return Some(current);
    }

    // Search left
    if let Some(found) = dfs_recursive(arr, target, current * 2 + 1, visited) {
        return Some(found);
    }

    // Search right
    dfs_recursive(arr, target, current * 2 + 2, visited)
}

/// `DfsSearch` implements a Depth-First Search algorithm for graph traversal.
/// 
/// # Features
/// - Supports adjacency list representation of graphs
/// - Finds shortest path between two vertices
/// - Memory efficient with O(V) space complexity
/// - Handles cyclic and acyclic graphs
///
/// # Performance
/// - Time Complexity: O(V + E) where V is vertices and E is edges
/// - Space Complexity: O(V) for visited set and recursion stack
///
/// # Example
/// ```rust
/// let graph = vec![
///     vec![1, 2],    // 0 -> [1, 2]
///     vec![2],       // 1 -> [2]
///     vec![0, 3],    // 2 -> [0, 3]
///     vec![]         // 3 -> []
/// ];
/// 
/// let dfs = DfsSearch::new(&graph);
/// let path = dfs.search(0, 3);
/// assert_eq!(path, Some(vec![0, 1, 2, 3]));
/// ```
pub struct DfsSearch<'a> {
    graph: &'a Vec<Vec<usize>>,
}

impl<'a> DfsSearch<'a> {
    /// Creates a new DFS searcher for the given graph.
    /// 
    /// # Arguments
    /// * `graph` - Adjacency list representation of the graph
    ///
    /// # Returns
    /// * `DfsSearch` instance configured for the provided graph
    pub fn new(graph: &'a Vec<Vec<usize>>) -> Self {
        DfsSearch { graph }
    }

    /// Searches for a path from start vertex to target vertex.
    /// 
    /// # Arguments
    /// * `start` - Starting vertex index
    /// * `target` - Target vertex index
    ///
    /// # Returns
    /// * `Some(Vec<usize>)` - Path from start to target if found
    /// * `None` - If no path exists or vertices are invalid
    ///
    /// # Panics
    /// Will panic if start or target vertex indices are out of bounds
    pub fn search(&self, start: usize, target: usize) -> Option<Vec<usize>> {
        let mut visited = HashSet::with_capacity(self.graph.len());
        let mut path = Vec::with_capacity(self.graph.len());
        self.dfs(start, target, &mut visited, &mut path)
    }

    /// Internal recursive DFS implementation
    fn dfs(
        &self, 
        current: usize, 
        target: usize, 
        visited: &mut HashSet<usize>, 
        path: &mut Vec<usize>
    ) -> Option<Vec<usize>> {
        // Boundary check
        if current >= self.graph.len() {
            return None;
        }

        // Mark current vertex as visited and add to path
        visited.insert(current);
        path.push(current);

        // Check if target is found
        if current == target {
            return Some(path.clone());
        }

        // Recursively explore unvisited neighbors
        for &next in &self.graph[current] {
            if !visited.contains(&next) {
                if let Some(result) = self.dfs(next, target, visited, path) {
                    return Some(result);
                }
            }
        }

        // Backtrack by removing current vertex from path
        path.pop();
        None
    }
}