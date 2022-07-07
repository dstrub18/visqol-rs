#![feature(test)]
extern crate test;

use ndarray::Array2;
use visqol_rs::{gammatone_filterbank::GammatoneFilterbank, equivalent_rectangular_bandwidth};

#[bench]
fn bench_gammatone_filterbank(b: &mut test::Bencher)
{
    let fs =  48000;
    let num_bands = 32;
    let min_freq = 50.0f64;

    let ten_samples = vec![0.0;fs];

    let erb = equivalent_rectangular_bandwidth::make_filters(fs, num_bands, min_freq, fs as f64 / 2.0);
    
    let mut filterbank = GammatoneFilterbank::new(num_bands, min_freq);
    filterbank.reset_filter_conditions();
    filterbank.set_filter_coefficients(&erb.filter_coeffs);

    b.iter(|| filterbank.apply_filter(&ten_samples));
}
