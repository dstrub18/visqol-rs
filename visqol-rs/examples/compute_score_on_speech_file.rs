use std::error::Error;

use visqol_rs::{constants::NUM_BANDS_SPEECH, variant::Variant, *};
fn main() -> Result<(), Box<dyn Error>> {
    let path_to_reference_file = "./test_data/clean_speech/reference_signal.wav";
    let path_to_degraded_file = "./test_data/clean_speech/degraded_signal.wav";

    let mut visqol = visqol_manager::VisqolManager::<NUM_BANDS_SPEECH>::new(
        Variant::Wideband {
            use_unscaled_mos_mapping: false,
        },
        Variant::DEFAULT_WINDOW_SIZE,
    );

    let similarity_result = visqol.run(path_to_reference_file, path_to_degraded_file)?;

    println!(
        "Mean objective score for degraded file {}: {}",
        path_to_degraded_file, similarity_result.moslqo
    );

    Ok(())
}
