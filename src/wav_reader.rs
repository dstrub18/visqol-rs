use std::fs::File;
use std::io::{Seek, SeekFrom, Read};
use std::mem::{align_of, size_of};


const _WAV_FORMAT_SIZE: usize = 24;
const _CHUNK_HEADER_SIZE_: usize =  8;
const _WAV_HEADER_SIZE: usize = 44;
const _EXTENSIBLE_WAV_FORMAT: u16 = 0xfffe;
const _PCM_FORMAT: u16 = 0x1;

use num::Integer;
#[allow(dead_code)]
pub struct WavReader
{
    init: bool,
    num_channels: u16,
    sample_rate: u32,
    num_total_samples: u32,
    bytes_per_sample: u16,
    pub header: WavHeader
}

impl WavReader
{
    pub fn new (binary_stream: &mut File) -> Self
    {
        // Should contain "RIFF" 
        let mut riff_id= [0;4];
        let read_position = 0;
        let new_position = read_binary_data_from_stream(binary_stream, &mut riff_id, read_position);

        // Total size minus 8 bytes
        let mut header_size_buffer = [0;4];
        let new_position = read_binary_data_from_stream(binary_stream,&mut header_size_buffer,new_position);
        let header_size = u32::from_le_bytes(header_size_buffer);
        let riff_chunk_header = ChunkHeader{id: riff_id, size: header_size};
        
        // Should contain "WAVE"
        let mut wav_format_id = [0;4];
        let new_position = read_binary_data_from_stream(binary_stream, &mut wav_format_id, new_position);

        let riff = Riff{header: riff_chunk_header, format: wav_format_id};
        
        // New chunk!
        // Should contain "fmt "
        let mut subchunk_1_id = [0;4];
        let new_position = read_binary_data_from_stream(binary_stream, &mut subchunk_1_id, new_position);

        // If 16, then PCM
        let mut subchunk_1_size_buffer = [0;4];
        let new_position = read_binary_data_from_stream(binary_stream, &mut subchunk_1_size_buffer, new_position);
        
        let subchunk_1_size = u32::from_le_bytes(subchunk_1_size_buffer);
        
        
        // Extra Header stuff here!!
        //assert!(subchunk_1_size == 16);
        
        let subchunk_1_header = ChunkHeader{id: subchunk_1_id, size: subchunk_1_size};

        // Start format info
        // Should be 1 for PCM
        let mut audio_format_buffer = [0;2];
        let new_position = read_binary_data_from_stream(binary_stream, &mut audio_format_buffer, new_position);
        let audio_format = u16::from_le_bytes(audio_format_buffer);
        assert!(audio_format == 1);
        
        // Num channels
        let mut num_chan_buffer = [0;2];
        let new_position = read_binary_data_from_stream(binary_stream, &mut num_chan_buffer, new_position);
        let num_channels = u16::from_le_bytes(num_chan_buffer);
        //assert!(num_channels == 1);
        
        // Sample rate
        let mut sample_rate_buffer = [0;4];
        let new_position = read_binary_data_from_stream(binary_stream, &mut sample_rate_buffer, new_position);
        let sample_rate = u32::from_le_bytes(sample_rate_buffer);
        
        // Byte rate
        let mut byte_rate_buffer = [0;4];
        let new_position = read_binary_data_from_stream(binary_stream, &mut byte_rate_buffer, new_position);
        let byte_rate = u32::from_le_bytes(byte_rate_buffer);

        // Bit depth
        let mut bit_depth_buffer = [0;2];
        let new_position = read_binary_data_from_stream(binary_stream, &mut bit_depth_buffer, new_position);
        let bits_per_sample = u16::from_le_bytes(bit_depth_buffer);

        // WavFormat here
        let wav_format = WavFormat{
            header: subchunk_1_header,
            format_tag: audio_format,
            num_channels: num_channels,
            sample_rate: sample_rate,
            average_bytes_per_second: byte_rate,
            bits_per_sample: bits_per_sample
        };

        // Increment by 2 to get to subchunk 2. This is where additional info might be hidden.
        let new_position = new_position + 2;

        // New Subchunk!
        let mut subchunk_2_id = [0;4];
        let new_position = read_binary_data_from_stream(binary_stream, &mut subchunk_2_id, new_position);
        
        let mut subchunk_2_size_buffer = [0;4];
        let _new_position = read_binary_data_from_stream(binary_stream, &mut subchunk_2_size_buffer, new_position);


        // Samples * 2
        let num_audio_bytes = u32::from_le_bytes(subchunk_2_size_buffer);

        let data_header = Data
        {
            header: ChunkHeader
            {
                id: subchunk_2_id,
                size: num_audio_bytes 
            }
        };

        let wav_header = WavHeader
        {
            riff: riff,
            format: wav_format,
            data: data_header
        };

        Self
        {
            init: true,
            num_channels: num_channels,
            sample_rate: sample_rate,
            num_total_samples: num_audio_bytes / 2,
            bytes_per_sample: bits_per_sample / 8,
            header: wav_header
        }
    }


