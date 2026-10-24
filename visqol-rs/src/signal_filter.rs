/// Applies a filter described by its numerator `numerator_coeffs` and its denominator `denominator_coeffs` on `signal` in place.
/// `filter_conditions` holds the initial conditions on entry and the final conditions on return.
#[inline(always)]
pub fn filter_signal_in_place(
    numerator_coeffs: &[f64; 3],
    denom_coeffs: &[f64; 3],
    signal: &mut [f64],
    filter_conditions: &mut [f64; 2],
) {
    // Transposed direct form II: each output sample only depends on the current
    // input sample and the filter state, so the signal can be overwritten in place.
    signal.iter_mut().for_each(|element| {
        let signal_element = *element;
        let filtered_element = numerator_coeffs[0] * signal_element + filter_conditions[0];

        filter_conditions[0] = numerator_coeffs[1] * signal_element + filter_conditions[1]
            - denom_coeffs[1] * filtered_element;
        filter_conditions[1] =
            numerator_coeffs[2] * signal_element - denom_coeffs[2] * filtered_element;

        *element = filtered_element;
    });
}
