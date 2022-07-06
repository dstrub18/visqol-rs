use num::{complex::Complex64, Zero};

use visqol_rs::{test_utility::compare_complex_vec};
use rustfft::FftPlanner;
use approx::assert_abs_diff_eq;

#[test]
fn rustfft_test()
{
    let mut samples = vec![Complex64::new(0.000150529, 0.0), Complex64::new(5.89739e-05, 0.0),
    Complex64::new(-9.36187e-05, 0.0), Complex64::new(-9.36187e-05, 0.0), Complex64::new(0.000394677, 0.0), Complex64::new(0.000303122, 0.0), Complex64::new(-9.36187e-05, 0.0),
    Complex64::new(0.000303122, 0.0), Complex64::new(8.94924e-05, 0.0), Complex64::new(0.000150529, 0.0), Complex64::new(-2.06314e-06, 0.0), Complex64::new(-0.000154656, 0.0),
    Complex64::new(2.84554e-05, 0.0), Complex64::new(0.000272603, 0.0), Complex64::new(-0.000185174, 0.0), Complex64::new(5.89739e-05, 0.0), Complex64::new(0.000364159, 0.0),
    Complex64::new(0.000425196, 0.0), Complex64::new(0.000120011, 0.0), Complex64::new(5.89739e-05, 0.0), Complex64::new(0.000120011, 0.0), Complex64::new(0.000211566, 0.0),
    Complex64::new(0.000150529, 0.0), Complex64::new(2.84554e-05, 0.0), Complex64::new(0.000120011, 0.0), Complex64::new(0.000150529, 0.0), Complex64::new(8.94924e-05, 0.0),
    Complex64::new(-9.36187e-05, 0.0), Complex64::new(0.000242085, 0.0), Complex64::new(0.000486233, 0.0), Complex64::new(8.94924e-05, 0.0), Complex64::new(0.000120011, 0.0),
    Complex64::new(8.94924e-05, 0.0), Complex64::new(8.94924e-05, 0.0), Complex64::new(0.000150529, 0.0), Complex64::new(-0.000154656, 0.0), Complex64::new(-2.06314e-06, 0.0),
    Complex64::new(2.84554e-05, 0.0), Complex64::new(0.000150529, 0.0), Complex64::new(-3.25817e-05, 0.0), Complex64::new(5.89739e-05, 0.0), Complex64::new(5.89739e-05, 0.0),
    Complex64::new(-0.000124137, 0.0), Complex64::new(5.89739e-05, 0.0), Complex64::new(2.84554e-05, 0.0), Complex64::new(0.000181048, 0.0), Complex64::new(-0.000124137, 0.0),
    Complex64::new(-0.000368285, 0.0), Complex64::new(-9.36187e-05, 0.0), Complex64::new(-0.000246211, 0.0), Complex64::new(-3.25817e-05, 0.0), Complex64::new(2.84554e-05, 0.0),
    Complex64::new(-6.31002e-05, 0.0), Complex64::new(-9.36187e-05, 0.0), Complex64::new(-2.06314e-06, 0.0), Complex64::new(-2.06314e-06, 0.0), Complex64::new(-0.000185174, 0.0),
    Complex64::new(-0.000124137, 0.0), Complex64::new(2.84554e-05, 0.0), Complex64::new(8.94924e-05, 0.0), Complex64::new(-0.00027673, 0.0), Complex64::new(-6.31002e-05, 0.0),
    Complex64::new(-0.000215693, 0.0), Complex64::new(0.000425196, 0.0), Complex64::new(8.94924e-05, 0.0)];

    
    let mut planner = FftPlanner::<f64>::new();
    let fft_length = 128;
    let fft = planner.plan_fft_forward(fft_length);
    samples.resize(fft_length, Complex64::zero());
    
    let expected_td_signal = samples.clone();
    fft.process(&mut samples);
    // Check for double-sided spectrum
    assert_abs_diff_eq!(samples[1].re, samples.iter().last().unwrap().re);
    assert_abs_diff_eq!(samples[1].im, -samples.iter().last().unwrap().im);

    let ifft = planner.plan_fft_inverse(fft_length);
    ifft.process(&mut samples);
    samples.iter_mut().for_each(|element|{element.re *= 1.0f64/128.0f64});
    compare_complex_vec(&samples, &expected_td_signal, 0.000001);
}