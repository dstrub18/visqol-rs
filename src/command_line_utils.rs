use std::{error::Error, path::PathBuf};

use crate::path_pair::PathPair;
use clap::Parser;
use csv::{ReaderBuilder, StringRecord};

#[derive(Parser, Debug)]
#[clap(name = "visqol-rs")]
#[clap(version)]
#[clap(about = "Perceptual quality estimator for speech and audio")]
pub struct CommandLineArgs {
    /// Used to specify a path to a CSV file with the format:{n}
    /// ------------------{n}
    /// reference,degraded{n}
    /// ref1.wav,deg1.wav{n}
    /// ref2.wav,deg2.wav{n}
    /// ------------------{n}
    /// If the `batch_input_csv` flag is used, the `reference_file`
    /// and `degraded_file` flags will be ignored.
    #[clap(
        long = "batch_input_csv",
        name = "batch_input_csv",
        conflicts_with = "reference_file",
        conflicts_with = "degraded_file"
    )]
    pub batch_input_csv: Option<String>,

    /// The wav file path used as the reference audio.
    #[clap(
        long = "reference_file",
        requires = "degraded-file",
        conflicts_with = "batch_input_csv"
    )]
    pub reference_file: Option<String>,

    /// The wav file path used as the degraded audio.
    #[clap(
        long = "degraded_file",
        requires = "reference-file",
        conflicts_with = "batch_input_csv"
    )]
    pub degraded_file: Option<String>,

    /// Used to specify a path that the similarity score results will be
    /// . This will be a CSV file with the format:{n}
    /// ------------------{n}
    /// reference,degraded,moslqo{n}
    /// ref1.wav,deg1.wav,3.4{n}
    /// ref2.wav,deg2.wav,4.1{n}
    #[clap(long = "results_csv")]
    pub results_csv: Option<String>,

    /// Enables verbose output in the terminal [default: false]
    #[clap(long)]
    pub verbose: bool,

    /// Used to specify a file path where output debug information will be
    /// written
    /// to. This debug info contains the full details of the comparison
    /// between the
    /// reference and degraded audio signals and is in JSON format. The
    /// file does
    /// not need to previously exist. Contents will be appended to the file
    /// if it
    /// does already exist or if ViSQOL is run in batch mode.
    #[clap(long = "output_debug")]
    pub output_debug: Option<String>,

    ///The libsvm model to use during comparison. Use this only if you
    ///want to explicitly specify the model file location, otherwise the
    ///default model will be used.
    #[clap(
        long = "similarity_to_quality_model",
        default_value = "/model/libsvm_nu_svr_model.txt"
    )]
    pub similarity_to_quality_model: String,

    /// Use a wideband model (sensitive up to 8kHz) with voice activity
    /// detection
    /// that normalizes the polynomial NSIM->MOS mapping so that a perfect
    /// NSIM
    /// score of 1.0 translates to 5.0. [default: false]
    #[clap(long = "use_speech_mode")]
    pub use_speech_mode: bool,

    /// When used in conjunction with --use_speech_mode, this flag will
    /// prevent a
    /// perfect NSIM score of 1.0 being translated to a MOS score of 5.0.
    /// Perfect
    /// NSIM scores will instead result in MOS scores of ~4.x. [default: false]
    #[clap(long = "use_unscaled_speech_mos_mapping")]
    pub use_unscaled_speech_mos_mapping: bool,

    /// The search_window parameter determines how far the algorithm will
    /// search to discover patch matches. For a given reference frame, it
    /// will look at 2*search_window_radius + 1 patches to find the most
    /// optimal match.
    #[clap(long = "search_window_radius", default_value_t = 60)]
    pub search_window_radius: usize,
}

pub fn build_file_pair_paths(args: &CommandLineArgs) -> Result<Vec<PathPair>, Box<dyn Error>> {
    let mut file_pairs = Vec::<PathPair>::new();
    if let (Some(ref_file), Some(deg_file)) = (&args.reference_file, &args.degraded_file) {
        file_pairs.push(PathPair::new(ref_file, deg_file));
        Ok(file_pairs)
    } else if let Some(csv_file) = &args.batch_input_csv {
        read_files_to_compare(&PathBuf::from(csv_file))
    } else {
        Ok(file_pairs)
    }
}

pub fn read_files_to_compare(batch_input_path: &PathBuf) -> Result<Vec<PathPair>, Box<dyn Error>> {
    let mut file_paths = Vec::<PathPair>::new();
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path(batch_input_path)
        .unwrap_or_else(|_| panic!("Failed to read csv file!"));

    let header = StringRecord::from(vec!["reference", "degraded"]);
    while let Some(result) = reader.records().next() {
        let record = result?;
        let row: PathPair = record
            .deserialize(Some(&header))
            .expect("Failed to deserialize line in csv file!");
        file_paths.push(row);
    }
    Ok(file_paths)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_csv() {
        // Expected results
        let ref_file_1 = "ref_1.wav";
        let deg_file_1 = "deg_1.wav";
        let ref_file_2 = "ref_2.wav";
        let deg_file_2 = "deg_2.wav";

        let batch_file_entry_count = 2;

        let file_pairs =
            read_files_to_compare(&PathBuf::from("test_data/example_batch/batch_input.csv"))
                .unwrap();

        assert_eq!(file_pairs.len(), batch_file_entry_count);
        assert_eq!(file_pairs[0].reference, ref_file_1);
        assert_eq!(file_pairs[0].degraded, deg_file_1);
        assert_eq!(file_pairs[1].reference, ref_file_2);
        assert_eq!(file_pairs[1].degraded, deg_file_2);
    }
}
