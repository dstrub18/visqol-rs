/// Bundles all configuration fields for ViSQOL audio evaluation into a single struct
pub struct VisqolConfig {
    pub similarity_to_quality_model_path: String,
    pub use_speech_mode: bool,
    pub use_unscaled_speech_mos_mapping: bool,
    pub search_window: usize,
}

impl VisqolConfig {
    /// Creates a new config with the desired configuration.
    pub fn new(
        similarity_to_quality_model_path: &str,
        use_speech_mode: bool,
        use_unscaled_speech_mos_mapping: bool,
        search_window: usize,
    ) -> Self {
        Self {
            similarity_to_quality_model_path: String::from(similarity_to_quality_model_path),
            use_speech_mode,
            use_unscaled_speech_mos_mapping,
            search_window,
        }
    }

    /// Creates the default configuration for speech mode.
    pub fn get_speech_mode_config() -> Self {
        Self {
            similarity_to_quality_model_path: String::from(""),
            use_speech_mode: true,
            use_unscaled_speech_mos_mapping: false,
            search_window: 60,
        }
    }
    /// Creates the default configuration for audio mode.
    pub fn get_audio_mode_config() -> Self {
        Self {
            similarity_to_quality_model_path: String::from("./model/libsvm_nu_svr_model.txt"),
            use_speech_mode: false,
            use_unscaled_speech_mos_mapping: false,
            search_window: 60,
        }
    }
}
