use crate::{
    analysis_window::AnalysisWindow, audio_signal::AudioSignal, spectrogram::Spectrogram,
    visqol_error::VisqolError,
};

/// Given a time domain signal and an analysis window, structs implementing this trait can build a spectrogram representing the signal in the frequency domain.
pub trait SpectrogramBuilder {
    fn build(
        &mut self,
        signal: &AudioSignal,
        window: &AnalysisWindow,
    ) -> Result<Spectrogram, VisqolError>;
}
