//! Integration tests for Depth First Search Implementation
//! 
//! TEST CATEGORIES:
//! - Basic Path Finding: Simple paths between nodes
//! - Cycle Detection: Graphs with cycles
//! - Invalid Paths: Non-existent paths
//! - Empty Graphs: Edge cases with empty graphs
//! - Single Node: Minimal graph cases
//! - Disconnected Components: Multiple separate subgraphs
//! - Complex Paths: Multiple possible paths
//! 
//! DEV NOTES:
//! - Each test validates specific graph topology
//! - Edge cases are explicitly tested
//! - Path correctness is verified
//! - Performance implications documented

use dsa_in_rust::algorithms::searching::dfs_search::DfsSearch;

/// Tests basic path finding in a simple directed graph
/// 
/// # Test Case
/// - Graph: Linear path 0->1->2->3
/// - Search: 0 to 3
/// - Expected: Path [0,1,2,3]
#[test]
fn test_simple_path() {
    let graph = vec![
        vec![1],    // 0 -> 1
        vec![2],    // 1 -> 2
        vec![3],    // 2 -> 3
        vec![]      // 3 (end)
    ];
    let dfs = DfsSearch::new(&graph);
    assert_eq!(dfs.search(0, 3), Some(vec![0, 1, 2, 3]));
}

/// Tests path finding in a graph with cycles
/// 
/// # Test Case
/// - Graph: Cycle 0->1->2->0 with branch to 3
/// - Search: 0 to 3
/// - Expected: Path [0,1,2,3]
#[test]
fn test_cycle() {
    let graph = vec![
        vec![1],
        vec![2],
        vec![0, 3],
        vec![]
    ];
    let dfs = DfsSearch::new(&graph);
    assert_eq!(dfs.search(0, 3), Some(vec![0, 1, 2, 3]));
}

/// Tests search in empty graph
/// 
/// # Test Case
/// - Graph: Empty graph
/// - Search: Any nodes
/// - Expected: None
#[test]
fn test_empty_graph() {
    let graph: Vec<Vec<usize>> = vec![];
    let dfs = DfsSearch::new(&graph);
    assert_eq!(dfs.search(0, 1), None);
}

/// Tests single node graph
/// 
/// # Test Case
/// - Graph: Single node
/// - Search: Node to itself
/// - Expected: Path [0]
#[test]
fn test_single_node() {
    let graph = vec![vec![]];
    let dfs = DfsSearch::new(&graph);
    assert_eq!(dfs.search(0, 0), Some(vec![0]));
}

/// Tests disconnected components
/// 
/// # Test Case
/// - Graph: Two separate components
/// - Search: Between components
/// - Expected: None
#[test]
fn test_disconnected() {
    let graph = vec![
        vec![1],    // Component 1
        vec![0],
        vec![3],    // Component 2
        vec![2]
    ];
    let dfs = DfsSearch::new(&graph);
    assert_eq!(dfs.search(0, 2), None);
}

/// Tests complex path with multiple options
/// 
/// # Test Case
/// - Graph: Multiple paths to target
/// - Search: Start to target
/// - Expected: Any valid path
#[test]
fn test_multiple_paths() {
    let graph = vec![
        vec![1, 2],
        vec![3],
        vec![3],
        vec![]
    ];
    let dfs = DfsSearch::new(&graph);
    let result = dfs.search(0, 3);
    assert!(result == Some(vec![0, 1, 3]) || result == Some(vec![0, 2, 3]));
}

/// Tests self-loop in graph
/// 
/// # Test Case
/// - Graph: Node with self-loop
/// - Search: Through self-loop node
/// - Expected: Valid path avoiding cycle
#[test]
fn test_self_loop() {
    let graph = vec![
        vec![1],
        vec![1, 2],  // Self-loop at 1
        vec![]
    ];
    let dfs = DfsSearch::new(&graph);
    assert_eq!(dfs.search(0, 2), Some(vec![0, 1, 2]));
}

/// Tests invalid node indices
/// 
/// # Test Case
/// - Graph: Simple graph
/// - Search: Invalid node index
/// - Expected: None
#[test]
fn test_invalid_node() {
    let graph = vec![vec![1], vec![2], vec![]];
    let dfs = DfsSearch::new(&graph);
    assert_eq!(dfs.search(0, 5), None);
}

/// Tests dense graph
/// 
/// # Test Case
/// - Graph: Fully connected graph
/// - Search: Any two nodes
/// - Expected: Direct path
#[test]
fn test_dense_graph() {
    let graph = vec![
        vec![1, 2, 3],
        vec![0, 2, 3],
        vec![0, 1, 3],
        vec![0, 1, 2]
    ];
    let dfs = DfsSearch::new(&graph);
    assert_eq!(dfs.search(0, 3), Some(vec![0, 3]));
}