use visqol_rs::misc_math;
#[test]
fn test_next_pow_two() {
    let inputs = vec![2, 10, 3, 5, 48000, 7, 23, 32];
    let expected = vec![2, 16, 4, 8, 65536, 8, 32, 32];

    let mut results = Vec::new();
    for i in inputs.iter() {
        results.push(misc_math::next_pow_two(*i));
    }
    assert_eq!(results, expected);
}

#[test]
fn test_exponential_from_fit() {
    assert_eq!(
        1.446_176_4,
        misc_math::exponential_from_fit(0.5, 1.15, 4.68, 0.76)
    );
    assert_eq!(
        4.224_677_6,
        misc_math::exponential_from_fit(1.0, 1.15, 4.68, 0.76)
    );
}
