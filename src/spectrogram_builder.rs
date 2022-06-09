use crate::{audio_signal::AudioSignal, analysis_window::AnalysisWindow, spectrogram::Spectrogram};
pub trait SpectrogramBuilder
{
    fn build(&mut self, signal: &AudioSignal, window: &AnalysisWindow) -> Result<Spectrogram, ()>;
}