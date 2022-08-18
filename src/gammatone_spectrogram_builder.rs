use crate::{audio_signal::AudioSignal, visqol_error::VisqolError};
use crate::analysis_window::AnalysisWindow;
use crate::spectrogram::Spectrogram;
use crate::spectrogram_builder::SpectrogramBuilder;
use crate::gammatone_filterbank::GammatoneFilterbank;
use crate::equivalent_rectangular_bandwidth;
use ndarray::{Array2, Axis};

//use rayon::prelude::*;
pub struct GammatoneSpectrogramBuilder
{
    filter_bank: GammatoneFilterbank,
    speech_mode: bool
}

impl SpectrogramBuilder for GammatoneSpectrogramBuilder
{
    fn build(&mut self, signal: &AudioSignal, window: &AnalysisWindow) -> Result<Spectrogram, VisqolError>
    {
        let sig = &signal.data_matrix.to_vec();
        let sample_rate = signal.sample_rate;
        let max_freq = if self.speech_mode {Self::SPEECH_MODE_MAX_FREQ} else {sample_rate / 2};
        
        // get gammatone coefficients
        let (mut filter_coeffs, mut center_freqs) = equivalent_rectangular_bandwidth::make_filters(sample_rate as usize, self.filter_bank.num_bands, self.filter_bank.min_freq, max_freq as f64);
        filter_coeffs.invert_axis(Axis(0));
        self.filter_bank.set_filter_coefficients(&filter_coeffs);
        self.filter_bank.reset_filter_conditions();
        
        let hop_size = (window.size as f64 * window.overlap) as usize;
        
        if sig.len() < window.size 
        {
            return Err(VisqolError::TooFewSamples { found: sig.len(), minimum_required: window.size }); 
        }
        
        let num_cols = 1 + ((sig.len() - window.size) / hop_size);
        let mut out_matrix = Array2::<f64>::zeros((self.filter_bank.num_bands, num_cols));

        for (i, frame) in sig.windows(window.size).step_by(hop_size).enumerate()
        {
            self.filter_bank.reset_filter_conditions();
            let mut filtered_signal = self.filter_bank.apply_filter(frame);
            
            filtered_signal.map_inplace(|e|{*e = *e * *e});
            
            let mut row_means = filtered_signal.mean_axis(Axis(1)).expect("Failed to compute for gammatone spectrogram!");
            
            row_means.map_inplace(|e|{*e = e.sqrt();});
            
            for j in 0..row_means.to_vec().len()
            {
                out_matrix[(j, i)] = row_means[j];
            }
        }

        center_freqs.as_mut_slice().sort_by(|a, b|{a.partial_cmp(b).expect("Failed to sort center frequencies!")});
        Ok(Spectrogram::new(out_matrix, center_freqs))
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