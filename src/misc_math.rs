use ndarray::Array1;
use ndarray_stats::QuantileExt;
pub fn normalize_signal(mat: &Array1<f64>) -> Array1<f64>
{
    let normalized_mat = mat.clone();
    let max = get_max(mat);
    normalized_mat / max
}

pub fn next_pow_two(input: usize) -> usize
{
    let mut number = input - 1;

    number |= number >> 1;
    number |= number >> 2;
    number |= number >> 4;
    number |= number >> 1;
    number |= number >> 16;
    number + 1
}

pub fn exponential_from_fit(x: f32, a: f32, b: f32, x_0: f32) -> f32
{
    a + (b * (x - x_0)).exp()
}


pub fn normalize_int16_to_double(input: &[i16]) -> Vec<f64>
{
    input.iter().map(|x| *x as f64 / 32767.0f64).collect::<Vec<f64>>()
}

fn get_max(mat: &Array1<f64>) -> f64
{
    *mat.max().expect("Could not compute maximum of matrix!")
}