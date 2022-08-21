use crate::audio_signal::AudioSignal;
use crate::envelope;
use crate::xcorr;
use ndarray::Array1;
use ndarray::{concatenate, s, Axis};
pub fn align_and_truncate(
    ref_signal: &AudioSignal,
    deg_signal: &AudioSignal,
) -> Option<(AudioSignal, AudioSignal, f64)> {
    let (aligned_deg_signal, lag) = globally_align(ref_signal, deg_signal)?;

    let mut new_ref_matrix = ref_signal.data_matrix.clone();
    let mut new_deg_matrix = aligned_deg_signal.data_matrix;

    match new_ref_matrix.len().cmp(&new_deg_matrix.len()) {
        std::cmp::Ordering::Less => {
            // For positive lag, the beginning of ref is now padded with zeros, so
            // that amount should be truncated.
            new_ref_matrix = new_ref_matrix
                .slice(s![
                    (lag * ref_signal.sample_rate as f64) as usize..ref_signal.len()
                ])
                .to_owned();
            new_deg_matrix = new_deg_matrix
                .slice(s![
                    (lag * deg_signal.sample_rate as f64) as usize..ref_signal.len()
                ])
                .to_owned();
        }
        std::cmp::Ordering::Greater => {
            new_ref_matrix = new_ref_matrix.slice(s![..new_deg_matrix.len()]).to_owned();
        }
        _ => (),
    }
    Some((
        AudioSignal::new(
            new_ref_matrix
                .as_slice()
                .expect("Failed to create AudioSignal from slice!"),
            ref_signal.sample_rate,
        ),
        AudioSignal::new(
            new_deg_matrix
                .as_slice()
                .expect("Failed to create AudioSignal from slice!"),
            deg_signal.sample_rate,
        ),
        lag,
    ))
}

pub fn globally_align(
    ref_signal: &AudioSignal,
    deg_signal: &AudioSignal,
) -> Option<(AudioSignal, f64)> {
    let ref_upper_env = envelope::calculate_upper_env(&ref_signal.data_matrix)?;
    let deg_upper_env = envelope::calculate_upper_env(&deg_signal.data_matrix)?;

    let best_lag = xcorr::calculate_best_lag(&ref_upper_env, &deg_upper_env)?;

    if best_lag == 0 || best_lag.abs() > (ref_signal.data_matrix.len() / 2) as i64 {
        // If signals are correlated already, return deg signal and 0.
        let new_deg_signal = AudioSignal::new(
            deg_signal
                .data_matrix
                .as_slice()
                .expect("Failed to create AudioSignal from slice!"),
            deg_signal.sample_rate,
        );
        Some((new_deg_signal, 0.0f64))
    } else {
        let mut new_deg_matrix = deg_signal.data_matrix.clone();
        // align degraded matrix
        if best_lag < 0 {
            new_deg_matrix = new_deg_matrix
                .slice(s![
                    best_lag.unsigned_abs() as usize..deg_signal.data_matrix.len()
                ])
                .to_owned();
        } else {
            let zeros = Array1::<f64>::zeros(best_lag as usize);
            new_deg_matrix = concatenate(Axis(0), &[zeros.view(), new_deg_matrix.view()])
                .expect("Failed to zero pad degraded matrix!");
        }

        let new_deg_signal = AudioSignal::new(
            new_deg_matrix
                .as_slice()
                .expect("Failed to create AudioSignal from slice!"),
            deg_signal.sample_rate,
        );
        Some((
            new_deg_signal,
            (best_lag as f64 / deg_signal.sample_rate as f64) as f64,
        ))
    }
}
