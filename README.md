# Visqol RS
* Reimplementation of Visqol algorithm for speech quality

## dev log
- [x] alignment.cc
- [x] amatrix.cc
- [x] analysis_window.h
- [x] commandline_parser.cc (all tests work, but the tests don't cover everything)
- [ ] comparison_patches_selector.cc
- [x] convolution_2d.cc
- [x] envelope.cc
- [x] equivalent_rectangular_bandwidth.cc
- [x] fast_fourier_transform.cc
- [x] fft_manager.cc
- [x] gammatone_filterbank.cc
- [ ] gammatone_spectrogram_builder.cc
- [x] image_patch_creator.cc
- [ ] libsvm_target_observation_convertor.cc
- [x] misc_audio.cc
- [x] misc_math.cc
- [x] misc_vector.cc
- [ ] neurogram_similiarity_index_measure.cc
- [x] rms_vad.cc
- [x] signal_filter.cc
- [x] spectrogram.cc
- [ ] speech_similarity_to_quality_mapper.cc
- [ ] support_vector_regression_model.cc
- [ ] svr_similarity_to_quality_mapper.cc
- [x] vad_patch_creator.cc (not tested)
- [ ] visqol.cc
- [x] visqol_api.cc
- [ ] visqol_manager.cc
- [x] wav_reader.cc
- [x] xcorr.cc