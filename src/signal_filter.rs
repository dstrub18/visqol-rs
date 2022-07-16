pub struct FilterResults
{
    pub filtered_signal: Vec<f64>,
    pub final_conditions: [f64;Self::NUM_FILTER_CONDITIONS]
}

impl FilterResults
{
    pub const NUM_FILTER_CONDITIONS: usize = 2;
}

pub fn filter_signal(numerator_coeffs: &[f64], denom_coeffs: &[f64], signal: &[f64], init_conditions: &mut [f64]) -> FilterResults
{
    let mut filtered_signal = vec![0.0f64; signal.len()];

    filtered_signal.iter_mut().zip(signal).for_each(|(filtered_element, signal_element)|
    {
        *filtered_element = numerator_coeffs[0] * *signal_element + init_conditions[0];

        init_conditions[0] = numerator_coeffs[1] * signal_element + init_conditions[1] - denom_coeffs[1] * *filtered_element;
        init_conditions[1] = numerator_coeffs[2] * signal_element - denom_coeffs[2] * *filtered_element;
    }
    );

    FilterResults
    {
        filtered_signal,
        final_conditions: [init_conditions[0], init_conditions[1]]
    }
}