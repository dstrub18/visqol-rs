use std::error::Error;

use visqol_rs::*;
fn main() -> Result<(), Box<dyn Error>> {
    
    let path_to_reference_file = "./test_data/clean_speech/reference_signal.wav";
    let path_to_degraded_file = "./test_data/clean_speech/degraded_signal.wav";

    let config = visqol_config::VisqolConfig::get_speech_mode_config();

    let mut visqol = visqol_manager::VisqolManager::from_config(&config);

    let similarity_result = visqol.run(path_to_reference_file, path_to_degraded_file)?;

    println!("Mean objective score for degraded file {}: {}", path_to_degraded_file, similarity_result.moslqo);

    Ok(())
}
