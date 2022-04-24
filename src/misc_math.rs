use ndarray::{Array2};
use itertools::Itertools;
#[allow(unused)]
pub fn normalize_2d_matrix(mat: &mut Array2<f64>) -> Array2<f64>
{
    let mut normalized_mat = mat.clone();
    let max = get_max_in_2d_array(mat);
    
    for i in 0..mat.shape()[0]
    {
        for j in 0..mat.shape()[1]
        {
            normalized_mat[(i,j)] /= max;
        }
    }
    normalized_mat
}

#[allow(unused)]
pub fn next_pow_two(input: &usize) -> usize
{
    let mut number = input - 1;

    number |= number >> 1;
    number |= number >> 2;
    number |= number >> 4;
    number |= number >> 1;
    number |= number >> 16;
    number + 1
}

pub fn exponential_from_fit(x: f64, a: f64, b: f64, x_0: f64) -> f64
{
    a + (b * (x - x_0)).exp()
}
#[allow(unused)]
fn abs_scale(x: i16) -> f64
{
    return (x as f64) / 32768.0f64
}

pub fn normalize_int16_to_double(input: &Vec<i16>) -> Vec<f64>
{
    input.iter().map(|x| *x as f64 / 32768.0f64).collect::<Vec<f64>>()
}

pub fn get_max_in_2d_array(mat: &Array2<f64>) -> f64
{
    *(mat.iter().minmax().into_option().unwrap().1)
}