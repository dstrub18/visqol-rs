use ndarray::Array1;

/// represents a time domain audio signal.
pub struct AudioSignal {
    /// The actual time domain samples
    pub data_matrix: Array1<f32>,
    /// The sample rate
    pub sample_rate: u32,
}

impl AudioSignal {
    /// Creates a new `AudioSignal`.
    pub fn new(data_matrix: &[f32], sample_rate: u32) -> AudioSignal {
        AudioSignal {
            data_matrix: Array1::from_vec(data_matrix.to_vec()),
            sample_rate,
        }
    }
    /// Returns the length of the signal in seconds
    pub fn get_duration(&self) -> f32 { self.len() as f32 / self.sample_rate as f32 }

    /// Returns the length of the signal in samples
    pub fn len(&self) -> usize { self.data_matrix.len() }

    /// Returns `true` if the number o samples is 0
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}

impl std::ops::Index<usize> for AudioSignal {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 { &(self.data_matrix[index]) }
}

impl std::ops::IndexMut<usize> for AudioSignal {
    fn index_mut(&mut self, index: usize) -> &mut f32 { &mut (self.data_matrix[index]) }
}
