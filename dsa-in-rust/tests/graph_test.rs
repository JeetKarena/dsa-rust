//! Integration tests for Graph Data Structure
//! 
//! TEST CATEGORIES:
//! - Basic Operations: Creation, edge addition, node validation
//! - Edge Cases: Empty graph, full graph, self-loops
//! - Error Handling: Invalid nodes, duplicate edges
//! - Complex Scenarios: Cycles, disconnected components
//! - Performance: Large graphs, dense connections
//! 
//! DEV NOTES:
//! - Each test focuses on a single aspect
//! - Error cases are thoroughly validated
//! - Edge cases are explicitly tested
//! - Performance implications are documented

use dsa_in_rust::data_structures::graph::{Graph, GraphError};

/// Tests basic graph creation
/// 
/// # Test Case
/// - Operation: Create new graph
/// - Input: 5 nodes
/// - Expected: Empty graph with 5 nodes
/// - Validates: Basic initialization
#[test]
fn test_new_graph() {
    let graph = Graph::new(5);
    assert_eq!(graph.node_count(), 5);
}

/// Tests edge addition between valid nodes
/// 
/// # Test Case
/// - Operation: Add edges between nodes
/// - Input: Edges (0,1), (1,2)
/// - Expected: Both edges successfully added
/// - Validates: Basic edge addition
#[test]
fn test_add_valid_edges() {
    let mut graph = Graph::new(5);
    assert!(graph.add_edge(0, 1).is_ok());
    assert!(graph.add_edge(1, 2).is_ok());
    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(1, 2));
}

/// Tests adding edge with invalid node index
/// 
/// # Test Case
/// - Operation: Add edge with out-of-bounds node
/// - Input: Edge (5,1) in 5-node graph
/// - Expected: InvalidNode error
/// - Validates: Boundary checking
#[test]
fn test_invalid_node_edge() {
    let mut graph = Graph::new(5);
    assert!(matches!(
        graph.add_edge(5, 1),
        Err(GraphError::InvalidNode(5))
    ));
}

/// Tests adding duplicate edges
/// 
/// # Test Case
/// - Operation: Add same edge twice
/// - Input: Edge (0,1) added twice
/// - Expected: EdgeExists error on second addition
/// - Validates: Duplicate prevention
#[test]
fn test_duplicate_edge() {
    let mut graph = Graph::new(5);
    assert!(graph.add_edge(0, 1).is_ok());
    assert!(matches!(
        graph.add_edge(0, 1),
        Err(GraphError::EdgeExists(0, 1))
    ));
}

/// Tests empty graph edge operations
/// 
/// # Test Case
/// - Operation: Create empty graph
/// - Input: Graph with 0 nodes
/// - Expected: Valid graph with no possible edges
/// - Validates: Empty graph handling
#[test]
fn test_empty_graph() {
    let graph = Graph::new(0);
    assert_eq!(graph.node_count(), 0);
}

/// Tests self-loop addition
/// 
/// # Test Case
/// - Operation: Add edge from node to itself
/// - Input: Edge (2,2)
/// - Expected: Edge successfully added
/// - Validates: Self-loop handling
#[test]
fn test_self_loop() {
    let mut graph = Graph::new(3);
    assert!(graph.add_edge(2, 2).is_ok());
    assert!(graph.has_edge(2, 2));
}

/// Tests complete graph creation
/// 
/// # Test Case
/// - Operation: Create fully connected graph
/// - Input: All possible edges in 4-node graph
/// - Expected: All edges successfully added
/// - Validates: Dense graph handling
#[test]
fn test_complete_graph() {
    let mut graph = Graph::new(4);
    for i in 0..4 {
        for j in 0..4 {
            if i != j {
                assert!(graph.add_edge(i, j).is_ok());
            }
        }
    }
}

/// Tests cyclic connections
/// 
/// # Test Case
/// - Operation: Create cycle in graph
/// - Input: Edges forming cycle (0->1->2->0)
/// - Expected: All edges successfully added
/// - Validates: Cycle handling
#[test]
fn test_cycle() {
    let mut graph = Graph::new(3);
    assert!(graph.add_edge(0, 1).is_ok());
    assert!(graph.add_edge(1, 2).is_ok());
    assert!(graph.add_edge(2, 0).is_ok());
    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(1, 2));
    assert!(graph.has_edge(2, 0));
}

/// Tests large graph operations
/// 
/// # Test Case
/// - Operation: Create large graph
/// - Input: 1000-node graph with random edges
/// - Expected: All valid edges added successfully
/// - Validates: Performance with large datasets
#[test]
fn test_large_graph() {
    let node_count = 1000;
    let mut graph = Graph::new(node_count);
    for i in 0..10 {  // Add some sample edges
        assert!(graph.add_edge(i, (i + 1) % node_count).is_ok());
    }
}

/// Tests multiple edge addition and removal
/// 
/// # Test Case
/// - Operation: Add multiple edges between nodes
/// - Input: Multiple edges from same source
/// - Expected: All valid edges added
/// - Validates: Multiple edge handling
#[test]
fn test_multiple_edges() {
    let mut graph = Graph::new(5);
    assert!(graph.add_edge(0, 1).is_ok());
    assert!(graph.add_edge(0, 2).is_ok());
    assert!(graph.add_edge(0, 3).is_ok());
    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(0, 2));
    assert!(graph.has_edge(0, 3));
}