## Visqol 3.1 CLI tool 

- Implementation of the [Visqol v3.1](https://github.com/google/visqol) algorithm for speech quality evaluation in Rust
- Builds a command line executable which produces Visqol scores, with a more ergonomic command line interface compared to the C++ implementation.
- Also check out [the library](https://crates.io/crates/visqol-rs) for using visqol within your Rust project!

## Installation
- Assuming you have the `cargo` package manager installed, type `cargo install visqol` to install the binary.

## Usage
`visqol --help` or simply `visqol` will show you all the flags you can set when invoking the binary.
Note that the CLI tool has 2 subcommands:
1. wideband: for speech signals, sample rate 16 kHz
2. fullband: for music signals, sample rate 48 kHz

All command line flags pertaining only the individual modes will have to be specified _after_ the subcommand.
Flags like `--reference_file` will have to be specified _before_ the subcommand as they are mandatory regardless of which mode the algorithm runs in.
### Example
```bash
visqol \ # command
--reference_file visqol-rs/test_data/clean_speech/CA01_01.wav \ # reference file
--degraded_file visqol-rs/test_data/clean_speech/degraded_signal.wav \ # degraded file
wideband \ # mode: wideband|fullband
--use_unscaled_speech_mos_mapping # flag for wideband mode only
` # 
```