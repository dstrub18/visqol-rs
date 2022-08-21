use crate::{constants, signal_filter};

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

    pub fn reset_filter_conditions(&mut self) {
        self.filter_conditions_1 = vec![[0.0, 0.0]; self.num_bands];
        self.filter_conditions_2 = vec![[0.0, 0.0]; self.num_bands];
        self.filter_conditions_3 = vec![[0.0, 0.0]; self.num_bands];
        self.filter_conditions_4 = vec![[0.0, 0.0]; self.num_bands];
    }

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

    pub fn apply_filter(&mut self, signal: &[f64]) -> ndarray::Array2<f64> {
        let mut a1 = [0.0; 3];
        let mut a2 = [0.0; 3];
        let mut a3 = [0.0; 3];
        let mut a4 = [0.0; 3];
        let mut b = [0.0; 3];

        let mut output = ndarray::Array2::<f64>::zeros((self.num_bands, signal.len()));
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
            let mut filter_result =
                signal_filter::filter_signal(&a1, &b, signal, &mut self.filter_conditions_1[band]);
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
