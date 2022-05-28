use visqol_rs::rms_vad::RmsVad;
    #[test]
    fn rms_vad_test_short_sequence()
    {
        let k_chunk = vec![186, 236, 44, -152, -155, -2, 66, 5, -108,
        -107, 14, 141, 151, 31, -90];

        let mut rms_vad = RmsVad::default();
        let result = rms_vad.process_chunk(&k_chunk);
        let k_chunk_rms_expected_result = 120.7736;
        let tolerance  = 0.001;

        assert!((k_chunk_rms_expected_result - result).abs() < tolerance);
    }

    #[test]
    fn rms_vad_test_long_sequence()
    {
        let signal = vec![10000, 10000, 10000, 10000, 10000,
                            10, 10, 10, 10, 10,
                            10000, 10000, 10000, 10000, 10000,
                            10000, 10000, 10000, 10000, 10000,
                            10000, 10000, 10000, 10000, 10000,
                            10, 10, 10, 10, 10,
                            10, 10, 10, 10, 10,
                            10000, 10000, 10000, 10000, 10000,
                            10000, 10000, 10000, 10000, 10000,
                            10, 10, 10, 10, 10,
                            10, 10, 10, 10, 10,
                            10, 10, 10, 10, 10,
                            10000, 10000, 10000, 10000, 10000,
                            10, 10, 10, 10, 10,
                            10, 10, 10, 10, 10,
                            10, 10, 10, 10, 10,
                            10, 10, 10, 10, 10,
                            10000, 10000, 10000, 10000, 10000];

                            
        let mut rms_vad = RmsVad::default();

        let signal_chunk_size = 5;
        
        let expected_result = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0];
        
        for chunk in signal[..].chunks(signal_chunk_size)
        {
            rms_vad.process_chunk(&chunk.to_vec());
        }
        let result = rms_vad.get_vad_results();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn low_signal_at_start()
    {
        let signal_low_start = vec![10, 10, 10, 10, 10,
                                    10, 10, 10, 10, 10,
                                    10, 10, 10, 10, 10,
                                    10, 10, 10, 10, 10,
                                    10000, 10000, 10000, 10000, 10000];
        
        let expected_result = vec![1.0, 1.0, 0.0, 0.0, 1.0];

        let signal_chunk_size = 5;

        let mut rms_vad = RmsVad::default();
        for chunk in signal_low_start[..].chunks(signal_chunk_size)
        {
            rms_vad.process_chunk(&chunk.to_vec());
        }
        let result = rms_vad.get_vad_results();
        assert_eq!(result, expected_result);
    }
