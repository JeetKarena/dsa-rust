use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufWriter, Write};

/// Generates a vector of sorted integers from 1 to `count`.
///
/// # Arguments
///
/// * `count` - The number of integers to generate.
///
/// # Returns
///
/// A vector containing sorted integers from 1 to `count`.
///
/// # Examples
///
/// ```
/// use dsa_in_rust::utils::helpers::generate_sorted_integers;
///
/// let sorted_integers = generate_sorted_integers(5);
/// assert_eq!(sorted_integers, vec![1, 2, 3, 4, 5]);
/// ```
pub fn generate_sorted_integers(count: usize) -> Vec<i32> {
    (1..=count as i32).collect()
}

/// Writes a slice of integers to a file, each on a new line.
///
/// # Arguments
///
/// * `filename` - The name of the file to write to.
/// * `integers` - A slice of integers to write to the file.
///
/// # Returns
///
/// An `io::Result<()>` indicating success or failure.
///
/// # Examples
///
/// ```
/// use dsa_in_rust::utils::helpers::write_integers_to_file;
///
/// let integers = vec![1, 2, 3, 4, 5];
/// write_integers_to_file("output.txt", &integers).expect("Failed to write to file");
/// ```
pub fn write_integers_to_file(filename: &str, integers: &[i32]) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    for &num in integers {
        writeln!(writer, "{}", num)?;
    }
    Ok(())
}

/// Shuffles a mutable slice of integers in place.
///
/// # Arguments
///
/// * `integers` - A mutable slice of integers to shuffle.
///
/// # Examples
///
/// ```
/// use dsa_in_rust::utils::helpers::shuffle_integers;
///
/// let mut integers = vec![1, 2, 3, 4, 5];
/// shuffle_integers(&mut integers);
/// ```
pub fn shuffle_integers(integers: &mut [i32]) {
    let mut rng = rand::thread_rng();
    integers.shuffle(&mut rng);
}
