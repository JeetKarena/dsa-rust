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