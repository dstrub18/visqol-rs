use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum VisqolError {
    #[error("Sample rates differ! Reference signal is sampled at {reference:?} Hz, degraded signal is sampled at {degraded:?} Hz")]
    DifferentSampleRates { reference: u32, degraded: u32 },

    #[error(
        "Signal is too short! Found {found:?} samples, minimum required is {minimum_required:?}"
    )]
    TooFewSamples {
        found: usize,
        minimum_required: usize,
    },

    #[error(
        "reference spectrum size {spectrogram_length:?} smaller than minimum {minimum_required:?}"
    )]
    ReferenceSpectrogramTooSmall {
        spectrogram_length: usize,
        minimum_required: usize,
    },

    #[error("Degraded file was too short, different, or misaligned to score any of the reference patches.")]
    SignalsTooDifferent,

    #[error("Failed to align signals!")]
    FailedToAlignSignals,

    #[error("Failed to compute VAD!")]
    FailedToComputeVad,

    #[error("Visqol input files must be quantized to 16 bit. Found {bits_per_sample:?}!")]
    InvalidBitsPerSample { bits_per_sample: u16 },
}
