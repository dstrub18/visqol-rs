use crate::support_vector_regression_model::SupportVectorRegressionModel;

pub enum Variant {
    Fullband { model: SupportVectorRegressionModel },  /// use SupportVectorRegressionModel instead of the file path
    Wideband { use_unscaled_mos_mapping: bool },
}
