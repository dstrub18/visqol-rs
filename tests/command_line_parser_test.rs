use visqol_rs::{command_line_parser, file_path::FilePath};
use clap::Parser;
#[test]
#[ignore = "Not finished"]
fn test_parser()
{
    let args = command_line_parser::CommandLineArgs::parse();
}

#[test]
fn test_csv()
{
    // Expected results
    let ref_file_1 = String::from("ref_1.wav");
    let deg_file_1 = String::from("deg_1.wav");
    let ref_file_2 = String::from("ref_2.wav");
    let deg_file_2 = String::from("deg_2.wav");

    let batch_file_entry_count = 2;

    let file_pairs = command_line_parser::read_files_to_compare(&FilePath::new(String::from("test_data/example_batch/batch_input.csv")));

    assert_eq!(file_pairs.len(), batch_file_entry_count);
    assert_eq!(file_pairs[0].reference.to_string_lossy(), ref_file_1);
    assert_eq!(file_pairs[0].degraded.to_string_lossy(), deg_file_1);
    assert_eq!(file_pairs[1].reference.to_string_lossy(), ref_file_2);
    assert_eq!(file_pairs[1].degraded.to_string_lossy(), deg_file_2);

    println!("hi!");

}