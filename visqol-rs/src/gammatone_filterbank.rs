use crate::{constants, signal_filter};

/// Bank of gammatone filters on each frame of a time domain signal to construct a spectrogram representation.
/// This implementation is fixed to a 4th order filterbank.
pub struct GammatoneFilterbank<const NUM_BANDS: usize> {
    pub min_freq: f64,

    filter_conditions_1: [[f64; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
    filter_conditions_2: [[f64; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
    filter_conditions_3: [[f64; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
    filter_conditions_4: [[f64; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],

    filter_coeff_a0: Vec<f64>,
    filter_coeff_a11: Vec<f64>,
    filter_coeff_a12: Vec<f64>,
    filter_coeff_a13: Vec<f64>,
    filter_coeff_a14: Vec<f64>,
    filter_coeff_a2: Vec<f64>,
    filter_coeff_b0: Vec<f64>,
    filter_coeff_b1: Vec<f64>,
    filter_coeff_b2: Vec<f64>,
    filter_coeff_gain: Vec<f64>,
}

impl<const NUM_BANDS: usize> GammatoneFilterbank<NUM_BANDS> {
    /// Creates a new gammatone filterbank with the desired number of frequency bands and the minimum frequency.
    pub fn new(min_freq: f64) -> Self {
        Self {
            min_freq,
            filter_conditions_1: [[0.0; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
            filter_conditions_2: [[0.0; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
            filter_conditions_3: [[0.0; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
            filter_conditions_4: [[0.0; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
            filter_coeff_a0: Vec::new(),
            filter_coeff_a11: Vec::new(),
            filter_coeff_a12: Vec::new(),
            filter_coeff_a13: Vec::new(),
            filter_coeff_a14: Vec::new(),
            filter_coeff_a2: Vec::new(),
            filter_coeff_b0: Vec::new(),
            filter_coeff_b1: Vec::new(),
            filter_coeff_b2: Vec::new(),
            filter_coeff_gain: Vec::new(),
        }
    }

    /// Sets all internal states of the filterbank to 0.
    pub fn reset_filter_conditions(&mut self) {
        self.filter_conditions_1 = [[0.0, 0.0]; NUM_BANDS];
        self.filter_conditions_2 = [[0.0, 0.0]; NUM_BANDS];
        self.filter_conditions_3 = [[0.0, 0.0]; NUM_BANDS];
        self.filter_conditions_4 = [[0.0, 0.0]; NUM_BANDS];
    }

    /// Populates the filter coefficients with `filter_coeffs`.
    pub fn set_filter_coefficients(&mut self, filter_coeffs: &ndarray::Array2<f64>) {
        self.filter_coeff_a0 = filter_coeffs.column(0).to_vec();
        self.filter_coeff_a11 = filter_coeffs.column(1).to_vec();
        self.filter_coeff_a12 = filter_coeffs.column(2).to_vec();
        self.filter_coeff_a13 = filter_coeffs.column(3).to_vec();
        self.filter_coeff_a14 = filter_coeffs.column(4).to_vec();
        self.filter_coeff_a2 = filter_coeffs.column(5).to_vec();
        self.filter_coeff_b0 = filter_coeffs.column(6).to_vec();
        self.filter_coeff_b1 = filter_coeffs.column(7).to_vec();
        self.filter_coeff_b2 = filter_coeffs.column(8).to_vec();
        self.filter_coeff_gain = filter_coeffs.column(9).to_vec();
    }

    /// Applies the gammatone filterbank on the time-domain signal `signal`, producing a Gammetone spectrogram.
    #[inline(always)]
    pub fn apply_filter(&mut self, input_signal: &[f64]) -> ndarray::Array2<f64> {
        let mut a1 = [0.0; 3];
        let mut a2 = [0.0; 3];
        let mut a3 = [0.0; 3];
        let mut a4 = [0.0; 3];
        let mut b = [0.0; 3];

        let mut output = ndarray::Array2::<f64>::zeros((NUM_BANDS, input_signal.len()));
        for band in 0..NUM_BANDS {
            a1[0] = self.filter_coeff_a0[band] / self.filter_coeff_gain[band];
            a1[1] = self.filter_coeff_a11[band] / self.filter_coeff_gain[band];
            a1[2] = self.filter_coeff_a2[band] / self.filter_coeff_gain[band];

            a2[0] = self.filter_coeff_a0[band];
            a2[1] = self.filter_coeff_a12[band];
            a2[2] = self.filter_coeff_a2[band];

            a3[0] = self.filter_coeff_a0[band];
            a3[1] = self.filter_coeff_a13[band];
            a3[2] = self.filter_coeff_a2[band];

            a4[0] = self.filter_coeff_a0[band];
            a4[1] = self.filter_coeff_a14[band];
            a4[2] = self.filter_coeff_a2[band];

            b[0] = self.filter_coeff_b0[band];
            b[1] = self.filter_coeff_b1[band];
            b[2] = self.filter_coeff_b2[band];

            // 1st filter
            let mut filter_result = signal_filter::filter_signal(
                &a1,
                &b,
                input_signal,
                &mut self.filter_conditions_1[band],
            );
            self.filter_conditions_1[band] = filter_result.final_conditions;

            // 2nd filter
            filter_result = signal_filter::filter_signal(
                &a2,
                &b,
                &filter_result.filtered_signal,
                &mut self.filter_conditions_2[band],
            );
            self.filter_conditions_2[band] = filter_result.final_conditions;

            // 3rd filter
            filter_result = signal_filter::filter_signal(
                &a3,
                &b,
                &filter_result.filtered_signal,
                &mut self.filter_conditions_3[band],
            );
            self.filter_conditions_3[band] = filter_result.final_conditions;

            // 4th filter
            filter_result = signal_filter::filter_signal(
                &a4,
                &b,
                &filter_result.filtered_signal,
                &mut self.filter_conditions_4[band],
            );
            self.filter_conditions_4[band] = filter_result.final_conditions;

            for i in 0..filter_result.filtered_signal.len() {
                output.row_mut(band)[i] = filter_result.filtered_signal[i];
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::equivalent_rectangular_bandwidth;
    use approx::assert_abs_diff_eq;
    use ndarray::Axis;

    use super::*;
    #[test]
    fn gammatone_filterbank() {
        let fs = 48000;
        const NUM_BANDS: usize = 32;
        let min_freq = 50.0f64;

        let ten_samples = vec![0.2, 0.4, 0.6, 0.8, 0.9, 0.1, 0.3, 0.5, 0.7, 0.9];

        let (mut filter_coeffs, _) = equivalent_rectangular_bandwidth::make_filters::<NUM_BANDS>(
            fs,
            min_freq,
            fs as f64 / 2.0,
        );

        filter_coeffs.invert_axis(Axis(0));

        let epsilon = 0.0001;

        // Check if filtering works as intended.
        let mut filterbank = GammatoneFilterbank::<{ NUM_BANDS }>::new(min_freq);
        filterbank.reset_filter_conditions();
        filterbank.set_filter_coefficients(&filter_coeffs);

        let filtered_signal = filterbank.apply_filter(&ten_samples);

        // Check dimensions
        assert_eq!(filtered_signal.ncols(), 10);
        assert_eq!(filtered_signal.nrows(), 32);

        // Check individual elements
        let expected_output = [1.028e-10, 6.15143e-10, 2.14718e-09];

        for (&res, ex) in expected_output.iter().zip(filtered_signal) {
            assert_abs_diff_eq!(res, ex, epsilon = epsilon);
        }
    }
}
