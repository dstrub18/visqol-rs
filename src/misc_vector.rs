use num::Zero;
use ndarray::Array2;
pub fn array2_to_vec<T>(arr: &Array2<T>)
-> Vec<T> 
where T: Clone + Zero + Copy
{
    assert!(arr.dim().1 == 1, "Array needs to have 1 column!");

    let mut v = vec![T::zero(); arr.dim().0];
    for i in 0..arr.dim().0
    {
        v[i] = arr[(i, 0)];
    }
    v
}