    #[allow(unused)]
    pub fn read_samples(&self, binary_stream: &mut File, buffer_to_write_to: &mut Vec<i16>, offset: u64) 
    {
        let num_bytes = self.num_total_samples * 2;
        binary_stream.seek(SeekFrom::Start(offset)).unwrap();
        assert!(!num_bytes.is_odd(), "Found odd number of bytes for 16 Bit data!");
        
        let mut temp_buffer  = [0;2];
        for i in (0..num_bytes).step_by(2)
        {
            let sp = binary_stream.stream_position().unwrap();
            binary_stream.seek(SeekFrom::Start(offset + i as u64));
            let sp = binary_stream.stream_position().unwrap();
            binary_stream.read_exact(&mut temp_buffer).unwrap();
            buffer_to_write_to.push(i16::from_le_bytes(temp_buffer));
        }
        assert!(buffer_to_write_to.len() as u32 == num_bytes / 2);
    }

}


#[allow(dead_code)]
pub struct WavFormat
{
    header: ChunkHeader,
    format_tag: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
    average_bytes_per_second: u32,
    bits_per_sample: u16,
}

#[allow(dead_code)]
pub struct WavHeader
{
    riff: Riff,
    pub format: WavFormat,
    data: Data
}

#[allow(dead_code)]
struct Data
{
    header: ChunkHeader
}

#[allow(dead_code)]
struct ChunkHeader
{
    id: [u8;4],
    size: u32
}

#[allow(dead_code)]
struct Riff
{
    header: ChunkHeader,
    format: [u8;4]
}


/*
    Read does not move cursor!
    Read does not consume file!
    Seek moves cursor!
*/

#[test]
fn test_seek()
{
    let mut buffer = [0 as u8;4];
    let mut file = File::open("input.txt").unwrap();
    file.read_exact(&mut buffer).unwrap();
    //let stream_position = file.stream_position();

    for letter in buffer
    {
        println!("{}", letter as char);
    }
    //println!("new position: {}", new_position);
    let stream_position = file.stream_position();
    println!("stream_position: {}", stream_position.unwrap());
}


#[test]
fn test_wav_header_size()
{
    println!("Wavheader size {}", size_of::<WavHeader>() );
    println!("Data size {}", size_of::<Data>() );
    println!("Riff size {}", size_of::<Riff>() );
    println!("ChunkHeader size {}", size_of::<ChunkHeader>() );
    println!("Wavformat size {}", size_of::<WavFormat>() );
    //assert_eq!(size_of::<ChunkHeader>(), 8);
    
    println!("-------ALIGNOF--------");
    println!("Wavheader size {}", align_of::<WavHeader>() );
    println!("Data size {}", align_of::<Data>() );
    println!("Riff size {}", align_of::<Riff>() );
    println!("ChunkHeader size {}", align_of::<ChunkHeader>() );
    println!("Wavformat size {}", align_of::<WavFormat>() );
    //assert_eq!(size_of::<ChunkHeader>(), 8);
}


#[allow(unused)]
fn get_count_of_bytes_in_stream(binary_stream: &mut File) -> u64
{
    let count = binary_stream.seek(SeekFrom::End(0)).unwrap();
    binary_stream.seek(SeekFrom::Start(0));
    assert!(binary_stream.stream_position().unwrap() == 0);
    count
}

// Reads data and returns new read position.
#[allow(unused)]
fn read_binary_data_from_stream(binary_stream: &mut File, buffer_to_write_to: &mut [u8], offset: u64) -> u64
{
    let read_position = binary_stream.seek(SeekFrom::Start(offset)).unwrap();
    binary_stream.read_exact(buffer_to_write_to).unwrap();
    let new_position = binary_stream.stream_position().unwrap();
    //let new_position = binary_stream.seek(SeekFrom::Current(offset as i64)).unwrap();
    //println!("{}", new_position);
    new_position
}

// Does not modify read position
#[allow(unused)]
fn get_remaining_samples_to_read(binary_stream: &mut File, offset: u64) -> u64
{
    let start_position = binary_stream.seek(SeekFrom::Start(offset)).unwrap();
    let end_position = binary_stream.seek(SeekFrom::End(0)).unwrap();
    binary_stream.seek(SeekFrom::Start(0)).unwrap();
    end_position - start_position
}

