use std::error::Error;

use crate::alignment::align_and_truncate;
use crate::{
    analysis_window::AnalysisWindow,
    audio_signal::AudioSignal,
    misc_audio,
    neurogram_similiarity_index_measure::NeurogramSimiliarityIndexMeasure,
    patch_similarity_comparator::{PatchSimilarityComparator, PatchSimilarityResult},
    spectrogram_builder::SpectrogramBuilder,
    visqol_error::VisqolError,
};
use log;
use ndarray::{concatenate, s, Array1, Array2, Axis};
pub struct ComparisonPatchesSelector {
    sim_comparator: NeurogramSimiliarityIndexMeasure,
}

impl ComparisonPatchesSelector {
    pub fn new(sim_comparator: NeurogramSimiliarityIndexMeasure) -> Self { Self { sim_comparator } }

    /// This function composes the most suitable patches in a degraded signal given a reference signal.
    pub fn find_most_optimal_deg_patches(
        &self,
        ref_patches: &mut [Array2<f64>],
        ref_patch_indices: &mut [usize],
        spectrogram_data: &Array2<f64>,
        frame_duration: f64,
        search_window_radius: i32,
    ) -> Result<Vec<PatchSimilarityResult>, VisqolError> {
        let num_frames_per_patch = ref_patches[0].ncols();
        let num_frames_in_deg_spectro = spectrogram_data.ncols();
        let patch_duration = frame_duration * num_frames_per_patch as f64;
        let search_window = search_window_radius * num_frames_per_patch as i32;
        let num_patches = Self::calc_max_num_patches(
            ref_patch_indices,
            num_frames_in_deg_spectro,
            num_frames_per_patch,
        );

        if num_patches == 0 {
            return Err(VisqolError::SignalsTooDifferent);
        } else if num_patches < ref_patch_indices.len() {
            log::warn!(
                "Warning: Dropping {} (of {}) reference patches 
            due to the degraded file being misaligned or too short. If too many 
            patches are dropped, the score will be less meaningful.",
                ref_patch_indices.len() - num_patches,
                ref_patch_indices.len()
            );
        }

        // The vector to store the similarity results
        let mut best_deg_patches = Vec::<PatchSimilarityResult>::new();
        best_deg_patches.resize(num_patches, PatchSimilarityResult::default());

        let mut cumulative_similarity_dp =
            vec![vec![0.0f64; spectrogram_data.ncols()]; ref_patch_indices.len()];
        let mut backtrace = vec![vec![0usize; spectrogram_data.ncols()]; ref_patch_indices.len()];

        let mut deg_patches = Vec::<Array2<f64>>::with_capacity(spectrogram_data.ncols());

        for slide_offset in 0..spectrogram_data.ncols() {
            deg_patches.push(Self::build_degraded_patch(
                spectrogram_data,
                slide_offset,
                slide_offset + ref_patches[0].ncols()
            ));
        }

        // Attempt to get a good alignment with backtracking.
        for (index, ref_patch) in ref_patches.iter_mut().enumerate() {
            self.find_most_optimal_deg_patch(
                spectrogram_data,
                ref_patch,
                &mut deg_patches,
                &mut cumulative_similarity_dp,
                &mut backtrace,
                ref_patch_indices,
                index,
                search_window,
            );
        }
        let mut max_similarity_score = f64::MIN;
        // The patch index for the last reference patch.
        let last_index = num_patches - 1;

        // The last_offset stores the offset at which the last reference patch got the
        // maximal similarity score over all the reference patches.

        let mut last_offset = 0;

        let lower_limit = 0.max(ref_patch_indices[last_index] as i32 - search_window) as usize;

        // The for loop is used to find the offset which maximizes the similarity
        // score across all the patches.
        // +1 for including last
        for slide_offset in lower_limit..ref_patch_indices[last_index] + search_window as usize + 1
        {
            if slide_offset >= num_frames_in_deg_spectro {
                // The frame offset for degraded start patch cannot be more than the
                // number of frames in the degraded spectrogram.
                break;
            }

            if cumulative_similarity_dp[last_index][slide_offset] > max_similarity_score {
                max_similarity_score = cumulative_similarity_dp[last_index][slide_offset];
                last_offset = slide_offset;
            }
        }

        let mut patch_index: i32 = (num_patches - 1) as i32;
        while patch_index >= 0 {
            // This sets the reference and degraded patch start and end times.
            let mut ref_patch = ref_patches[patch_index as usize].clone();

            let mut deg_patch = Self::build_degraded_patch(
                spectrogram_data,
                last_offset,
                last_offset + ref_patch.ncols()
            );

            best_deg_patches[patch_index as usize] = self
                .sim_comparator
                .measure_patch_similarity(&mut ref_patch, &mut deg_patch);

            // This condition is true only if no matching patch was found for the given
            // reference patch. In this case, the matched patch is essentially set to
            // NULL (which is different from a silent patch).

            if last_offset == backtrace[patch_index as usize][last_offset] {
                best_deg_patches[patch_index as usize].deg_patch_start_time = 0.0;
                best_deg_patches[patch_index as usize].deg_patch_end_time = 0.0;
                best_deg_patches[patch_index as usize].similarity = 0.0;
                let num_rows = best_deg_patches[patch_index as usize].freq_band_means.len();
                best_deg_patches[patch_index as usize].freq_band_means = vec![0.0; num_rows];
            } else {
                best_deg_patches[patch_index as usize].deg_patch_start_time =
                    last_offset as f64 * frame_duration;
                best_deg_patches[patch_index as usize].deg_patch_end_time =
                    best_deg_patches[patch_index as usize].deg_patch_start_time + patch_duration;
            }

            best_deg_patches[patch_index as usize].ref_patch_start_time =
                ref_patch_indices[patch_index as usize] as f64 * frame_duration;
            best_deg_patches[patch_index as usize].ref_patch_end_time =
                best_deg_patches[patch_index as usize].ref_patch_start_time + patch_duration;
            last_offset = backtrace[patch_index as usize][last_offset];

            patch_index -= 1;
        }
        Ok(best_deg_patches)
    }

