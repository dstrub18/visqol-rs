use std::error::Error;

use crate::{
    alignment, analysis_window::AnalysisWindow, audio_signal::AudioSignal, audio_utils,
    comparison_patches_selector::ComparisonPatchesSelector, constants,
    gammatone_filterbank::GammatoneFilterbank,
    gammatone_spectrogram_builder::GammatoneSpectrogramBuilder,
    image_patch_creator::ImagePatchCreator,
    neurogram_similiarity_index_measure::NeurogramSimiliarityIndexMeasure,
    patch_creator::PatchCreator, similarity_result::SimilarityResult,
    similarity_to_quality_mapper::SimilarityToQualityMapper,
    spectrogram_builder::SpectrogramBuilder,
    speech_similarity_to_quality_mapper::SpeechSimilarityToQualityMapper,
    svr_similarity_to_quality_mapper::SvrSimilarityToQualityMapper,
    vad_patch_creator::VadPatchCreator, visqol, visqol_config::VisqolConfig,
    visqol_error::VisqolError,
};
use log;

/// Configures and executes audio evaluation using ViSQOL.
pub struct VisqolManager {
    search_window: usize,
    patch_creator: Box<dyn PatchCreator>,
    patch_selector: ComparisonPatchesSelector,
    spectrogram_builder: Box<dyn SpectrogramBuilder>,
    sim_to_quality_mapper: Box<dyn SimilarityToQualityMapper>,
}

impl VisqolManager {
    pub fn from_config(config: &VisqolConfig) -> Self {
        Self::new(
            &config.similarity_to_quality_model_path,
            config.use_speech_mode,
            config.use_unscaled_speech_mos_mapping,
            config.search_window,
        )
    }
    /// Creates a new instance of with the desired configurations.
    pub fn new(
        model_path: &str,
        use_speech_mode: bool,
        use_unscaled_speech_mos_mapping: bool,
        search_window: usize,
    ) -> Self {
        let patch_creator: Box<dyn PatchCreator> = if use_speech_mode {
            Box::new(VadPatchCreator::new(constants::PATCH_SIZE_SPEECH))
        } else {
            Box::new(ImagePatchCreator::new(constants::PATCH_SIZE_AUDIO))
        };

        let sim_to_quality_mapper: Box<dyn SimilarityToQualityMapper> = if use_speech_mode {
            Box::new(SpeechSimilarityToQualityMapper::new(
                !use_unscaled_speech_mos_mapping,
            ))
        } else {
            Box::new(SvrSimilarityToQualityMapper::new(model_path))
        };

        let spectrogram_builder: Box<dyn SpectrogramBuilder> = if use_speech_mode {
            Box::new(GammatoneSpectrogramBuilder::new(
                GammatoneFilterbank::new::<{ constants::NUM_BANDS_SPEECH }>(
                    constants::MINIMUM_FREQ,
                ),
                use_speech_mode,
            ))
        } else {
            Box::new(GammatoneSpectrogramBuilder::new(
                GammatoneFilterbank::new::<{ constants::NUM_BANDS_AUDIO }>(constants::MINIMUM_FREQ),
                false,
            ))
        };

        let patch_selector =
            ComparisonPatchesSelector::new(NeurogramSimiliarityIndexMeasure::default());

        Self {
            search_window,
            patch_creator,
            patch_selector,
            spectrogram_builder,
            sim_to_quality_mapper,
        }
    }

    /// Loads the audio store in `ref_signal_path` and `deg_signal_path` and computes its MOS.
    pub fn run(
        &mut self,
        ref_signal_path: &str,
        deg_signal_path: &str,
    ) -> Result<SimilarityResult, Box<dyn Error>> {
        let mut ref_signal = audio_utils::load_as_mono(ref_signal_path)?;
        let mut deg_signal = audio_utils::load_as_mono(deg_signal_path)?;

        self.validate_input_audio(&ref_signal, &deg_signal)?;

        self.compute_results(&mut ref_signal, &mut deg_signal)
    }

    pub fn compute_results(
        &mut self,
        ref_signal: &mut AudioSignal,
        deg_signal: &mut AudioSignal,
    ) -> Result<SimilarityResult, Box<dyn Error>> {
        let (mut deg_signal, _) = alignment::globally_align(ref_signal, deg_signal)
            .ok_or(VisqolError::FailedToAlignSignals)?;

        let window = AnalysisWindow::new(
            ref_signal.sample_rate,
            constants::OVERLAP,
            constants::WINDOW_DURATION,
        );

        visqol::calculate_similarity(
            ref_signal,
            &mut deg_signal,
            self.spectrogram_builder.as_mut(),
            &window,
            self.patch_creator.as_mut(),
            &self.patch_selector,
            self.sim_to_quality_mapper.as_mut(),
            self.search_window,
        )
    }

    /// Performs sanity checks on the configuration to prevent incorrect use of the algorithm.
    fn validate_input_audio(
        &self,
        ref_signal: &AudioSignal,
        deg_signal: &AudioSignal,
    ) -> Result<(), VisqolError> {
        if ref_signal.sample_rate != deg_signal.sample_rate {
            return Err(VisqolError::DifferentSampleRates {
                reference: ref_signal.sample_rate,
                degraded: deg_signal.sample_rate,
            });
        }

        if (ref_signal.get_duration() - deg_signal.get_duration()).abs()
            > constants::DURATION_MISMATCH_TOLERANCE
        {
            log::warn!("Mismatch in duration between reference and degraded signal. Reference is {} seconds. Degraded is {} seconds.", ref_signal.get_duration(), deg_signal.get_duration());
        }
        Ok(())
    }
}
