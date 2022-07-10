#[test]
fn test_slice_windowing()
{   
    let ten_samples = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    let expected_windows =  [[0.1, 0.2, 0.3, 0.4],
                                            [0.3, 0.4, 0.5, 0.6],
                                            [0.5, 0.6, 0.7, 0.8],
                                            [0.7, 0.8, 0.9, 1.0]];
    for (outer_index, chunk) in ten_samples.windows(4).step_by(2).enumerate()
    {
        for i in 0..chunk.len()
        {
            assert_eq!(chunk[i], expected_windows[outer_index][i]);
        }
    }
}