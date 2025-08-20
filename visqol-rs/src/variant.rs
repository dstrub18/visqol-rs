pub enum Variant {
    Fullband { model_path: String },
    Wideband { use_unscaled_mos_mapping: bool },
}
