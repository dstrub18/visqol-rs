use crate::{image_patch_creator::ImagePatchCreator, comparison_patches_selector::ComparisonPatchesSelector, spectrogram_builder::SpectrogramBuilder, similarity_to_quality_mapper::{SimilarityToQualityMapper}, patch_creator::PatchCreator, vad_patch_creator::VadPatchCreator, svr_similarity_to_quality_mapper::SvrSimilarityToQualityMapper, gammatone_spectrogram_builder::GammatoneSpectrogramBuilder, gammatone_filterbank::GammatoneFilterbank, neurogram_similiarity_index_measure::NeurogramSimiliarityIndexMeasure, audio_signal::AudioSignal, alignment, analysis_window::AnalysisWindow, visqol, similarity_result::SimilarityResult, file_path::FilePath, misc_audio};
use crate::speech_similarity_to_quality_mapper::SpeechSimilarityToQualityMapper;
use crate::constants;
pub struct VisqolManager
{
    pub use_speech_mode: bool,
    pub use_unscaled_speech_mos_mapping: bool,
    pub search_window: usize,
    pub patch_creator: Box<dyn PatchCreator>,
    pub patch_selector: ComparisonPatchesSelector,
    pub spectrogram_builder: Box<dyn SpectrogramBuilder>,
    pub sim_to_quality_mapper: Box<dyn SimilarityToQualityMapper>
}

impl VisqolManager
{
    

    pub fn new(model_path: &str, use_speech_mode: bool, use_unscaled_speech_mos_mapping: bool, search_window: usize)
    -> VisqolManager
    {
        let pc: Box<dyn PatchCreator>;
        if use_speech_mode
        {
            pc = Box::new(VadPatchCreator::new(constants::PATCH_SIZE_SPEECH));
        }
        else
        {
            pc = Box::new(ImagePatchCreator::new(constants::PATCH_SIZE_AUDIO));
        }

        let sim_to_quality_mapper: Box<dyn SimilarityToQualityMapper>;
        if use_speech_mode
        {
            sim_to_quality_mapper = Box::new(SpeechSimilarityToQualityMapper::new(!use_unscaled_speech_mos_mapping));
        }
        else
        {
            sim_to_quality_mapper = Box::new(SvrSimilarityToQualityMapper::new(model_path));
        }

        let sb: Box<dyn SpectrogramBuilder>;
        if use_speech_mode 
        {
            sb = Box::new(GammatoneSpectrogramBuilder::new(GammatoneFilterbank::new(constants::NUM_BANDS_SPEECH, constants::MINIMUM_FREQ), use_speech_mode));    
        }
        else
        {
            sb = Box::new(GammatoneSpectrogramBuilder::new(GammatoneFilterbank::new(constants::NUM_BANDS_AUDIO, constants::MINIMUM_FREQ), false));    
        }

        let patch_selector = ComparisonPatchesSelector::new(NeurogramSimiliarityIndexMeasure::default());

        Self
        {
            use_speech_mode,
            use_unscaled_speech_mos_mapping,
            search_window,
            patch_creator: pc,
            patch_selector,
            spectrogram_builder: sb,
            sim_to_quality_mapper
        }
    }

    pub fn run_from_filepaths(&mut self, ref_signal_path: &FilePath, deg_signal_path: &FilePath)
    -> SimilarityResult     
    {
        let ref_signal = misc_audio::load_as_mono(ref_signal_path.path.to_str().unwrap());
        let mut deg_signal = misc_audio::load_as_mono(deg_signal_path.path.to_str().unwrap());
        self.run(&ref_signal, &mut deg_signal)
    }
    
    
    pub fn run(&mut self,ref_signal: &AudioSignal, deg_signal: &mut AudioSignal)
    -> SimilarityResult     
    {
        let (mut deg_signal, _) = alignment::globally_align(ref_signal, deg_signal);

        let window = AnalysisWindow::new(ref_signal.sample_rate, constants::OVERLAP, constants::WINDOW_DURATION);

        visqol::calculate_similarity(ref_signal, &mut deg_signal, self.spectrogram_builder.as_mut(), &window, self.patch_creator.as_mut(), &self.patch_selector, self.sim_to_quality_mapper.as_mut(), self.search_window)
    }

}