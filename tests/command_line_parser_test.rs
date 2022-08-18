use std::path::PathBuf;

use visqol_rs::command_line_parser;

#[test]
fn test_csv()
{
    // Expected results
    let ref_file_1 = "ref_1.wav";
    let deg_file_1 = "deg_1.wav";
    let ref_file_2 = "ref_2.wav";
    let deg_file_2 = "deg_2.wav";

    let batch_file_entry_count = 2;

    let file_pairs = command_line_parser::read_files_to_compare(&PathBuf::from("test_data/example_batch/batch_input.csv")).unwrap();

    assert_eq!(file_pairs.len(), batch_file_entry_count);
    assert_eq!(file_pairs[0].reference, ref_file_1);
    assert_eq!(file_pairs[0].degraded, deg_file_1);
    assert_eq!(file_pairs[1].reference, ref_file_2);
    assert_eq!(file_pairs[1].degraded, deg_file_2);
}