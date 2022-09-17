use crate::{constants, signal_filter};

/// Bank of gammatone filters on each frame of a time domain signal to construct a spectrogram representation.
/// This implementation is fixed to a 4th order filterbank.
pub struct GammatoneFilterbank {
    pub num_bands: usize,
    pub min_freq: f64,

    filter_conditions_1: Vec<[f64; constants::NUM_FILTER_CONDITIONS]>,
    filter_conditions_2: Vec<[f64; constants::NUM_FILTER_CONDITIONS]>,
    filter_conditions_3: Vec<[f64; constants::NUM_FILTER_CONDITIONS]>,
    filter_conditions_4: Vec<[f64; constants::NUM_FILTER_CONDITIONS]>,

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

impl GammatoneFilterbank {
    /// Creates a new gammatone filterbank with the desired number of frequency bands and the minimum frequency.
    pub fn new(num_bands: usize, min_freq: f64) -> Self {
        Self {
            num_bands,
            min_freq,
            filter_conditions_1: vec![[0.0; constants::NUM_FILTER_CONDITIONS]; num_bands],
            filter_conditions_2: vec![[0.0; constants::NUM_FILTER_CONDITIONS]; num_bands],
            filter_conditions_3: vec![[0.0; constants::NUM_FILTER_CONDITIONS]; num_bands],
            filter_conditions_4: vec![[0.0; constants::NUM_FILTER_CONDITIONS]; num_bands],
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
        self.filter_conditions_1 = vec![[0.0, 0.0]; self.num_bands];
        self.filter_conditions_2 = vec![[0.0, 0.0]; self.num_bands];
        self.filter_conditions_3 = vec![[0.0, 0.0]; self.num_bands];
        self.filter_conditions_4 = vec![[0.0, 0.0]; self.num_bands];
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

        let mut output = ndarray::Array2::<f64>::zeros((self.num_bands, input_signal.len()));
        for band in 0..self.num_bands {
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
    use ndarray::{Array2, Axis};

    use super::*;
    #[test]
    fn gammatone_filterbank() {
        let fs = 48000;
        let num_bands = 32;
        let min_freq = 50.0f64;

        let ten_samples = vec![0.2, 0.4, 0.6, 0.8, 0.9, 0.1, 0.3, 0.5, 0.7, 0.9];

        let (mut filter_coeffs, _) = equivalent_rectangular_bandwidth::make_filters(
            fs,
            num_bands,
            min_freq,
            fs as f64 / 2.0,
        );

        filter_coeffs.invert_axis(Axis(0));


        let epsilon = 0.0001;


        // Check if filtering works as intended.
        let mut filterbank = GammatoneFilterbank::new(num_bands, min_freq);
        filterbank.reset_filter_conditions();
        filterbank.set_filter_coefficients(&filter_coeffs);

        let filtered_signal = filterbank.apply_filter(&ten_samples);

        // Check dimensions
        assert_eq!(filtered_signal.ncols(), 10);
        assert_eq!(filtered_signal.nrows(), 32);

        // Check individual elements
        let expected_output = vec![
            1.028e-10,
            6.15143e-10,
            2.14718e-09,
            5.71021e-09,
            1.27613e-08,
            2.47844e-08,
            4.33919e-08,
            7.04246e-08,
            1.0805e-07,
            1.58858e-07,
            1.79716e-10,
            1.07493e-09,
            3.75028e-09,
            9.96822e-09,
            2.22642e-08,
            4.32104e-08,
            7.55911e-08,
            1.22574e-07,
            1.87878e-07,
            2.75939e-07,
            3.13758e-10,
            1.87569e-09,
            6.54e-09,
            1.73711e-08,
            3.87673e-08,
            7.51665e-08,
            1.31342e-07,
            2.12696e-07,
            3.25538e-07,
            4.77363e-07,
            5.47461e-10,
            3.27074e-09,
            1.13952e-08,
            3.02388e-08,
            6.74105e-08,
            1.30526e-07,
            2.27702e-07,
            3.68039e-07,
            5.62087e-07,
            8.22287e-07,
            9.54914e-10,
            5.70061e-09,
            1.98409e-08,
            5.25855e-08,
            1.17053e-07,
            2.26224e-07,
            3.93742e-07,
            6.34692e-07,
            9.66337e-07,
            1.40882e-06,
            1.6651e-09,
            9.93091e-09,
            3.45197e-08,
            9.13398e-08,
            2.02909e-07,
            3.91148e-07,
            6.78622e-07,
            1.08974e-06,
            1.65187e-06,
            2.39635e-06,
            2.90252e-09,
            1.7291e-08,
            6.00032e-08,
            1.58425e-07,
            3.5098e-07,
            6.74214e-07,
            1.16458e-06,
            1.86013e-06,
            2.80213e-06,
            4.03634e-06,
            5.05767e-09,
            3.00866e-08,
            1.04182e-07,
            2.74276e-07,
            6.05419e-07,
            1.15743e-06,
            1.9871e-06,
            3.15034e-06,
            4.70404e-06,
            6.70759e-06,
            8.80934e-09,
            5.23113e-08,
            1.80634e-07,
            4.73735e-07,
            1.04055e-06,
            1.97639e-06,
            3.36473e-06,
            5.27907e-06,
            7.78452e-06,
            1.09388e-05,
            1.53366e-08,
            9.08698e-08,
            3.12642e-07,
            8.1579e-07,
            1.78001e-06,
            3.35104e-06,
            5.63909e-06,
            8.71842e-06,
            1.26271e-05,
            1.73659e-05,
            2.66855e-08,
            1.57675e-07,
            5.39929e-07,
            1.39939e-06,
            3.02621e-06,
            5.6284e-06,
            9.31926e-06,
            1.41097e-05,
            1.99037e-05,
            2.64943e-05,
            4.64034e-08,
            2.73227e-07,
            9.29851e-07,
            2.38846e-06,
            5.10301e-06,
            9.33382e-06,
            1.51057e-05,
            2.21839e-05,
            3.00649e-05,
            3.7976e-05,
            8.06325e-08,
            4.72685e-07,
            1.59569e-06,
            4.04998e-06,
            8.51188e-06,
            1.52116e-05,
            2.38229e-05,
            3.34132e-05,
            4.2446e-05,
            4.88261e-05,
            1.39994e-07,
            8.16105e-07,
            2.72591e-06,
            6.80845e-06,
            1.39911e-05,
            2.41965e-05,
            3.6091e-05,
            4.70224e-05,
            5.31291e-05,
            4.95905e-05,
            2.42827e-07,
            1.40553e-06,
            4.62952e-06,
            1.13159e-05,
            2.25401e-05,
            3.71709e-05,
            5.13695e-05,
            5.86526e-05,
            5.04691e-05,
            1.71731e-05,
            4.20734e-07,
            2.41323e-06,
            7.80318e-06,
            1.85217e-05,
            3.53061e-05,
            5.41889e-05,
            6.5676e-05,
            5.55346e-05,
            6.9789e-06,
            -9.61956e-05,
            7.28072e-07,
            4.1275e-06,
            1.30231e-05,
            2.96907e-05,
            5.30972e-05,
            7.25476e-05,
            6.68569e-05,
            7.87681e-06,
            -0.000132533,
            -0.000373849,
            1.2581e-06,
            7.02559e-06,
            2.14531e-05,
            4.62326e-05,
            7.50386e-05,
            8.26823e-05,
            2.61336e-05,
            -0.000143382,
            -0.000461003,
            -0.000927986,
            2.17039e-06,
            1.18859e-05,
            3.47296e-05,
            6.904e-05,
            9.5525e-05,
            6.06428e-05,
            -0.000113954,
            -0.000496069,
            -0.00109621,
            -0.00183079,
            3.73712e-06,
            1.99536e-05,
            5.49075e-05,
            9.67432e-05,
            9.82883e-05,
            -4.30632e-05,
            -0.000449093,
            -0.00117089,
            -0.00209555,
            -0.00291187,
            6.42082e-06,
            3.31663e-05,
            8.39951e-05,
            0.000121892,
            4.68025e-05,
            -0.000313564,
            -0.00109871,
            -0.0022202,
            -0.00323051,
            -0.00340587,
            1.10043e-05,
            5.4424e-05,
            0.000122522,
            0.000123862,
            -0.000126682,
            -0.000862745,
            -0.00212067,
            -0.00340113,
            -0.0036436,
            -0.0017683,
            1.88057e-05,
            8.78158e-05,
            0.000166157,
            5.82356e-05,
            -0.000521654,
            -0.00176044,
            -0.00330141,
            -0.00386608,
            -0.00180368,
            0.00333298,
            3.2033e-05,
            0.000138557,
            0.000199025,
            -0.000152934,
            -0.00122796,
            -0.00285313,
            -0.00388236,
            -0.00223557,
            0.00320785,
            0.00993226,
            5.43595e-05,
            0.000212072,
            0.000182866,
            -0.000612925,
            -0.00219063,
            -0.00350466,
            -0.00267867,
            0.00197555,
            0.00874927,
            0.0107822,
            9.1852e-05,
            0.000311075,
            4.47808e-05,
            -0.00138449,
            -0.00296412,
            -0.00263263,
            0.000504145,
            0.00576003,
            0.00860718,
            0.00185651,
            0.000154444,
            0.000428755,
            -0.00032244,
            -0.00231263,
            -0.00256793,
            0.000111724,
            0.00246955,
            0.00422373,
            0.00363507,
            -0.00539835,
            0.000258238,
            0.000535607,
            -0.00100997,
            -0.00275819,
            -0.000140297,
            0.00197327,
            -0.00138272,
            0.00224969,
            0.00588903,
            -0.00970154,
            0.000429047,
            0.000559124,
            -0.00192844,
            -0.00158922,
            0.00295764,
            -0.00205131,
            -0.00531702,
            0.012985,
            0.00121162,
            -0.0270996,
            0.000707771,
            0.000364181,
            -0.00249945,
            0.00170197,
            0.00193977,
            -0.00869188,
            0.0048275,
            0.0165009,
            -0.0280708,
            0.00164967,
            0.00115902,
            -0.00023268,
            -0.00144458,
            0.00447121,
            -0.00428925,
            -0.00394416,
            0.0120091,
            -0.00317878,
            -0.0194642,
            0.0381694,
            0.00190606,
            -0.00134351,
            0.00244894,
            0.00124486,
            -0.00252133,
            -0.00117925,
            0.00894377,
            -0.00940723,
            0.00334385,
            0.0157258,
        ];

        let expected_output_mat = Array2::from_shape_vec((32, 10), expected_output).unwrap();

        for (&res, ex) in filtered_signal.iter().zip(expected_output_mat) {
            assert_abs_diff_eq!(res, ex, epsilon = epsilon);
        }
    }
}
