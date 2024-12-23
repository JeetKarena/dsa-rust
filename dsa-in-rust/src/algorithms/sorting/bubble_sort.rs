// src/algorithms/sorting/bubble_sort.rs

/// Optimized Bubble Sort implementation for generic types
///
/// # Type Parameters
/// * `T` - Type that implements Ord trait
///
/// # Arguments
/// * `arr` - A mutable slice of type T to be sorted
///
/// # Example
///
/// ```
/// use dsa_in_rust::algorithms::sorting::bubble_sort::bubble_sort;
///
/// let mut data = [64, 34, 25, 12, 22, 11, 90];
/// bubble_sort(&mut data);
/// assert_eq!(data, [11, 12, 22, 25, 34, 64, 90]);
/// ```
///
/// # Performance
/// - Time Complexity: O(n²) worst/average case, O(n) best case
/// - Space Complexity: O(1)
/// - Stable: Yes
/// - Adaptive: Yes (optimized for nearly sorted arrays)
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let mut n = arr.len();
    let mut new_n;

    loop {
        new_n = 0;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_n = i;
            }
        }
        if new_n == 0 {
            break;
        }
        n = new_n;
    }
}
