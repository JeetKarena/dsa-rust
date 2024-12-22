//! # DSA in Rust
//! 
//! A comprehensive library of Data Structures and Algorithms implemented in Rust.
//! Focused on performance, safety, and extensive benchmarking capabilities.

// Module declarations with documentation
pub mod algorithms {
    //! Contains implementations of various sorting and searching algorithms
    
    pub mod sorting {
        //! Sorting algorithm implementations including:
        //! - Bubble Sort
        //! - Quick Sort
        //! - Merge Sort
        pub mod bubble_sort;
    }

    pub mod searching {
        //! Searching algorithm implementations including:
       
    }
}

pub mod data_structures {
    //! Custom implementations of fundamental data structures
    
}

pub mod utils {
    //! Utility functions for benchmarking and testing
    //! 
    //! Includes helpers for:
    //! - Generating test data (1M integers)
    //! - File I/O operations
    //! - Data shuffling
    
    pub mod helpers;
    pub mod validators;
    pub mod conversions;
}

// Re-export commonly used items
pub use algorithms::sorting::*;
pub use utils::helpers::*;