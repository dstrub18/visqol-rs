use crate::{
    analysis_window::AnalysisWindow, audio_signal::AudioSignal, spectrogram::Spectrogram,
    visqol_error::VisqolError,
};
pub trait SpectrogramBuilder {
    fn build(
        &mut self,
        signal: &AudioSignal,
        window: &AnalysisWindow,
    ) -> Result<Spectrogram, VisqolError>;
}
