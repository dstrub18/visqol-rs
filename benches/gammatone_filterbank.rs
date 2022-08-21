#![feature(test)]
extern crate test;

use visqol_rs::{equivalent_rectangular_bandwidth, gammatone_filterbank::GammatoneFilterbank};

#[bench]
fn bench_gammatone_filterbank(b: &mut test::Bencher) {
    let fs = 48000;
    let num_bands = 32;
    let min_freq = 50.0f64;

    let input = vec![0.0; fs];

    let (filter_coeffs, _) =
        equivalent_rectangular_bandwidth::make_filters(fs, num_bands, min_freq, fs as f64 / 2.0);

    let mut filterbank = GammatoneFilterbank::new(num_bands, min_freq);
    filterbank.reset_filter_conditions();
    filterbank.set_filter_coefficients(&filter_coeffs);

    b.iter(|| filterbank.apply_filter(&input));
}
