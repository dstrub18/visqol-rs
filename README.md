# Visqol-RS
- Implementation of Visqol v3 algorithm for speech quality evaluation in Rust
- Contains files to build Visqol as a command line executable, with a command line interface identical to the C++ implementation.
- API allows for computing Visqol scores in your own application or library.

# Example
```rust
use visqol_rs::*;
    
let path_to_reference_file = "./test_data/clean_speech/reference_signal.wav";
let path_to_degraded_file = "./test_data/clean_speech/degraded_signal.wav";
let config = visqol_config::VisqolConfig::get_speech_mode_config();
let mut visqol = visqol_manager::VisqolManager::from_config(&config);
let similarity_result = visqol.run(path_to_reference_file, path_to_degraded_file).unwrap();
println!("Mean objective score for degraded file {}: {}", path_to_degraded_file, similarity_result.moslqo);
```

# Notes
- For reasonable computation times, it is recommended to compile this library in Release mode. Due to the high complexity of the gammatone filterbank, ViSQOL tends to be rather slow in debug mode.