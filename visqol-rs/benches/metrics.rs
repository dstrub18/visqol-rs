use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use visqol_rs::{
    audio_utils,
    constants::{DEFAULT_WINDOW_SIZE, NUM_BANDS_AUDIO, NUM_BANDS_SPEECH},
    variant::Variant,
    visqol_manager::VisqolManager,
};

fn bench_metrics(c: &mut Criterion) {
    c.bench_function("audio", |b| {
        let mut fullband = VisqolManager::<NUM_BANDS_AUDIO>::new(
            Variant::Fullband {
                model_path: "../model/libsvm_nu_svr_model.txt".into(),
            },
            DEFAULT_WINDOW_SIZE,
        );
        // TODO: we should probably not mutate these audio samples repeatedly,
        // but for benchmark purposes it may not matter
        let mut ref_audio =
            audio_utils::load_as_mono("./test_data/conformance_testdata_subset/ravel48_stereo.wav")
                .unwrap();
        let mut deg_audio = audio_utils::load_as_mono(
            "./test_data/conformance_testdata_subset/ravel48_stereo_128kbps_opus.wav",
        )
        .unwrap();
        b.iter(|| {
            let _ = black_box(fullband.compute_results(&mut ref_audio, &mut deg_audio));
        });
    });

    c.bench_function("speech", |b| {
        let mut wideband = VisqolManager::<NUM_BANDS_SPEECH>::new(
            Variant::Wideband {
                use_unscaled_mos_mapping: false,
            },
            DEFAULT_WINDOW_SIZE,
        );
        let mut ref_speech =
            audio_utils::load_as_mono("./test_data/clean_speech/reference_signal_16k.wav").unwrap();
        let mut deg_speech =
            audio_utils::load_as_mono("./test_data/clean_speech/degraded_signal_16k.wav").unwrap();
        b.iter(|| {
            let _ = black_box(wideband.compute_results(&mut ref_speech, &mut deg_speech));
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_metrics
}
criterion_main!(benches);
