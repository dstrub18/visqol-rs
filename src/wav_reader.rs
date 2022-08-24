use hound::WavReader;
use std::error::Error;

/// Represents the metadata and contents of a wav file.
/// Simple wrapper around the `hound` library.
pub struct WavFile {
    /// The number of channels in the wav file
    pub num_channels: u16,
    /// The sample rate of the wav file
    pub sample_rate: u32,
    /// The samples in the wav file. Note that these are not scaled from -1.0 to 1.0 but its integer values.
    pub samples: Vec<i16>,
}

impl WavFile {
    /// given a `file_path` to the desired wav file, the contents of the wav file are returned.
    /// Any possible errors are reported by `hound`.
    pub fn open(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut reader = WavReader::open(file_path)?;
        let spec = reader.spec();

        let samples: Vec<i16> = reader
            .samples::<i16>()
            .map(|x| x.expect("Failed to read samples"))
            .collect();

        Ok(Self {
            num_channels: spec.channels,
            sample_rate: spec.sample_rate,
            samples,
        })
    }
}
