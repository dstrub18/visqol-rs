use crate::{
    command_line_utils::CommandLineArgs, path_pair::PathPair,
};
use visqol_rs::similarity_result::SimilarityResult;

use csv::WriterBuilder;
use prettytable::{
    format::{FormatBuilder, LinePosition, LineSeparator, TableFormat},
    Cell, Row, Table,
};
use serde_json;

/// Writes debug info to either console or to file.
pub fn write_results(
    args: &CommandLineArgs,
    results: &Vec<SimilarityResult>,
    file_pairs: &Vec<PathPair>,
) {
    let version_number = env!("CARGO_PKG_VERSION");
    println!("ViSQOL conformance version: {version_number:}");
    if args.use_speech_mode {
        println!("Speech mode");
    } else {
        println!("Audio mode");
    }

    if let Some(json_output_path) = &args.output_debug {
        write_debug_json(json_output_path, results);
    }

    if let Some(csv_output_path) = &args.results_csv {
        write_results_to_csv(csv_output_path, results);
    }

    for (sim_result, file_pair) in results.iter().zip(file_pairs) {
        write_to_console(args, sim_result, file_pair);
    }
}

/// Writes debug info to console
fn write_to_console(args: &CommandLineArgs, result: &SimilarityResult, file_pair: &PathPair) {
    if args.verbose {
        println!("Reference Filepath:\t {:}", file_pair.reference);
        println!("Degraded Filepath:\t {:}", file_pair.degraded);
    }

    println!("MOS-LQO:\t\t{}", result.moslqo);

    if args.verbose {
        write_fvnsim_table(result);
        write_patch_similarity(result);
    }
}
/// Writes json formatted debug information.
fn write_debug_json(json_output_path: &String, results: &Vec<SimilarityResult>) {
    let mut json_output = String::new();
    for result in results {
        json_output = serde_json::to_string_pretty(result).expect("Could not format JSON!");
    }
    std::fs::write(json_output_path, json_output)
        .unwrap_or_else(|_| panic!("Could not write JSON to {}!", json_output_path.as_str()));
}

/// Writes computes MOS values to csv file.
fn write_results_to_csv(csv_output_path: &String, results: &Vec<SimilarityResult>) {
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_path(csv_output_path)
        .expect("Failed to instantiate CSV writer!");
    for result in results {
        writer
            .serialize(result)
            .expect("Failed to serialize SimilarityResult!");
    }
    writer.flush().expect("Failed to flush csv file!")
}

/// Formats FVNSIM info to table and writes it to console.
fn write_fvnsim_table(result: &SimilarityResult) {
    let mut table = Table::new();

    let format = get_default_table_format();

    table.set_format(format);
    for (fn_sim, freq_band) in result.fvnsim.iter().zip(&result.center_freq_bands) {
        table.add_row(Row::new(vec![
            Cell::new(&fn_sim.to_string()[..]),
            Cell::new(&freq_band.to_string()[..]),
        ]));
    }
    table.set_titles(Row::new(vec![Cell::new("FVNSIM"), Cell::new("Freq Band")]));
    table.printstd();
}

/// Formats patch similarity info for a file pair to a table and outputs it to the console
fn write_patch_similarity(result: &SimilarityResult) {
    let mut table = Table::new();

    let format = get_default_table_format();

    table.set_format(format);

    for (idx, patch_sim) in result.patch_sims.iter().enumerate() {
        let patch_idx_str = &idx.to_string()[..];
        let similarity = &patch_sim.similarity.to_string()[..];
        let ref_info = patch_sim.ref_patch_start_time.to_string()
            + "  -   "
            + &patch_sim.ref_patch_end_time.to_string()[..];
        let deg_info = patch_sim.deg_patch_start_time.to_string()
            + "  -   "
            + &patch_sim.deg_patch_end_time.to_string()[..];
        let row = Row::new(vec![
            Cell::new(patch_idx_str),
            Cell::new(similarity),
            Cell::new(&ref_info),
            Cell::new(&deg_info),
        ]);
        table.add_row(row);
    }

    let titles = Row::new(vec![
        Cell::new("Patch Idx"),
        Cell::new("Similarity"),
        Cell::new("Ref Patch: Start - End"),
        Cell::new("Deg Patch: Start - End"),
    ]);
    table.set_titles(titles);
    table.printstd();
}

/// Returns the default table format for the path similarity table
fn get_default_table_format() -> TableFormat {
    FormatBuilder::new()
        .column_separator('|')
        .separator(LinePosition::Title, LineSeparator::new('-', '-', '-', '-'))
        .separator(LinePosition::Top, LineSeparator::new('-', '-', '-', '-'))
        .borders('|')
        .padding(1, 1)
        .build()
}
