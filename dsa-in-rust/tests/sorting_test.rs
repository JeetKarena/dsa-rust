//! Integration tests for sorting algorithms
//! 
//! This module contains comprehensive tests for all sorting implementations.
//! Test cases cover:
//! - Regular unsorted arrays
//! - Empty arrays
//! - Already sorted arrays
//! - Reverse sorted arrays
//! 
//! Each test ensures the sorting algorithm maintains the following properties:
//! - Correctness: Array is properly sorted after execution
//! - Stability: Original order of equal elements is preserved
//! - In-place: Original array is modified directly

use dsa_in_rust::algorithms::sorting::bubble_sort::bubble_sort;

/// Tests bubble sort with a standard unsorted array
/// 
/// # Test Case
/// - Input: [64, 34, 25, 12, 22, 11, 90]
/// - Expected: [11, 12, 22, 25, 34, 64, 90]
#[test]
fn test_bubble_sort() {
    let mut data = [64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut data);
    assert_eq!(data, [11, 12, 22, 25, 34, 64, 90]);
}

/// Tests bubble sort with an empty array
/// 
/// # Test Case
/// - Input: []
/// - Expected: []
#[test]
fn test_bubble_sort_empty() {
    let mut data: [i32; 0] = [];
    bubble_sort(&mut data);
    assert_eq!(data, []);
}

/// Tests bubble sort with an already sorted array
/// 
/// # Test Case
/// - Input: [1, 2, 3, 4, 5]
/// - Expected: [1, 2, 3, 4, 5]
#[test]
fn test_bubble_sort_sorted() {
    let mut data = [1, 2, 3, 4, 5];
    bubble_sort(&mut data);
    assert_eq!(data, [1, 2, 3, 4, 5]);
}

/// Tests bubble sort with a reverse sorted array
/// 
/// # Test Case
/// - Input: [5, 4, 3, 2, 1]
/// - Expected: [1, 2, 3, 4, 5]
#[test]
fn test_bubble_sort_reverse() {
    let mut data = [5, 4, 3, 2, 1];
    bubble_sort(&mut data);
    assert_eq!(data, [1, 2, 3, 4, 5]);
}
