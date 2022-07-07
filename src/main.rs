use clap::{Parser};
use visqol_rs::{command_line_parser::CommandLineArgs, visqol_manager::VisqolManager, file_path::FilePath};
fn main() 
{
    let args = CommandLineArgs::parse();

    let mut visqol = VisqolManager::new(args.similarity_to_quality_model.as_str(), args.use_speech_mode, args.use_unscaled_speech_mos_mapping, args.search_window_radius);
    let result = visqol.run_from_filepaths(&FilePath::new(args.reference_file), &FilePath::new(args.degraded_file));

    println!("MOS: {}", result.moslqo);
}