    /// This function finds the most suitable patch in a degraded signal given a reference patch.
    pub fn find_most_optimal_deg_patch(
        &self,
        spectrogram_data: &Array2<f64>,
        ref_patch: &mut Array2<f64>,
        deg_patches: &mut [Array2<f64>],
        cumulative_similarity_dp: &mut [Vec<f64>],
        backtrace: &mut [Vec<usize>],
        ref_patch_indices: &[usize],
        patch_index: usize,
        search_window: i32,
    ) {
        let ref_frame_index = ref_patch_indices[patch_index];

        let mut sim_result;

        let mut slide_offset = ref_frame_index as i32 - search_window as i32;
        while slide_offset <= ref_frame_index as i32 + search_window as i32 {
            if slide_offset < 0 {
                // The degraded patch index cannot be less than 0.
                slide_offset = 0;
                continue;
            }

            if slide_offset == spectrogram_data.ncols() as i32 {
                // The start of the degraded is past the end of the spectrogram, so
                // nothing left to compare.

                break;
            }
            let deg_patch = &mut deg_patches[slide_offset as usize];
            sim_result = self
                .sim_comparator
                .measure_patch_similarity(ref_patch, deg_patch);
            let mut past_slide_offset = -1;
            let mut highest_sim = f64::MIN;

            if patch_index > 0 {
                // The lower_limit parameter tells us how far we should go
                // back to look for a possible match for the previous patch index
                // (patch_index - 1). The current value of lower_limit is used because the
                // search space for the previous patch index  is
                // (ref_patch_indices[patch_index - 1] - search_window,
                // ref_patch_indices[patch_index - 1] + search_window).
                let mut lower_limit: i32 =
                    ref_patch_indices[patch_index - 1] as i32 - search_window;
                lower_limit = lower_limit.max(0);
                // The back_offset parameter determines all the offsets that should be
                // considered while calculating the highest cumulative similarity score
                // achieved till patch_index - 1. Since two reference patches should
                // not map to the exact same degraded patch, the initial value of
                // back_offset is set to slide_offset - 1.
                let mut back_offset = slide_offset - 1;

                // The current for loop is used to find out the highest cumulative score
                // achieved till the previous ref_patch_index.
                while back_offset >= lower_limit {
                    if cumulative_similarity_dp[patch_index - 1][back_offset as usize] > highest_sim
                    {
                        highest_sim =
                            cumulative_similarity_dp[patch_index - 1][back_offset as usize];
                        past_slide_offset = back_offset;
                    }
                    back_offset -= 1;
                }

                sim_result.similarity += highest_sim;

                // If the current reference patch experienced a packet loss, then the
                // cumulative similarity score till the previous patch might be more and
                // in that case no matching patch for the current reference patch is found
                // in the degraded window.

                if cumulative_similarity_dp[patch_index - 1][slide_offset as usize]
                    > sim_result.similarity
                {
                    sim_result.similarity =
                        cumulative_similarity_dp[patch_index - 1][slide_offset as usize];
                    past_slide_offset = slide_offset;
                }
            }
            cumulative_similarity_dp[patch_index][slide_offset as usize] = sim_result.similarity;
            backtrace[patch_index][slide_offset as usize] = past_slide_offset as usize;
            slide_offset += 1;
        }
    }

