use clap::{Parser};
use visqol_rs::{command_line_parser::{CommandLineArgs, build_file_pair_paths}, visqol_manager::VisqolManager, file_path::FilePath, similarity_result::SimilarityResult, output_utility};


struct ArrayWrapper<T, const N: usize>
{
    a: [T;N]
}

impl<T, const N: usize> ArrayWrapper<T, N>
{
    
}

fn main()
{


    /* let args = CommandLineArgs::parse();

    let files_to_compare = build_file_pair_paths(&args);

    let mut results = Vec::<SimilarityResult>::with_capacity(files_to_compare.len());
    let mut visqol = VisqolManager::new(args.similarity_to_quality_model.as_str(), args.use_speech_mode, args.use_unscaled_speech_mos_mapping, args.search_window_radius);
    
    for file_pair in &files_to_compare
    {
        let result = visqol.run_from_filepaths(&FilePath::new(&file_pair.reference), &FilePath::new(&file_pair.degraded));
        results.push(result);
    }
    output_utility::write_results(&args, &results, &files_to_compare); */
}