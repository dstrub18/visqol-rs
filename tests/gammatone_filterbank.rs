#[test]
fn gammatone_filterbank()
{
    //use visqol_rs::gammatone_filterbank::GammatoneFilterbank;
    use visqol_rs::equivalent_rectangular_bandwidth::*;
    let fs =  48000;
    let num_bands = 32;
    let min_freq = 50.0f64;

    let k_10_samples = vec![0.2, 0.4, 0.6, 0.8, 0.9, 0.1, 0.3, 0.5, 0.7, 0.9];

    let erb = equivalent_rectangular_bandwidth::make_filters(fs, num_bands, min_freq, fs as f64 / 2.0);
    assert_eq!(erb.filter_coeffs.shape()[0], 10);
    assert_eq!(erb.filter_coeffs.shape()[1], num_bands);
}