    /// Calculate the maximum number of patches that the degraded spectrogram can support.
    pub fn calc_max_num_patches(
        ref_patch_indices: &[usize],
        num_frames_in_deg_spectro: usize,
        num_frames_per_patch: usize,
    ) -> usize {
        let mut num_patches = ref_patch_indices.len();

        if num_patches != 0 {
            while (ref_patch_indices[num_patches - 1] - (num_frames_per_patch / 2))
                > num_frames_in_deg_spectro
            {
                num_patches -= 1;
            }
        }
        num_patches
    }

    /// Given an `AudioSignal` and the desired start and end times in seconds, this function returns a copy of the segment in the audio signal ranging from `start_time` to `end_time`
    pub fn slice(in_signal: &AudioSignal, start_time: f64, end_time: f64) -> AudioSignal {
        let start_index = ((start_time * in_signal.sample_rate as f64) as usize).max(0);
        let end_index =
            ((end_time * in_signal.sample_rate as f64) as usize).min(in_signal.data_matrix.len());

        let mut sliced_matrix = in_signal
            .data_matrix
            .slice(s![start_index..end_index])
            .to_owned();
        let end_time_diff =
            (end_time * in_signal.sample_rate as f64 - in_signal.data_matrix.len() as f64) as usize;

        if end_time_diff > 0 {
            let post_silence_matrix = Array1::<f64>::zeros(end_time_diff);
            sliced_matrix =
                concatenate(Axis(0), &[sliced_matrix.view(), post_silence_matrix.view()])
                    .expect("Failed to zero-pad patch!");
        }

        if start_time < 0.0 {
            let pre_silence_matrix =
                Array1::<f64>::zeros((-1.0 * start_time * in_signal.sample_rate as f64) as usize);
            sliced_matrix =
                concatenate(Axis(0), &[pre_silence_matrix.view(), sliced_matrix.view()])
                    .expect("Failed to zero-pad patch!");
        }
        AudioSignal::new(
            sliced_matrix
                .as_slice()
                .expect("Failed to create AudioSignal from slice!"),
            in_signal.sample_rate,
        )
    }

