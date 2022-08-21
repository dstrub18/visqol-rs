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
