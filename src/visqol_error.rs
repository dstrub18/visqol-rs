use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum VisqolError
{
    #[error("Found invalid sample rate {sample_rate:?} detected")]
    InvalidSampleRate
    {
        sample_rate: u32
    },

    #[error("Sample rates differ! Reference signal is sampled at {reference:?} Hz, degraded signal is sampled at {degraded:?} Hz")]
    DifferentSampleRates
    {
        reference: u32,
        degraded: u32
    },
    
    #[error("Signal is too short! Found {found:?} samples, minimum required is {minimum_required:?}")]
    TooFewSamples
    {
        found: usize,
        minimum_required: usize
    },

    #[error("File not found")]
    FileNotFound(#[from] hound::Error),
    
    #[error("Could not compute prediction using SVM model")]
    PredictionError,
    
    #[error("Degraded file was too short, different, or misaligned to score any of the reference patches.")]
    SignalsTooDifferent
}
