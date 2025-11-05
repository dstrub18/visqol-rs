use crate::{constants, signal_filter};

/// Bank of gammatone filters on each frame of a time domain signal to construct a spectrogram representation.
/// This implementation is fixed to a 4th order filterbank.
pub struct GammatoneFilterbank<const NUM_BANDS: usize> {
    // intermediate states
    filter_conditions_1: [[f32; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
    filter_conditions_2: [[f32; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
    filter_conditions_3: [[f32; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
    filter_conditions_4: [[f32; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
    // filter coefficients
    a1: [[f32; 3]; NUM_BANDS],
    a2: [[f32; 3]; NUM_BANDS],
    a3: [[f32; 3]; NUM_BANDS],
    a4: [[f32; 3]; NUM_BANDS],
    b: [[f32; 3]; NUM_BANDS],
}

impl<const NUM_BANDS: usize> GammatoneFilterbank<NUM_BANDS> {
    /// Creates a new gammatone filterbank with the desired number of frequency bands and the minimum frequency.
    pub fn new() -> Self {
        Self {
            filter_conditions_1: [[0.0; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
            filter_conditions_2: [[0.0; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
            filter_conditions_3: [[0.0; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
            filter_conditions_4: [[0.0; constants::NUM_FILTER_CONDITIONS]; NUM_BANDS],
            a1: [[0.0; 3]; NUM_BANDS],
            a2: [[0.0; 3]; NUM_BANDS],
            a3: [[0.0; 3]; NUM_BANDS],
            a4: [[0.0; 3]; NUM_BANDS],
            b: [[0.0; 3]; NUM_BANDS],
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
    pub fn set_filter_coefficients<'a>(&mut self, filter_coeffs: &ndarray::ArrayView2<'a, f32>) {
        for band in 0..NUM_BANDS {
            self.a1[band][0] = filter_coeffs.column(0)[band] / filter_coeffs.column(9)[band];
            self.a1[band][1] = filter_coeffs.column(1)[band] / filter_coeffs.column(9)[band];
            self.a1[band][2] = filter_coeffs.column(5)[band] / filter_coeffs.column(9)[band];

            self.a2[band][0] = filter_coeffs.column(0)[band];
            self.a2[band][1] = filter_coeffs.column(2)[band];
            self.a2[band][2] = filter_coeffs.column(5)[band];

            self.a3[band][0] = filter_coeffs.column(0)[band];
            self.a3[band][1] = filter_coeffs.column(3)[band];
            self.a3[band][2] = filter_coeffs.column(5)[band];

            self.a4[band][0] = filter_coeffs.column(0)[band];
            self.a4[band][1] = filter_coeffs.column(4)[band];
            self.a4[band][2] = filter_coeffs.column(5)[band];

            self.b[band][0] = filter_coeffs.column(6)[band];
            self.b[band][1] = filter_coeffs.column(7)[band];
            self.b[band][2] = filter_coeffs.column(8)[band];
        }
    }

    /// Applies the gammatone filterbank on the time-domain signal `signal`, producing a Gammetone spectrogram.
    #[inline(always)]
    pub fn apply_filter(&mut self, input_signal: &[f32]) -> ndarray::Array2<f32> {
        let mut output = ndarray::Array2::<f32>::zeros((NUM_BANDS, input_signal.len()));
        for band in 0..NUM_BANDS {
            // 1st filter
            let mut filter_result = signal_filter::filter_signal(
                &self.a1[band],
                &self.b[band],
                input_signal,
                &mut self.filter_conditions_1[band],
            );

            // 2nd filter
            filter_result = signal_filter::filter_signal(
                &self.a2[band],
                &self.b[band],
                &filter_result,
                &mut self.filter_conditions_2[band],
            );

            // 3rd filter
            filter_result = signal_filter::filter_signal(
                &self.a3[band],
                &self.b[band],
                &filter_result,
                &mut self.filter_conditions_3[band],
            );

            // 4th filter
            filter_result = signal_filter::filter_signal(
                &self.a4[band],
                &self.b[band],
                &filter_result,
                &mut self.filter_conditions_4[band],
            );

            // This is all unnecessary allocations right?
            for i in 0..filter_result.len() {
                output.row_mut(band)[i] = filter_result[i];
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
        let min_freq = 50.0f32;

        let ten_samples = vec![0.2, 0.4, 0.6, 0.8, 0.9, 0.1, 0.3, 0.5, 0.7, 0.9];

        let (mut filter_coeffs, _) = equivalent_rectangular_bandwidth::make_filters::<NUM_BANDS>(
            fs,
            min_freq,
            fs as f32 / 2.0,
        );

        filter_coeffs.invert_axis(Axis(0));

        let epsilon = 0.0001;

        // Check if filtering works as intended.
        let mut filterbank = GammatoneFilterbank::<{ NUM_BANDS }>::new();
        filterbank.reset_filter_conditions();
        filterbank.set_filter_coefficients(&filter_coeffs.view());

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
