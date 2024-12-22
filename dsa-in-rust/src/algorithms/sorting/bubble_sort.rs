// src/algorithms/sorting/bubble_sort.rs

/// Optimized Bubble Sort implementation
///
/// This function sorts an array of integers in ascending order using the Bubble Sort algorithm.
/// The optimization includes an early exit if the array is already sorted.
///
/// # Arguments
///
/// * `arr` - A mutable slice of integers to be sorted.
///
/// # Example
///
/// ```
/// let mut data = [64, 34, 25, 12, 22, 11, 90];
/// bubble_sort(&mut data);
/// assert_eq!(data, [11, 12, 22, 25, 34, 64, 90]);
/// ```
pub fn bubble_sort(arr: &mut [i32]) {
    let mut n = arr.len();
    let mut swapped;

    // Outer loop to traverse through all elements
    while n > 1 {
        swapped = false;

        // Inner loop to compare adjacent elements
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }

        // If no elements were swapped, the array is already sorted
        if !swapped {
            break;
        }

        // Reduce the range of the next pass
        n -= 1;
    }
}
