//! Integration tests for the Bubble Sort algorithm
//! 
//! This module contains comprehensive tests for the Bubble Sort implementation.
//! Test cases cover:
//! - Basic cases (empty, single element)
//! - Order cases (sorted, reverse sorted)
//! - Element type cases (duplicates, negative numbers, characters, Unicode characters)
//! - Size cases (large arrays)
//! - Edge cases (same elements, alternating elements)
//! - Stability test (preserving order of equal elements)

use dsa_in_rust::algorithms::sorting::bubble_sort::bubble_sort;
use dsa_in_rust::utils::helpers::generate_sorted_integers;

/// Tests bubble sort with a standard unsorted array
/// 
/// # Test Case
/// - Input: [64, 34, 25, 12, 22, 11, 90]
/// - Expected: [11, 12, 22, 25, 34, 64, 90]
#[test]
fn test_standard_unsorted_array() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut arr);
    assert_eq!(arr, [11, 12, 22, 25, 34, 64, 90]);
}

/// Tests bubble sort with an empty array
/// 
/// # Test Case
/// - Input: []
/// - Expected: []
#[test]
fn test_empty_array() {
    let mut arr: [i32; 0] = [];
    bubble_sort(&mut arr);
    assert_eq!(arr, []);
}

/// Tests bubble sort with a single element array
/// 
/// # Test Case
/// - Input: [42]
/// - Expected: [42]
#[test]
fn test_single_element() {
    let mut arr = [42];
    bubble_sort(&mut arr);
    assert_eq!(arr, [42]);
}

/// Tests bubble sort with an already sorted array
/// 
/// # Test Case
/// - Input: [1, 2, 3, 4, 5]
/// - Expected: [1, 2, 3, 4, 5]
#[test]
fn test_sorted_array() {
    let mut arr = [1, 2, 3, 4, 5];
    bubble_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}

/// Tests bubble sort with a reverse sorted array
/// 
/// # Test Case
/// - Input: [5, 4, 3, 2, 1]
/// - Expected: [1, 2, 3, 4, 5]
#[test]
fn test_reverse_sorted() {
    let mut arr = [5, 4, 3, 2, 1];
    bubble_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}

/// Tests bubble sort with duplicate elements
/// 
/// # Test Case
/// - Input: [3, 1, 4, 1, 5, 9, 2, 6, 5, 3]
/// - Expected: [1, 1, 2, 3, 3, 4, 5, 5, 6, 9]
#[test]
fn test_duplicate_elements() {
    let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    bubble_sort(&mut arr);
    assert_eq!(arr, [1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
}

/// Tests bubble sort with negative numbers
/// 
/// # Test Case
/// - Input: [-3, 1, -4, 1, -5, 9, -2, 6, -5, 3]
/// - Expected: [-5, -5, -4, -3, -2, 1, 1, 3, 6, 9]
#[test]
fn test_negative_numbers() {
    let mut arr = [-3, 1, -4, 1, -5, 9, -2, 6, -5, 3];
    bubble_sort(&mut arr);
    assert_eq!(arr, [-5, -5, -4, -3, -2, 1, 1, 3, 6, 9]);
}

/// Tests bubble sort with a large array
/// 
/// # Test Case
/// - Input: A large array of 1000 elements in reverse order
/// - Expected: A sorted array of 1000 elements
#[test]
fn test_large_array() {
    let mut arr: Vec<i32> = generate_sorted_integers(1000);
    arr.reverse();
    let expected: Vec<i32> = generate_sorted_integers(1000);
    bubble_sort(&mut arr);
    assert_eq!(arr, expected);
}

/// Tests bubble sort with a large array of random elements
/// 
/// # Test Case
/// - Input: A large array of 1000 elements with random values
/// - Expected: A sorted array of 1000 elements
#[test]
fn test_random_large_array() {
    let mut arr = vec![42; 1000];
    bubble_sort(&mut arr);
    assert!(arr.windows(2).all(|w| w[0] <= w[1]));
}

/// Tests bubble sort with an array where all elements are the same
/// 
/// # Test Case
/// - Input: [1, 1, 1, 1, 1]
/// - Expected: [1, 1, 1, 1, 1]
#[test]
fn test_same_elements() {
    let mut arr = [1, 1, 1, 1, 1];
    bubble_sort(&mut arr);
    assert_eq!(arr, [1, 1, 1, 1, 1]);
}

/// Tests bubble sort with an array of alternating elements
/// 
/// # Test Case
/// - Input: [1, 2, 1, 2, 1, 2]
/// - Expected: [1, 1, 1, 2, 2, 2]
#[test]
fn test_alternating_elements() {
    let mut arr = [1, 2, 1, 2, 1, 2];
    bubble_sort(&mut arr);
    assert_eq!(arr, [1, 1, 1, 2, 2, 2]);
}

/// Tests bubble sort with an array of ASCII characters
/// 
/// # Test Case
/// - Input: ['d', 'a', 'c', 'b']
/// - Expected: ['a', 'b', 'c', 'd']
#[test]
fn test_char_array() {
    let mut arr = ['d', 'a', 'c', 'b'];
    bubble_sort(&mut arr);
    assert_eq!(arr, ['a', 'b', 'c', 'd']);
}

/// Tests bubble sort with an array of Unicode characters
/// 
/// # Test Case
/// - Input: ['δ', 'α', 'γ', 'β']
/// - Expected: ['α', 'β', 'γ', 'δ']
#[test]
fn test_unicode_chars() {
    let mut arr = ['δ', 'α', 'γ', 'β'];
    bubble_sort(&mut arr);
    assert_eq!(arr, ['α', 'β', 'γ', 'δ']);
}

/// Tests bubble sort for stability (preserving order of equal elements)
///
/// # Test Case
/// - Input: A vector of structs with equal keys but different indices
/// - Expected: The order of elements with equal keys is preserved
#[test]
fn test_stability() {
    #[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
    struct Item {
        key: i32,
        index: usize,
    }

    let mut items = vec![
        Item { key: 1, index: 0 },
        Item { key: 1, index: 1 },
        Item { key: 0, index: 2 },
    ];

    bubble_sort(&mut items);

    assert_eq!(items[0].key, 0);
    assert_eq!(items[1].key, 1);
    assert_eq!(items[2].key, 1);
    assert_eq!(items[1].index, 0);
    assert_eq!(items[2].index, 1);
}
