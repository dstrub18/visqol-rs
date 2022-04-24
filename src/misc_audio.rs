use crate::{audio_signal::AudioSignal};
use crate::{wav_reader::WavReader};
use crate::misc_math;
use crate::spectrogram::Spectrogram;
use ndarray::{Array2, ShapeBuilder};
use num::complex::Complex64;

// Constants
const NUM_CHANNELS_MONO: usize = 1;
const SPL_REFERENCE_POINT: f64 = 0.00002;
const NOISE_FLOOR_RELATIVE_TO_PEAK_DB: f64 = 45.0;
const NOISE_FLOOR_ABSOLUTE_DB: f64 = -45.0;

pub fn scale_to_match_sound_pressure_level(reference: &AudioSignal, degraded: &AudioSignal) -> AudioSignal
{
    let ref_spl = calculate_sound_pressure_level(reference);
    let deg_spl = calculate_sound_pressure_level(degraded);
    let scale_factor = 10.0f64.powf((ref_spl - deg_spl) / 20.0);
    let scaled_mat = degraded.data_matrix.clone() * scale_factor;
    AudioSignal{data_matrix: scaled_mat,sample_rate:  degraded.sample_rate}
}

pub fn calculate_sound_pressure_level(signal: &AudioSignal) -> f64
{
    let energy: f64 = signal.data_matrix.iter().map(|element| {element.powi(2)}).sum();
    let sound_pressure = energy.sqrt();
    20.0 * (sound_pressure / SPL_REFERENCE_POINT).log10()
}

pub fn to_mono_matrix(sample_matrix: &Array2::<f64>) ->Array2::<f64>
{
    if sample_matrix.ncols() >= NUM_CHANNELS_MONO
    {
    let num_rows = sample_matrix.nrows();
    let num_cols = sample_matrix.ncols();

    let mut mono_matrix = Array2::<f64>::zeros((num_rows, 1));
    
    for i_chan  in 0..num_cols
    {
        for i_sample in 0..num_rows
        {
            mono_matrix[(i_sample, 0)] += sample_matrix[(i_sample, i_chan)];
        }
    }
    mono_matrix /= num_cols as f64;
    
    mono_matrix
    }
    else 
    {
        // return if already mono.
        sample_matrix.clone()
    }
}

pub fn to_mono(signal: &AudioSignal) -> AudioSignal
{
    let sample_rate = signal.sample_rate;
    let data_to_monoize = signal.data_matrix.clone();
    if signal.data_matrix.ncols() >= NUM_CHANNELS_MONO
    {
        AudioSignal
        {   
            data_matrix: to_mono_matrix(&data_to_monoize), 
            sample_rate 
        }
    }
    else 
    {
        AudioSignal
        {   
            data_matrix: data_to_monoize, 
            sample_rate 
        }
    }
}

pub fn load_as_mono(file_path: String) -> AudioSignal
{
    let mut wav_file = std::fs::File::open(file_path).expect("File not found!");
    let wav_reader = WavReader::new(&mut wav_file);

    let mut data_vector = Vec::<i16>::new();
    wav_reader.read_samples(&mut wav_file, &mut data_vector, 44);
    let data_vector_float = misc_math::normalize_int16_to_double(&data_vector);
    let final_signal = extract_multichannel(wav_reader.header.format.num_channels as usize, &data_vector_float);

    let signal =  AudioSignal
    {
        data_matrix: final_signal,
        sample_rate: wav_reader.header.format.sample_rate
    };
    to_mono(&signal)
}

pub fn extract_multichannel(num_channels: usize, interleaved_vector: &Vec<f64>) -> Array2<f64>
{
    assert!(interleaved_vector.len() % num_channels as usize == 0);
    let sub_vector_size = interleaved_vector.len() / num_channels as usize;
    Array2::from_shape_vec((sub_vector_size, num_channels).strides((1, num_channels)), interleaved_vector.clone()).unwrap()
}

pub fn prepare_spectrograms_for_comparison(reference: &mut Spectrogram, degraded: &mut Spectrogram)
{
    reference.convert_to_db();
    degraded.convert_to_db();

    reference.raise_floor(NOISE_FLOOR_ABSOLUTE_DB);
    degraded.raise_floor(NOISE_FLOOR_ABSOLUTE_DB);

    reference.raise_floor_per_frame(NOISE_FLOOR_RELATIVE_TO_PEAK_DB, degraded);

    let ref_floor = reference.get_minimum();
    let deg_floor = reference.get_minimum();
    let lowest_floor = ref_floor.min(deg_floor);

    reference.raise_floor(lowest_floor);
    degraded.raise_floor(lowest_floor);
}

pub fn mirror_spectrum(spectrum: &mut Vec<Complex64>)
{
    let nyquist_bin = Complex64::new(spectrum.last().unwrap().re.clone(), 0.0);
    let zero_hz_bin = Complex64::new(spectrum[0].re.clone(), 0.0);
    
    spectrum.pop();
    spectrum.remove(0);

    let mut mirrored_spectrum = spectrum.clone();
    mirrored_spectrum.reverse();
    mirrored_spectrum.iter_mut().for_each(|element|{element.im = -1.0 * element.im;});
    spectrum.push(nyquist_bin);
    spectrum.extend(mirrored_spectrum);
    spectrum.insert(0, zero_hz_bin);
}