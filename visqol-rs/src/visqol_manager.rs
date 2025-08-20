use std::error::Error;

use crate::{
    alignment,
    analysis_window::AnalysisWindow,
    audio_signal::AudioSignal,
    audio_utils,
    comparison_patches_selector::ComparisonPatchesSelector,
    constants::{self, PATCH_SIZE_AUDIO, PATCH_SIZE_SPEECH},
    gammatone_filterbank::GammatoneFilterbank,
    gammatone_spectrogram_builder::GammatoneSpectrogramBuilder,
    image_patch_creator::ImagePatchCreator,
    neurogram_similiarity_index_measure::NeurogramSimiliarityIndexMeasure,
    patch_creator::PatchCreator,
    similarity_result::SimilarityResult,
    similarity_to_quality_mapper::SimilarityToQualityMapper,
    speech_similarity_to_quality_mapper::SpeechSimilarityToQualityMapper,
    svr_similarity_to_quality_mapper::SvrSimilarityToQualityMapper,
    vad_patch_creator::VadPatchCreator,
    variant::Variant,
    visqol,
    visqol_error::VisqolError,
};

/// Configures and executes audio evaluation using ViSQOL.
pub struct VisqolManager<const NUM_BANDS: usize> {
    search_window: usize,
    patch_creator: Box<dyn PatchCreator>,
    patch_selector: ComparisonPatchesSelector,
    spectrogram_builder: GammatoneSpectrogramBuilder<NUM_BANDS>,
    sim_to_quality_mapper: Box<dyn SimilarityToQualityMapper>,
}

impl<const NUM_BANDS: usize> VisqolManager<NUM_BANDS> {
    /// Creates a new instance of with the desired configurations.
    pub fn new(variant: Variant, window_size: usize) -> Self {
        let patch_creator: Box<dyn PatchCreator>;
        let sim_to_quality_mapper: Box<dyn SimilarityToQualityMapper>;
        match variant {
            Variant::Wideband {
                use_unscaled_mos_mapping,
            } => {
                patch_creator = Box::new(VadPatchCreator::new(PATCH_SIZE_AUDIO));
                sim_to_quality_mapper = Box::new(SpeechSimilarityToQualityMapper::new(
                    !use_unscaled_mos_mapping,
                ));
            }
            Variant::Fullband { model_path } => {
                patch_creator = Box::new(ImagePatchCreator::new(PATCH_SIZE_SPEECH));
                sim_to_quality_mapper = Box::new(SvrSimilarityToQualityMapper::new(&model_path));
            }
        }

        let spectrogram_builder = GammatoneSpectrogramBuilder::<NUM_BANDS>::new(
            GammatoneFilterbank::new(constants::MINIMUM_FREQ),
        );

        let patch_selector =
            ComparisonPatchesSelector::new(NeurogramSimiliarityIndexMeasure::default());

        Self {
            search_window: window_size,
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

        Self::validate_input_audio(&ref_signal, &deg_signal)?;

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
            &mut self.spectrogram_builder, // this does not need to be self
            &window,
            self.patch_creator.as_mut(),
            &self.patch_selector,
            self.sim_to_quality_mapper.as_mut(),
            self.search_window,
        )
    }

    /// Performs sanity checks on the configuration to prevent incorrect use of the algorithm.
    fn validate_input_audio(
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
