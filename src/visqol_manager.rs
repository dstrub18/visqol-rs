use crate::{image_patch_creator::ImagePatchCreator, comparison_patches_selector::ComparisonPatchesSelector, spectrogram_builder::SpectrogramBuilder, similarity_to_quality_mapper::SimilarityToQualityMapper};

pub struct VisqolManager
{
    use_speech_mode: bool,
    use_unsclaed_speech_mos_mapping: bool,
    search_window: usize,
    patch_creator: ImagePatchCreator,
    patch_selector: ComparisonPatchesSelector,
    spectrogram_builder: Box<dyn SpectrogramBuilder>,
    sim_to_quality_mapper: Box<dyn SimilarityToQualityMapper>
}

impl VisqolManager
{
    const PATCH_SIZE: usize = 30;
    const PATCH_SIZE_SPEECH: usize = 20;
    const NUM_BANDS_AUDIO: usize = 32;
    const NUM_BANDS_SPEECH: usize = 21;
    const MINIMUM_FREQ: usize = 50;
    const OVERLAP: f64 = 0.25;
    const DURATION_MISMATCH_TOLERANCE: f64 = 1.0;

    pub fn new(model_path: &str, use_speech_mode: bool, use_unsclaed_speech_mos_mapping: bool, search_window: usize)
    {
        if use_speech_mode
        {
            // Be sweet Daniel :)
        }
        else
        {

        }
    }

}