use clap::Parser;
use std::error::Error;
use visqol_rs::{
    command_line_parser::{build_file_pair_paths, CommandLineArgs},
    output_utility,
    similarity_result::SimilarityResult,
    visqol_manager::VisqolManager,
};

fn main() -> Result<(), Box<dyn Error>>
{
    let args = CommandLineArgs::parse();

    let files_to_compare = build_file_pair_paths(&args);

    let mut results = Vec::<SimilarityResult>::with_capacity(files_to_compare.len());
    let mut visqol = VisqolManager::new(
        &args.similarity_to_quality_model,
        args.use_speech_mode,
        args.use_unscaled_speech_mos_mapping,
        args.search_window_radius,
    );

    for file_pair in &files_to_compare
    {
        let result = visqol
            .run(&file_pair.reference, &file_pair.degraded)?;
        results.push(result);
    }
    output_utility::write_results(&args, &results, &files_to_compare);
    Ok(())
}
