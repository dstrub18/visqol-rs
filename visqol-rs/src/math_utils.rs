use ndarray::Array1;
use ndarray_stats::QuantileExt;
pub fn normalize_signal(signal: &Array1<f64>) -> Array1<f64> {
    let normalized_mat = signal.clone();
    let max = get_max(signal);
    normalized_mat / max
}

pub fn next_pow_two(input: usize) -> usize {
    let mut next_power_of_two = input - 1;

    next_power_of_two |= next_power_of_two >> 1;
    next_power_of_two |= next_power_of_two >> 2;
    next_power_of_two |= next_power_of_two >> 4;
    next_power_of_two |= next_power_of_two >> 1;
    next_power_of_two |= next_power_of_two >> 16;
    next_power_of_two + 1
}

/// Returns the exponential fit between 2 points
pub fn exponential_from_fit(x: f64, a: f64, b: f64, x_0: f64) -> f64 { a + (b * (x - x_0)).exp() }

/// Normalizes a slice of `i16` to a vector of `f64` values
pub fn normalize_int16_to_double(input: &[i16]) -> Vec<f64> {
    input
        .iter()
        .map(|x| *x as f64 / 32767.0f64)
        .collect::<Vec<f64>>()
}

/// Returns the maximum of an `ndarray::Array1<f64>`
fn get_max(mat: &Array1<f64>) -> f64 { *mat.max().expect("Failed to compute maximum of matrix!") }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_pow_two() {
        let inputs = vec![2, 10, 3, 5, 48000, 7, 23, 32];
        let expected = vec![2, 16, 4, 8, 65536, 8, 32, 32];

        let mut results = Vec::new();
        for i in inputs.iter() {
            results.push(next_pow_two(*i));
        }
        assert_eq!(results, expected);
    }

    #[test]
    fn test_exponential_from_fit() {
        assert_eq!(1.446_176_4, exponential_from_fit(0.5, 1.15, 4.68, 0.76));
        assert_eq!(4.224_677_6, exponential_from_fit(1.0, 1.15, 4.68, 0.76));
    }
}
