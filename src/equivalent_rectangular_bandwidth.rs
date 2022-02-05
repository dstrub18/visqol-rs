#[allow(unused)]
pub mod equivalent_rectangular_bandwidth
{
    use num::complex::Complex;
    use crate::complex_vec::ComplexVec;
    #[allow(unused)]
    // returns ErbFiltersResult
    pub fn make_filters(sample_rate: usize, num_channels: usize, low_freq: f64, high_freq: f64) ->ErbFiltersResult
    {
        let cf1 = calculate_uniform_center_freqs(low_freq, high_freq, num_channels);
        let mut cf = ComplexVec::new();
        for i in 0..cf1.len()
        {
            cf.push(Complex::new(cf1[i], 0.0));
        }
        let ear_q = 9.26449;  // Glasberg and Moore Parameters
        let min_bw: f64 = 24.7;
        let order = 1.0;

        let mut b = ComplexVec::new();
        let mut b1 = ComplexVec::new();

        let pi = std::f64::consts::PI;
        for i in &cf.v
        {
            let erb = ((i / ear_q).powf(order) + min_bw.powf(order)).powf(1.0 / order);
            b.push(1.019 * 2.0 * pi * erb);
        }
        let t = 1.0 / sample_rate as f64;
        
        let mut exp_bt = ComplexVec::new();
        for i in 0..b.len()
        {
            exp_bt.push((b[i] * t * -2.0).exp());
        }
        
        for i in 0..b.len().clone()
        {
            b1.push(-2.0 * (2.0 * cf[i] * pi * t).cos() / &exp_bt[i]);
        }
        let bt: Vec<Complex<f64>> = b.v.clone().into_iter().map(|x|{x * t}).collect();
        let bt = ComplexVec::new_from_complex_vector(&bt);
        let mut b_2 = ComplexVec::new();
        for i in 0..bt.len()
        {
            b_2.push((bt[i] * t * -2.0).exp());
        }

        let m = (((&cf * 2.0 * pi * t).sin()) * t);

        let b_1 = ComplexVec::from(&(((&cf * 2.0 * pi * t).sin()) * t));
        // debug here
        let b_pos = &b_1 * 2.0 * (3.0 + 2.0f64.powf(1.5)).sqrt();
        let b_neg = &b_1 * 2.0 * (3.0 + 2.0f64.powf(1.5)).sqrt();
        let a = ComplexVec::from(&(((&cf * 2.0 * pi * t).cos()) * 2.0 * t));

        let a_11 = -(&a / &exp_bt + &b_pos / &exp_bt) / 2.0;
        let a_12 = -(&a / &exp_bt - &b_pos / &exp_bt) / 2.0;
        let a_13 = -(&a / &exp_bt + &b_neg / &exp_bt) / 2.0;
        let a_14 = -(&a / &exp_bt - &b_neg / &exp_bt) / 2.0;

        // Setup gain variables
        let i = Complex::<f64>::new(0.0, 1.0);
        let p_1 = 2.0f64.powf(3.0 / 2.0);
        let s_1 = (3.0 - p_1).sqrt();
        let s_2 = (3.0 + p_1).sqrt();

        let x_exp = (&cf * 4.0 * i * pi * t).exp();
        let x_01 = &x_exp * -2.0 * t;
        let x_02 = (-(&b * t) +  &cf * 2.0 * i * pi * t).exp() * t * 2.0;
        let x_cos = (&cf * 2.0 * pi * t).cos();
        let x_sin = (&cf * 2.0 * pi * t).sin();

        // calculate gain
        let x_12 = &x_cos - &(&x_sin * s_1);
        let x_1 = &x_01 + &(&x_02 * &x_12);
        let x_22 = &x_cos + &(&x_sin * s_1);
        let x_2 = &x_01 + &(&x_02 * &x_22);
        let x_32 = &x_cos - &(&x_sin * s_2);
        let x_3 = &x_01 + &(&x_02 * &x_32);
        let x_42 = &x_cos + &(&x_sin * s_2);
        let x_4 = &x_01 + &(&x_02 * &x_42);

        let x_5 = ((&b * 2.0 * t).exp() / 2.0) - &x_exp * 2.0 + ((&x_exp + 1.0) * 2.0) / (&b * t).exp();
        
        let gain = ((&x_1 * &x_2 * &x_3 * &x_4) / (x_5.powf(4.0))).abs();

        let a_0 = vec![t;num_channels];
        let a_2 = vec![0.0;num_channels];
        let b_0 = vec![1.0;num_channels];

        let mut vf_coeffs = Vec::<Vec<f64>>::new();
        
        
        vf_coeffs.push(a_0);
        vf_coeffs.push(a_11.to_real_vector());
        vf_coeffs.push(a_12.to_real_vector());
        vf_coeffs.push(a_13.to_real_vector());
        vf_coeffs.push(a_14.to_real_vector());
        vf_coeffs.push(a_2);
        vf_coeffs.push(b_0);
        vf_coeffs.push(b_1.to_real_vector());
        vf_coeffs.push(b_2.to_real_vector());
        vf_coeffs.push(gain);

        ErbFiltersResult
        {
            center_freqs: cf.to_real_vector(),
            filter_coeffs: vf_coeffs
        }
    }

    // Returns vec<f64>
    fn calculate_uniform_center_freqs(low_freq: f64, high_freq: f64, num_channels: usize) -> Vec<f64>
    {
        // Glasberg and Moore Parameters
        let ear_q = 9.26449;
        let min_bandwidth = 24.7;

        let a = -(ear_q * min_bandwidth);
        let b = -((high_freq + ear_q * min_bandwidth).ln());
        let c = -((low_freq + ear_q * min_bandwidth).ln());
        let d = high_freq + ear_q * min_bandwidth;
        let e = (b + c) / num_channels as f64;
        let mut coefficients = Vec::<f64>::new();
        for i  in 0 .. num_channels
        {
            let f = ((i as f64 + 1.0) * e).exp() * d;
            coefficients.push(a + f);
        }
        coefficients
    }
    #[allow(unused)]
    pub struct ErbFiltersResult
    {
        filter_coeffs: Vec<Vec<f64>>,
        center_freqs: Vec<f64>
    }
}

