use ndarray::{arr2, Axis};

#[test]
fn test_flipud() {
    let mut a = arr2(&[[1.0, 2.0], [3.0, 4.0]]);

    assert_eq!(a.shape(), [2, 2]);
    a.invert_axis(Axis(0));
    assert_eq!(a.shape(), [2, 2]);
    a[(0, 0)] = 3.0;
    a[(0, 1)] = 4.0;
    a[(1, 0)] = 1.0;
    a[(1, 1)] = 2.0;
}
