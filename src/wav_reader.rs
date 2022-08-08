use hound;

const _WAV_FORMAT_SIZE: usize = 24;
const _CHUNK_HEADER_SIZE_: usize =  8;
const _WAV_HEADER_SIZE: usize = 44;
const _EXTENSIBLE_WAV_FORMAT: u16 = 0xfffe;
const _PCM_FORMAT: u16 = 0x1;

pub struct WavReader
{
    pub num_channels: u16,
    pub sample_rate: u32,
    pub num_total_samples: u32,
    pub bytes_per_sample: u16,
    pub format: hound::SampleFormat,
    pub samples: Vec<i16>
}

impl WavReader
{
    pub fn open (file_name: &str) -> Self
    {
        let mut reader = hound::WavReader::open(file_name).expect("Failed to open Wav file!");
        let spec = reader.spec();
        
        let samples: Vec<i16> = reader.samples::<i16>().map(|x| x.expect("Failed to read samples")).collect();
        
        Self
        {
            num_channels: spec.channels,
            sample_rate: spec.sample_rate,
            num_total_samples: 0,
            bytes_per_sample: spec.bits_per_sample / 8,
            format: spec.sample_format,
            samples
        }
    }
}