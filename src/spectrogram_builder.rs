use crate::{audio_signal::AudioSignal, analysis_window::AnalysisWindow, spectrogram::Spectrogram};
pub trait SpectrogramBuilder
{
    fn build(&self, signal: &AudioSignal, window: &AnalysisWindow) -> Result<Spectrogram, ()>;
}