    pub fn build_degraded_patch(
        spectrogram_data: &Array2<f64>,
        window_beginning: usize,
        window_end: usize
    ) -> Array2<f64> {
        let first_real_frame = 0.max(window_beginning);
        let last_real_frame = window_end.min(spectrogram_data.ncols());

        let mut deg_patch = spectrogram_data
            .slice(s![.., first_real_frame..last_real_frame])
            .to_owned();

        if window_end > spectrogram_data.ncols() {
            let append_matrix = Array2::<f64>::zeros((
                spectrogram_data.nrows(),
                window_end - spectrogram_data.ncols(),
            ));

            deg_patch = concatenate(Axis(1), &[deg_patch.view(), append_matrix.view()])
                .expect("Could not zero-pad patch!");
        }
        deg_patch
    }

    /// Performs alignment on a per-patch level.
    pub fn finely_align_and_recreate_patches(
        &self,
        sim_results: &mut [PatchSimilarityResult],
        ref_signal: &AudioSignal,
        deg_signal: &AudioSignal,
        spect_builder: &mut dyn SpectrogramBuilder,
        analysis_window: &AnalysisWindow,
    ) -> Result<Vec<PatchSimilarityResult>, Box<dyn Error>> {
        // Case: The patches are already matched.  Iterate over each pair.
        let mut realigned_results = Vec::<PatchSimilarityResult>::with_capacity(sim_results.len());
        realigned_results.resize(sim_results.len(), PatchSimilarityResult::default());
        for (i, result) in sim_results.iter_mut().enumerate() {
            if result.deg_patch_start_time == result.deg_patch_end_time
                && result.deg_patch_start_time == 0.0
            {
                realigned_results[i] = result.clone();
                continue;
            }
            // 1. The sim results keep track of the start and end points of each matched
            // pair.  Extract the audio for this segment.
            let ref_patch_audio = Self::slice(
                ref_signal,
                result.ref_patch_start_time,
                result.ref_patch_end_time,
            );
            let deg_patch_audio = Self::slice(
                deg_signal,
                result.deg_patch_start_time,
                result.deg_patch_end_time,
            );

            // 2. For any pair, we want to shift the degraded signal to be maximally
            // aligned.
            let (ref_audio_aligned, deg_audio_aligned, lag) =
                align_and_truncate(&ref_patch_audio, &deg_patch_audio)
                    .ok_or(VisqolError::FailedToAlignSignals)?;

            let new_ref_duration = ref_audio_aligned.get_duration();
            let new_deg_duration = deg_audio_aligned.get_duration();
            // 3. Compute a new spectrogram for the degraded audio.

            let mut ref_spectrogram = spect_builder.build(&ref_audio_aligned, analysis_window)?;
            let mut deg_spectrogram = spect_builder.build(&deg_audio_aligned, analysis_window)?;
            // 4. Recreate an aligned degraded patch from the new spectrogram.

            misc_audio::prepare_spectrograms_for_comparison(
                &mut ref_spectrogram,
                &mut deg_spectrogram,
            );
            // 5. Update the similarity result with the new patch.

            let mut new_sim_result = self
                .sim_comparator
                .measure_patch_similarity(&mut ref_spectrogram.data, &mut deg_spectrogram.data);
            // Compare to the old result and take the max.
            if new_sim_result.similarity < result.similarity {
                realigned_results[i] = result.clone();
            } else {
                if lag > 0.0 {
                    new_sim_result.ref_patch_start_time = result.ref_patch_start_time + lag;
                    new_sim_result.deg_patch_start_time = result.deg_patch_start_time;
                } else {
                    new_sim_result.ref_patch_start_time = result.ref_patch_start_time;
                    new_sim_result.deg_patch_start_time = result.deg_patch_start_time - lag;
                }
                new_sim_result.ref_patch_end_time =
                    new_sim_result.ref_patch_start_time + new_ref_duration;
                new_sim_result.deg_patch_end_time =
                    new_sim_result.deg_patch_start_time + new_deg_duration;
                realigned_results[i] = new_sim_result;
            }
        }
        Ok(realigned_results)
    }
}
