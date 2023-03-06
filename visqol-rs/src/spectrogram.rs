use ndarray::{Array2, Axis};
use ndarray_stats::QuantileExt;

/// Contains the spectral representation of audio data
pub struct Spectrogram {
    /// Spectrogram data, rows signify center frequencies, columns signify time
    pub data: Array2<f64>,
    /// Center frequencies in Hz
    pub center_freq_bands: Vec<f64>,
}

impl Spectrogram {
    /// Creates a new spectrogram. Note that `data` and `center_freq_bands` are moved
    pub fn new(data: Array2<f64>, center_freq_bands: Vec<f64>) -> Self {
        Self {
            data,
            center_freq_bands,
        }
    }

    /// Converts the spectrogram from linear scale to dB scale
    pub fn convert_to_db(&mut self) {
        let sample_to_db = |element: f64| {
            let sample: f64 = if element == 0.0 {
                f64::EPSILON
            } else {
                element.abs()
            };
            10.0 * (sample.log10())
        };
        self.data.mapv_inplace(sample_to_db);
    }

    /// Returns the minimum value of the spectrogram
    pub fn get_minimum(&self) -> f64 {
        *self
            .data
            .min()
            .expect("Failed to compute minimum for spectrogram")
    }

    /// Elementwise subtraction of the spectrogram
    pub fn subtract_floor(&mut self, floor: f64) { self.data -= floor; }

    /// Clamps each value in the spectrogram to `new_floor`
    pub fn raise_floor(&mut self, new_floor: f64) {
        self.data.mapv_inplace(|element| new_floor.max(element));
    }

    /// Given a noise threshold and a second spectrogram, both spectrograms are raised to share the same noise floor specified by `noise_threshold`
    pub fn raise_floor_per_frame(&mut self, noise_threshold: f64, other: &mut Self) {
        let min_columns = self.data.ncols().min(other.data.ncols());

        for index in 0..min_columns {
            let our_frame = &mut self.data.index_axis_mut(Axis(1), index);
            let other_frame = &mut other.data.index_axis_mut(Axis(1), index);
            let our_max = our_frame
                .max()
                .expect("Failed to raise level for spectrogram!");
            let other_max = other_frame
                .max()
                .expect("Failed to raise level for spectrogram!");
            let any_max = our_max.max(*other_max);
            let floor_db = any_max - noise_threshold;
            our_frame.mapv_inplace(|element| floor_db.max(element));
            other_frame.mapv_inplace(|element| floor_db.max(element));
        }
    }
}

impl std::ops::Index<(usize, usize)> for Spectrogram {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output { &self.data[index] }
}

impl std::ops::IndexMut<(usize, usize)> for Spectrogram {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output { &mut self.data[index] }
}

#[cfg(test)]
mod tests {
    use crate::test_utility;
    use approx::assert_abs_diff_eq;
    use ndarray::Array2;

    use super::*;

    const TOLERANCE: f64 = 0.0001;
    const MIN_ELEM: f64 = -53.2;
    const FLOOR: f64 = 0.1;

    #[test]
    fn convert_to_db_test() {
        let elements = Array2::<f64>::from_shape_vec(
            (10, 1),
            vec![
                10.21, -4.63, 0.54, 87.98, 0.065, 0.0, MIN_ELEM, 8.7, 0.0, -2.76,
            ],
        )
        .unwrap();

        let elements_db_scaled = Array2::<f64>::from_shape_vec(
            (10, 1),
            vec![
                10.0903, 6.6558, -2.6761, 19.4438, -11.8709, -156.5356, 17.2591, 9.3952, -156.5356,
                4.4091,
            ],
        )
        .unwrap();

        let mut spectrogram = Spectrogram::new(elements, vec![]);
        spectrogram.convert_to_db();

        test_utility::compare_real_matrix(&spectrogram.data, &elements_db_scaled, TOLERANCE);
    }

    #[test]
    fn minimum_test() {
        let elements = Array2::<f64>::from_shape_vec(
            (10, 1),
            vec![
                10.21, -4.63, 0.54, 87.98, 0.065, 0.0, MIN_ELEM, 8.7, 0.0, -2.76,
            ],
        )
        .unwrap();

        let spectrogram = Spectrogram::new(elements, vec![]);

        assert_abs_diff_eq!(spectrogram.get_minimum(), MIN_ELEM, epsilon = TOLERANCE);
    }

    #[test]
    fn subtract_floor_test() {
        let elements = Array2::<f64>::from_shape_vec(
            (10, 1),
            vec![
                10.21, -4.63, 0.54, 87.98, 0.065, 0.0, MIN_ELEM, 8.7, 0.0, -2.76,
            ],
        )
        .unwrap();

        let elements_floor_subtracted = Array2::<f64>::from_shape_vec(
            (10, 1),
            vec![
                10.21 - FLOOR,
                -4.63 - FLOOR,
                0.54 - FLOOR,
                87.98 - FLOOR,
                0.065 - FLOOR,
                0.0 - FLOOR,
                MIN_ELEM - FLOOR,
                8.7 - FLOOR,
                0.0 - FLOOR,
                -2.76 - FLOOR,
            ],
        )
        .unwrap();

        let mut spectrogram = Spectrogram::new(elements, vec![]);

        spectrogram.subtract_floor(FLOOR);
        test_utility::compare_real_matrix(&spectrogram.data, &elements_floor_subtracted, TOLERANCE);
    }
}
