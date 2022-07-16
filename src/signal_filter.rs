pub struct FilterResults
{
    pub filtered_signal: Vec<f64>,
    pub final_conditions: [f64;Self::NUM_FILTER_CONDITIONS]
}

impl FilterResults
{
    pub const NUM_FILTER_CONDITIONS: usize = 2;
}

// returns filter results
pub fn filter_signal(numerator_coeffs: &[f64], denom_coeffs: &[f64], signal: &[f64], init_conditions: &[f64]) -> FilterResults
{
    let mut filtered_signal = vec![0.0f64; signal.len()];

    let mut final_conditions = init_conditions.clone().to_vec();
    // Final conditions does not need to be resized, does it?
    final_conditions.push(0.0);

    filtered_signal.iter_mut().zip(signal).for_each(|(filtered_element, signal_element)|
    {
        *filtered_element = numerator_coeffs[0] * *signal_element + final_conditions[0];
        
        for (i, (num, den)) in numerator_coeffs.iter()
                                                                  .zip(denom_coeffs)
                                                                  .skip(1)
                                                                  .enumerate()
        {
            final_conditions[i] = *num * signal_element +
            final_conditions[i + 1] - *den * *filtered_element;
        }
    }
    );

    FilterResults
    {
        filtered_signal,
        final_conditions: [final_conditions[0], final_conditions[1]]
    }
}