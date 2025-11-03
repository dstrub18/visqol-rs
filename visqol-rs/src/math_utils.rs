use ndarray::Array1;
use ndarray_stats::QuantileExt;
pub fn normalize_signal(signal: &Array1<f32>) -> Array1<f32> {
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
pub fn exponential_from_fit(x: f32, a: f32, b: f32, x_0: f32) -> f32 { a + (b * (x - x_0)).exp() }

/// Normalizes a slice of `i16` to a vector of `f32` values
pub fn normalize_int16_to_double(input: &[i16]) -> Vec<f32> {
    input
        .iter()
        .map(|x| *x as f32 / 32767.0f32)
        .collect::<Vec<f32>>()
}

/// Returns the maximum of an `ndarray::Array1<f32>`
fn get_max(mat: &Array1<f32>) -> f32 { *mat.max().expect("Failed to compute maximum of matrix!") }

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;
    #[test]
    fn test_next_pow_two() {
        let inputs = [2, 10, 3, 5, 48000, 7, 23, 32];
        let expected = vec![2, 16, 4, 8, 65536, 8, 32, 32];

        let mut results = Vec::new();
        for i in inputs.iter() {
            results.push(next_pow_two(*i));
        }
        assert_eq!(results, expected);
    }

    #[test]
    fn test_exponential_from_fit() {
        assert_abs_diff_eq!(
            1.446_176_4,
            exponential_from_fit(0.5, 1.15, 4.68, 0.76),
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            4.224_677_6,
            exponential_from_fit(1.0, 1.15, 4.68, 0.76),
            epsilon = 0.0001
        );
    }
}
