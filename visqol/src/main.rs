use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::error::Error;

use visqol_rs::{
    similarity_result::SimilarityResult,
    visqol_manager::VisqolManager,
};

use visqol::{command_line_utils::{CommandLineArgs, build_file_pair_paths}, output_utils};
fn main() -> Result<(), Box<dyn Error>> {
    // Set up logger
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Always,
    )?;

    // Parse arguments
    let args = CommandLineArgs::parse();

    // Figure out file paths
    let files_to_compare = build_file_pair_paths(&args)?;

    let mut results = Vec::<SimilarityResult>::with_capacity(files_to_compare.len());
    let mut visqol = VisqolManager::new(
        &args.similarity_to_quality_model,
        args.use_speech_mode,
        args.use_unscaled_speech_mos_mapping,
        args.search_window_radius,
    );

    for file_pair in &files_to_compare {
        let result = visqol.run(&file_pair.reference, &file_pair.degraded)?;
        results.push(result);
    }
    output_utils::write_results(&args, &results, &files_to_compare);
    Ok(())
}
