# Visqol-RS
- Implementation of the [Visqol v3.1](https://github.com/google/visqol) algorithm for speech quality evaluation in Rust
- Builds a command line executable which produces Visqol scores, with a command line interface identical to the C++ implementation.
- Or you can compute scores within your rust code! Just note that you will need to compile in Release mode.

# Audience
- Researchers, engineers, academics who work within the field of speech enhancement and perceptual audio evaluation.

# Build instructions
- You will need the [rust toolchain](https://rustup.rs/).
- A minimum supported Rust version(MSRV) is underway! Submit an issue in case things don't work for you :)
- So far, the executable builds successfully on macOS 10.15 and WSL2 Ubuntu.

# Example
```rust
use visqol_rs::{
    constants::{DEFAULT_WINDOW_SIZE, NUM_BANDS_SPEECH},
    variant::Variant,
    *,
};
let path_to_reference_file = "./test_data/clean_speech/reference_signal.wav";
let path_to_degraded_file = "./test_data/clean_speech/degraded_signal.wav";

let mut visqol = visqol_manager::VisqolManager::<NUM_BANDS_SPEECH>::new(
    Variant::Wideband {
        use_unscaled_mos_mapping: false,
    },
    DEFAULT_WINDOW_SIZE,
);

let similarity_result = visqol.run(path_to_reference_file, path_to_degraded_file)?;

println!(
    "Mean objective score for degraded file {}: {}",
    path_to_degraded_file, similarity_result.moslqo
);
```

# Notes
- For reasonable computation times, it is recommended to compile this library in Release mode. Due to the high complexity of the gammatone filterbank and computing the corresponding spectrogram, ViSQOL tends to be rather slow in debug mode.
- This is a spare time project. Please expect delays with regard to issues, pull requests etc.

# Papers
I highly encourage you to get familiar with Visqol by reading these papers:
- [Objective Assessment of Perceptual Audio Quality Using ViSQOLAudio](https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=7940042)
- [ViSQOL v3: An Open Source Production Ready Objective Speech and Audio Metric](https://arxiv.org/abs/2004.09584)

# Acknowledgement
- Since this project was more an exercise for me to learn Rust, none of the actual algorithm creation comes from me. I'd like to thank Jan Skoglund, Michael Chinen and Andrew Hines for their tremendous effort and innovation in the field of perceptual audio evaluation.
