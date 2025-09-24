use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::error::Error;
use std::fs;

use visqol_rs::{
    constants::{NUM_BANDS_AUDIO, NUM_BANDS_SPEECH},
    similarity_result::SimilarityResult,
    support_vector_regression_model::SupportVectorRegressionModel,
    variant::Variant,
    visqol_manager::VisqolManager,
};

pub mod command_line_utils;
pub mod output_utils;
pub mod path_pair;
pub use crate::command_line_utils::{build_file_pair_paths, CommandLineArgs};
use crate::path_pair::PathPair;

fn run<const NUM_BANDS: usize>(
    path_pairs: &Vec<PathPair>,
    visqol: &mut VisqolManager<NUM_BANDS>,
) -> Result<Vec<SimilarityResult>, Box<dyn Error>> {
    let mut results = Vec::<SimilarityResult>::with_capacity(path_pairs.len());
    for file_pair in path_pairs {
        let result = visqol.run(&file_pair.reference, &file_pair.degraded)?;
        results.push(result);
    }
    Ok(results)
}
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

    let files_to_compare = build_file_pair_paths(&args)?;

    let variant: Variant;
    let mut visqol_speech: VisqolManager<NUM_BANDS_SPEECH>;
    let mut visqol_audio: VisqolManager<NUM_BANDS_AUDIO>;

    let results: Vec<SimilarityResult>;
    match &args.subcommand {
        command_line_utils::Subcommands::Wideband {
            use_unscaled_speech_mos_mapping,
        } => {
            variant = Variant::Wideband {
                use_unscaled_mos_mapping: *use_unscaled_speech_mos_mapping,
            };
            visqol_speech = VisqolManager::new(variant, args.search_window_radius);
            results = run(&files_to_compare, &mut visqol_speech)?;
        }
        command_line_utils::Subcommands::Fullband {
            similarity_to_quality_model,
        } => {
            /// Creating an SVM model depending on whether the user has specified the path.
            let svm_model = match similarity_to_quality_model {
                /// If the path is specified, read the file and create a model from its contents.
                Some(path) => {
                    log::info!("Loading custom SVM model from: {}", path);
                    let model_content = fs::read_to_string(path)?;
                    SupportVectorRegressionModel::from_str(&model_content)
                }
                // If the path is not specified, we use the built-in default model.
                None => {
                    log::info!("Using embedded default SVM model.");
                    SupportVectorRegressionModel::default()
                }
            };

            // Creating a variant with a ready-made model
            variant = Variant::Fullband { model: svm_model };

            // VisqolManager::new should now accept this new variant.
            visqol_audio = VisqolManager::new(variant, args.search_window_radius);
            results = run(&files_to_compare, &mut visqol_audio)?;
        }
    }

    output_utils::write_results(&args, &results, &files_to_compare);
    Ok(())
}