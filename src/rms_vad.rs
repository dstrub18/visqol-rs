/// Calculates the root mean square of a signal.
/// Based on a threshold, it computes a binary signal for a single chunk of data.
/// When returning the VAD results, previous chunks are analyzed to make the VAD less erratic
pub struct RmsVad {
    /// Indicates that voice acitivity is detected
    voice_activity_present: f64,
    /// Indicates that no voice acitivity is detected
    voice_activity_absent: f64,
    /// Criterion to replace the presence of voice chunks with absence
    silent_chunk_count: usize,
    /// Threshold which determines when activity is detected
    rms_threshold: f64,
    /// Vector containing the VAD results per chunk
    results_per_chunk: Vec<f64>,
}

impl Default for RmsVad {
    /// Provides a default congfiguration as specified in the Visqol standard.
    fn default() -> Self {
        Self {
            voice_activity_present: 1.0,
            voice_activity_absent: 0.0,
            silent_chunk_count: 3,
            rms_threshold: 5000.0,
            results_per_chunk: Vec::new(),
        }
    }
}
impl RmsVad {
    /// Given a chunk of data this function determines whether or not voice acitivity is present, storing its result in `each_chunk_result`
    pub fn process_chunk(&mut self, chunk: &[i16]) -> f64 {
        let rms = self.calc_root_mean_square(chunk);
        if rms < self.rms_threshold {
            self.results_per_chunk.push(self.voice_activity_absent);
        } else {
            self.results_per_chunk.push(self.voice_activity_present);
        }
        rms
    }

    /// Replaces seemingly present flags with silent flags if the previous chunks were considered silent and returns vad results.
    pub fn get_vad_results(&mut self) -> Vec<f64> {
        let mut vad_results = vec![1.0; self.silent_chunk_count - 1];

        for i in self.silent_chunk_count - 1..self.results_per_chunk.len() {
            if self.results_per_chunk[i] == 0.0 && self.check_previous_chunks_for_silence(&i) {
                vad_results.push(self.voice_activity_absent);
            } else {
                vad_results.push(self.voice_activity_present);
            }
        }
        vad_results
    }

    /// Calculates and returns the root mean square value of a given slice.
    fn calc_root_mean_square(&self, chunk: &[i16]) -> f64 {
        let mut square: i64 = 0;
        for elem in chunk {
            square += (*elem as i64).pow(2);
        }
        (square as f64 / chunk.len() as f64).sqrt()
    }

    /// Given an index, this function checks if acitivity was not detected in the previous chunk. Returns `true`, if activity was NOT detected.
    fn check_previous_chunks_for_silence(&self, idx: &usize) -> bool {
        let mut previous_chunks_silent = true;
        for j in 1..self.silent_chunk_count {
            if self.results_per_chunk[idx - j] == self.voice_activity_present {
                previous_chunks_silent = false;
                break;
            }
        }
        previous_chunks_silent
    }
}
