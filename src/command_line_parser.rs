use crate::file_path::{FilePath, ReferenceDegradedPathPair};
use clap::{Parser};
use csv::{ReaderBuilder, StringRecord};

#[derive(Parser, Debug)]
#[clap(name="visqol-rs")]
#[clap(version)]
#[clap(about="Perceptual quality estimator for speech and audio")]
//#[clap(term_width(0))]
pub struct CommandLineArgs
{
    /// The wav file path used as the reference audio.
    #[clap(long, conflicts_with="batch-input-csv", requires="degraded-file")]
    pub reference_file: String,
    
    /// The wav file path used as the degraded audio.
    #[clap(long, conflicts_with="batch-input-csv", requires="reference-file")]
    pub degraded_file: String,
    
    /// Used to specify a path to a CSV file with the format:{n}
    /// ------------------{n}
    /// reference,degraded{n}
    /// ref1.wav,deg1.wav{n}
    /// ref2.wav,deg2.wav{n}
    /// ------------------{n}
    /// If the `batch_input_csv` flag is used, the `reference_file` 
    /// and `degraded_file` flags will be ignored.);
    #[clap(long, default_value="input.csv", conflicts_with="reference-file", conflicts_with="degraded-file")]
    pub batch_input_csv: String,
    
    /// Used to specify a path that the similarity score results will be 
    /// . This will be a CSV file with the format:{n}
    /// ------------------{n}
    /// reference,degraded,moslqo{n}
    /// ref1.wav,deg1.wav,3.4{n}
    /// ref2.wav,deg2.wav,4.1{n}
    #[clap(long, default_value="results.csv")]
    pub results_csv: String,
    
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
    #[clap(long, default_value="debug_info.txt")]
    pub output_debug: String,
    
    ///The libsvm model to use during comparison. Use this only if you 
    ///want to explicitly specify the model file location, otherwise the 
    ///default model will be used.
    #[clap(long, default_value="/model/libsvm_nu_svr_model.txt")]
    pub similarity_to_quality_model: String,
    
    /// Use a wideband model (sensitive up to 8kHz) with voice activity 
    /// detection
    /// that normalizes the polynomial NSIM->MOS mapping so that a perfect 
    /// NSIM
    /// score of 1.0 translates to 5.0.); [default: false]
    #[clap(long)]
    pub use_speech_mode: bool,
    
    /// When used in conjunction with --use_speech_mode, this flag will 
    /// prevent a
    /// perfect NSIM score of 1.0 being translated to a MOS score of 5.0. 
    /// Perfect
    /// NSIM scores will instead result in MOS scores of ~4.x. [default: false]
    #[clap(long)]
    pub use_unscaled_speech_mos_mapping: bool,
    
    /// The search_window parameter determines how far the algorithm will 
    /// search to discover patch matches. For a given reference frame, it 
    /// will look at 2*search_window_radius + 1 patches to find the most 
    /// optimal match.
    #[clap(long, default_value_t = 60)]
    pub search_window_radius: usize
}

// Todo: Replace with result type to handle errors.
pub fn read_files_to_compare(batch_input_path: &FilePath)
-> Vec<ReferenceDegradedPathPair> 
{
    let mut file_paths = Vec::<ReferenceDegradedPathPair>::new();
    let mut reader = ReaderBuilder::new().has_headers(true).delimiter(b',').from_path(batch_input_path.path()).unwrap();

    while let Some(result) = reader.records().next()
    {
        let header = StringRecord::from(vec!["reference", "degraded"]);
        let record = result.unwrap();
        let row: ReferenceDegradedPathPair = record.deserialize(Some(&header)).unwrap();
        file_paths.push(row);
    }
    file_paths
}