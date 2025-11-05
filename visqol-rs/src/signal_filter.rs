/// Applies a filter described by its numerator `numerator_coeffs` and its denominator `denominator_coeffs` on `signal` and returns a filtered copy of the signal.
#[inline(always)]
pub fn filter_signal(
    numerator_coeffs: &[f32],
    denom_coeffs: &[f32],
    signal: &[f32],
    init_conditions: &mut [f32],
) -> Vec<f32> {
    let mut filtered_signal = vec![0.0f32; signal.len()];

    filtered_signal
        .iter_mut()
        .zip(signal)
        .for_each(|(filtered_element, signal_element)| {
            *filtered_element = numerator_coeffs[0] * *signal_element + init_conditions[0];

            init_conditions[0] = numerator_coeffs[1] * signal_element + init_conditions[1]
                - denom_coeffs[1] * *filtered_element;
            init_conditions[1] =
                numerator_coeffs[2] * signal_element - denom_coeffs[2] * *filtered_element;
        });

    filtered_signal
}
