//! Benchmarking for sorting algorithms
//! 
//! This module benchmarks the performance of various sorting algorithms implemented in the library.
//! The benchmarks are designed to measure the execution time of sorting algorithms on randomly shuffled arrays.
//! 
//! # Benchmarking Framework
//! 
//! We use the Criterion.rs library for benchmarking, which provides statistically rigorous measurements.
//! Each benchmark is run multiple times to ensure accuracy and reliability of the results.

extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use dsa_in_rust::algorithms::sorting::bubble_sort::bubble_sort;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Benchmarks the Bubble Sort algorithm
/// 
/// This function benchmarks the performance of the Bubble Sort algorithm on a randomly shuffled array of 1,000 integers.
/// The array is shuffled before each run to simulate average-case performance.
fn benchmark_bubble_sort(c: &mut Criterion) {
    // Generate a sorted array of 1,000 integers
    let mut data: Vec<i32> = (0..1_000).collect();
    let mut rng = thread_rng();
    
    // Shuffle the array to simulate average-case performance
    data.shuffle(&mut rng);

    // Benchmark the Bubble Sort algorithm
    c.bench_function("bubble_sort", |b| b.iter(|| bubble_sort(black_box(&mut data.clone()))));
}

// Group the benchmarks and define the main function
criterion_group!(benches, benchmark_bubble_sort);
criterion_main!(benches);
