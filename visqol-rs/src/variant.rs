pub enum Variant {
    Fullband { model_path: String },
    Wideband { use_unscaled_mos_mapping: bool },
}
impl Variant {
    pub const DEFAULT_WINDOW_SIZE: usize = 60;
}
