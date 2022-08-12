use visqol_rs::gammatone_filterbank::GammatoneFilterbank;
use visqol_rs::analysis_window::AnalysisWindow;
use visqol_rs::gammatone_spectrogram_builder::GammatoneSpectrogramBuilder;
use visqol_rs::misc_audio;
use visqol_rs::spectrogram_builder::SpectrogramBuilder;
use approx::assert_abs_diff_eq;
#[test]
fn test_spec_builder()
{
    // Fixed parameters
    const MINIMUM_FREQ:f64 = 50.0;
    const NUM_BANDS:usize = 32;
    const OVERLAP:f64 = 0.25;

    const REF_SPECTRO_NUM_COLS: usize = 802;
    
    let signal_ref = misc_audio::load_as_mono("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/conformance_testdata_subset/contrabassoon48_stereo.wav").unwrap();
    let filter_bank = GammatoneFilterbank::new(NUM_BANDS, MINIMUM_FREQ);
    let window = AnalysisWindow::new(signal_ref.sample_rate, OVERLAP, 0.08);

    let mut spectro_builder = GammatoneSpectrogramBuilder::new(filter_bank, false);
    let spectrogram_ref = spectro_builder.build(&signal_ref, &window).unwrap();
    
    // Check 1st element
    assert_abs_diff_eq!(spectrogram_ref.data[(0,0)], 9.44161e-05, epsilon=0.00001);
    // Check dimensions
    assert_eq!(spectrogram_ref.data.ncols(), REF_SPECTRO_NUM_COLS);
}
