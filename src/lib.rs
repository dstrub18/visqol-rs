//! An implementation of the `Virtual Speech Quality Objective Listener(ViSQOL)` algorithm, an objective, full-reference metric for perceived audio quality.
//! 
//! # Example
//! 
//! ```ignore
//! use visqol_rs::*;
//!     
//! let path_to_reference_file = "./test_data/clean_speech/reference_signal.wav";
//! let path_to_degraded_file = "./test_data/clean_speech/degraded_signal.wav";
//! 
//! let config = visqol_config::VisqolConfig::get_speech_mode_config();
//! 
//! let mut visqol = visqol_manager::VisqolManager::from_config(&config);
//! 
//! let similarity_result = visqol.run(path_to_reference_file, path_to_degraded_file).unwrap();
//! 
//! println!("Mean objective score for degraded file {}: {}", path_to_degraded_file, similarity_result.moslqo);
//! ```
//! 

mod alignment;
mod analysis_window;
mod audio_signal;
mod comparison_patches_selector;    
mod constants;
mod convolution_2d;
mod envelope;
mod equivalent_rectangular_bandwidth;
mod fast_fourier_transform;
mod fft_manager;
mod gammatone_filterbank;
mod gammatone_spectrogram_builder;
mod image_patch_creator;
mod math_utils;
mod neurogram_similiarity_index_measure;
pub mod command_line_utils;
pub mod audio_utils;
pub mod output_utils;
pub mod similarity_result;
pub mod visqol_manager;
pub mod visqol_config;
mod patch_creator;
mod patch_similarity_comparator;
mod path_pair;
mod rms_vad;
mod signal_filter;
mod similarity_to_quality_mapper;
mod spectrogram;
mod spectrogram_builder;
mod speech_similarity_to_quality_mapper;
mod support_vector_regression_model;
mod svr_similarity_to_quality_mapper;
mod tflite_quality_mapper;
mod vad_patch_creator;
mod visqol;
mod visqol_error;
mod wav_reader;
mod xcorr;

#[cfg(test)]
mod test_utility;
