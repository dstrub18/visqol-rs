use crate::audio_signal::AudioSignal;
use crate::analysis_window::AnalysisWindow;
use crate::spectrogram::Spectrogram;
use crate::spectrogram_builder::SpectrogramBuilder;
use crate::gammatone_filterbank::GammatoneFilterbank;
use crate::equivalent_rectangular_bandwidth;
use crate::misc_vector::array2_to_vec;
use ndarray::{Array2, Axis};
pub struct GammatoneSpectrogramBuilder
{
    filter_bank: GammatoneFilterbank,
    speech_mode: bool
}

impl SpectrogramBuilder for GammatoneSpectrogramBuilder
{
    fn build(&mut self, signal: &AudioSignal, window: &AnalysisWindow) -> Result<Spectrogram, ()>
    {
        let sig = array2_to_vec(&signal.data_matrix);
        let sample_rate = signal.sample_rate;
        let max_freq = if self.speech_mode {Self::SPEECH_MODE_MAX_FREQ} else{ sample_rate / 2};
        
        // get gammatone coefficients
        let erb_result = equivalent_rectangular_bandwidth::make_filters(sample_rate as usize, self.filter_bank.num_bands, self.filter_bank.min_freq, max_freq as f64);
        let mut coeffs = erb_result.filter_coeffs.clone();
        coeffs.invert_axis(Axis(0));
        self.filter_bank.set_filter_coefficients(&coeffs);
        self.filter_bank.reset_filter_conditions();
        
        let hop_size = (window.size as f64 * window.overlap) as usize;
        debug_assert!(sig.len() > window.size, "too few samples!");
        
        let num_cols = 1 + ((sig.len() - window.size) / hop_size);
        let mut out_matrix = Array2::<f64>::zeros((self.filter_bank.num_bands, num_cols));
        for i in 0..out_matrix.ncols() 
        {
            // select the next frame from the input signal to filter.
            let start_col = i * hop_size;
            let frame = &sig[start_col .. start_col + window.size];
            self.filter_bank.reset_filter_conditions();
            
            let mut filtered_signal = self.filter_bank.apply_filter(frame);
            filtered_signal.iter_mut().for_each(|e|{*e = *e * *e});
            
            let mut row_means = filtered_signal.mean_axis(Axis(1)).unwrap();
            
            row_means.iter_mut().for_each(|e|{*e = e.sqrt();});
            
            for j in 0..row_means.to_vec().len()
            {
                out_matrix[(j, i)] = row_means[j];
            }
        }

        let mut ordered_cfb = erb_result.center_freqs.clone();
        ordered_cfb.as_mut_slice().sort_by(|a, b|{a.partial_cmp(b).unwrap()});
        Ok(Spectrogram::new(out_matrix, ordered_cfb))

    }
}

impl GammatoneSpectrogramBuilder
{
    pub const SPEECH_MODE_MAX_FREQ: u32 = 8000;
    pub fn new(filter_bank: GammatoneFilterbank, use_speech_mode: bool)
    -> Self     
    {
        Self
        {
            filter_bank,
            speech_mode: use_speech_mode
        }
    }
}