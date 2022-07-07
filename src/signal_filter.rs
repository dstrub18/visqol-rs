pub struct FilterResults
{
    pub filtered_signal: Vec<f64>,
    pub final_conditions: Vec<f64>
}
// returns filter results
pub fn filter_signal(numerator_coeffs: &[f64], denom_coeffs: &[f64], signal: &[f64], init_conditions: &[f64]) -> FilterResults
{
    let mut filtered_signal = vec![0.0f64; signal.len()];

    let mut final_conditions = init_conditions.clone().to_vec();
    final_conditions.push(0.0);

    filtered_signal.iter_mut().zip(signal).for_each(|(filtered_element, signal_element)|
    {
        *filtered_element = numerator_coeffs[0] * *signal_element + final_conditions[0];
        for i in 1..denom_coeffs.len()
        {
            final_conditions[i - 1] = numerator_coeffs[i] * signal_element +
            final_conditions[i] - denom_coeffs[i] * *filtered_element;
        }
    }
    );

    FilterResults
    {
        filtered_signal,
        final_conditions: final_conditions[..final_conditions.len() - 1].to_vec()
    }
}