use ndarray::Array1;
use std::error::Error;

use crate::{
    analysis_window::AnalysisWindow, audio_signal::AudioSignal, audio_utils,
    comparison_patches_selector::ComparisonPatchesSelector, patch_creator::PatchCreator,
    patch_similarity_comparator::PatchSimilarityResult, similarity_result::SimilarityResult,
    similarity_to_quality_mapper::SimilarityToQualityMapper,
    spectrogram_builder::SpectrogramBuilder,
};

/// Perform a comparison on two audio signals. Their similarity is calculated
/// and converted to a quality score using the given similarity to quality
/// mapper.
pub fn calculate_similarity(
    ref_signal: &mut AudioSignal,
    deg_signal: &mut AudioSignal,
    spect_builder: &mut dyn SpectrogramBuilder,
    window: &AnalysisWindow,
    patch_creator: &dyn PatchCreator,
    selector: &ComparisonPatchesSelector,
    sim_to_qual_mapper: &dyn SimilarityToQualityMapper,
    search_window: usize,
) -> Result<SimilarityResult, Box<dyn Error>> {
    /////////////////// Stage 1: Preprocessing ///////////////////
    let deg_signal_scaled =
        audio_utils::scale_to_match_sound_pressure_level(ref_signal, deg_signal);
    let mut ref_spectrogram = spect_builder.build(ref_signal, window)?;
    let mut deg_spectrogram = spect_builder.build(&deg_signal_scaled, window)?;

    audio_utils::prepare_spectrograms_for_comparison(&mut ref_spectrogram, &mut deg_spectrogram);

    /////////////// Stage 2: Feature selection and similarity measure ////////////
    let mut ref_patch_indices =
        patch_creator.create_ref_patch_indices(&ref_spectrogram.data, ref_signal, window)?;

    let frame_duration = calculate_frame_duration(
        window.size as f64 * window.overlap,
        ref_signal.sample_rate as usize,
    );

    let mut ref_patches =
        patch_creator.create_patches_from_indices(&ref_spectrogram.data, &ref_patch_indices);

    let mut sim_match_info = selector.find_most_optimal_deg_patches(
        &mut ref_patches,
        &mut ref_patch_indices,
        &deg_spectrogram.data,
        frame_duration,
        search_window as i32,
    )?;
    // Realign the patches in time domain subsignals that start at the coarse
    // patch times.

    let realign_result = selector.finely_align_and_recreate_patches(
        &mut sim_match_info,
        ref_signal,
        &deg_signal_scaled,
        spect_builder,
        window,
    )?;
    sim_match_info = realign_result;

    let fvnsim = calc_per_patch_mean_freq_band_means(&sim_match_info);
    let fstdnsim = calc_per_patch_mean_freq_band_std_devs(&sim_match_info, frame_duration);
    let fvdegenergy = calc_per_patch_mean_freq_band_degraded_energy(&sim_match_info);

    let mut moslqo = predict_mos(&fvnsim.to_vec(), sim_to_qual_mapper);

    let vnsim = fvnsim.mean().expect("Failed to compute nsim mean");

    moslqo = alter_for_similarity_extremes(vnsim, moslqo as f64) as f32;
    Ok(SimilarityResult::new(
        moslqo as f64,
        vnsim,
        fvnsim.to_vec(),
        fstdnsim.to_vec(),
        fvdegenergy.to_vec(),
        ref_spectrogram.center_freq_bands,
        sim_match_info,
    ))
}

/// Computes prediction with the given `SimilarityToQualityMapper`
fn predict_mos(fvnsim: &[f64], mapper: &dyn SimilarityToQualityMapper) -> f32 {
    mapper.predict_quality(fvnsim)
}

/// Calculates the mean across all patch similarity per frequency band
fn calc_per_patch_mean_freq_band_means(sim_match_info: &Vec<PatchSimilarityResult>) -> Array1<f64> {
    let mut fvnsim = Array1::<f64>::zeros(sim_match_info[0].freq_band_means.len());
    for patch in sim_match_info {
        for (index, band) in fvnsim.iter_mut().enumerate() {
            *band += patch.freq_band_means[index];
        }
    }
    fvnsim / sim_match_info.len() as f64
}

/// Calculates the energy of the degraded patch across all patch similarity per frequency band
fn calc_per_patch_mean_freq_band_degraded_energy(
    sim_match_info: &Vec<PatchSimilarityResult>,
) -> Array1<f64> {
    let mut total_fvdegenergy = Array1::<f64>::zeros(sim_match_info[0].freq_band_means.len());
    for patch in sim_match_info {
        for (index, band) in total_fvdegenergy.iter_mut().enumerate() {
            *band += patch.freq_band_deg_energy[index];
        }
    }
    total_fvdegenergy / sim_match_info.len() as f64
}

/// Calculates the standard deviation across all patch similarity per frequency band
fn calc_per_patch_mean_freq_band_std_devs(
    sim_match_info: &Vec<PatchSimilarityResult>,
    frame_duration: f64,
) -> Array1<f64> {
    let fvn_sim = calc_per_patch_mean_freq_band_means(sim_match_info);

    let mut contribution = Array1::<f64>::zeros(sim_match_info[0].freq_band_means.len());
    // Now that we have the global mean, we can compute the combined
    // variance/stddev.
    let mut total_frame_count = 0;

    for patch in sim_match_info {
        let secs_in_patch = patch.ref_patch_end_time - patch.ref_patch_start_time;

        let frame_count = (secs_in_patch / frame_duration).ceil() as usize;
        total_frame_count += frame_count;

        for (index, contributing_element) in contribution.iter_mut().enumerate() {
            let dev = patch.freq_band_stddevs[index];
            let mean = patch.freq_band_means[index];

            *contributing_element += (frame_count - 1) as f64 * dev * dev;
            *contributing_element += frame_count as f64 * mean * mean;
        }
    }

    let mut result = (&contribution - (&fvn_sim * &fvn_sim * total_frame_count as f64))
        / (total_frame_count as f64 - 1.0);

    result.map_inplace(|element| {
        *element = if *element < 0.0 { 0.0 } else { element.sqrt() };
    });
    result
}

/// Clamps the MOS to 1.0 in case the files are too dissimilar
fn alter_for_similarity_extremes(vnsim: f64, moslqo: f64) -> f64 {
    if vnsim < 0.15 {
        1.0
    } else {
        moslqo
    }
}

/// Calculates fraeme duration in seonds
fn calculate_frame_duration(frame_size: f64, sample_rate: usize) -> f64 {
    frame_size / sample_rate as f64
}
