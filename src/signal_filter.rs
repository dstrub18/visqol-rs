#[allow(unused)]
pub struct FilterResults
{
    pub filtered_signal: Vec<f64>,
    pub final_conditions: Vec<f64>
}
// returns filter results
pub fn filter_signal(numerator_coeffs: &Vec<f64>, denom_coeffs: &Vec<f64>, signal: &Vec<f64>, init_conditions: &Vec<f64>) -> FilterResults
{
    let mut filtered_signal = vec![0.0f64; signal.len()];
    let mut final_conditions = init_conditions.clone();

    for m in 0..filtered_signal.len()
    {
        filtered_signal[m] = numerator_coeffs[0] * signal[m] + final_conditions[0];
        for i in 1..denom_coeffs.len()
        {
            final_conditions[i - 1] = numerator_coeffs[i] * signal[m] + 
                                      final_conditions[i] - denom_coeffs[i] * filtered_signal[m];
        }
    }

    FilterResults
    {
        filtered_signal: filtered_signal,
        final_conditions: final_conditions[..final_conditions.len()].to_vec()
    }
}