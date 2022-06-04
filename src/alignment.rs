#[allow(unused)]
use ndarray::{Array2, Axis, s, concatenate};
use crate::audio_signal::AudioSignal;
use crate::envelope;
use crate::xcorr;
pub fn align_and_truncate(ref_signal: &AudioSignal, deg_signal: &AudioSignal)
-> (AudioSignal, AudioSignal, f64) {
    let (aligned_deg_signal, lag) = globally_align(ref_signal, deg_signal);

    let mut new_ref_matrix = ref_signal.data_matrix.clone();
    let mut new_deg_matrix = aligned_deg_signal.data_matrix.clone();

    if new_ref_matrix.nrows() > new_deg_matrix.nrows() 
    {
        // This could be done better
        new_ref_matrix = new_ref_matrix.slice(s![0..new_deg_matrix.nrows(), ..]).to_owned();
    }
    else if new_ref_matrix.nrows() < new_deg_matrix.nrows() 
    {
        // For positive lag, the beginning of ref is now aligned with zeros, so
        // that amount should be truncated.
        // This could also be done better.
        new_ref_matrix = new_ref_matrix.slice(s![(lag as u32 * ref_signal.sample_rate) as usize .. ref_signal.nrows(), ..]).to_owned();
        new_deg_matrix = new_deg_matrix.slice(s![(lag as u32 * deg_signal.sample_rate) as usize .. ref_signal.nrows(), ..]).to_owned();
    }

    (
     AudioSignal::new(new_ref_matrix, ref_signal.sample_rate),
     AudioSignal::new(new_deg_matrix, deg_signal.sample_rate),
     lag)

}

pub fn globally_align(ref_signal: &AudioSignal, deg_signal: &AudioSignal)
-> (AudioSignal, f64)
{
    let ref_upper_env = envelope::calculate_upper_env(&ref_signal.data_matrix);
    let deg_upper_env = envelope::calculate_upper_env(&deg_signal.data_matrix);

    let best_lag = xcorr::calculate_best_lag(&ref_upper_env, &deg_upper_env);

    if best_lag == 0 || best_lag.abs() > (ref_signal.data_matrix.nrows() / 2) as i64 
    {
        // return deg signal and 0.
        let new_deg_signal = AudioSignal::new(deg_signal.data_matrix.clone(),deg_signal.sample_rate);
        (new_deg_signal, 0.0f64)
    }
    else
    {
        let mut new_deg_matrix = deg_signal.data_matrix.clone();
        // align degraded matrix
        if best_lag < 0
        {
            new_deg_matrix = new_deg_matrix.slice(s![best_lag.abs() as usize ..deg_signal.data_matrix.nrows(), ..]).to_owned();
        }
        else
        {
            let zeros = Array2::<f64>::zeros((best_lag as usize, 1));
            new_deg_matrix = concatenate(Axis(0), &[zeros.view(), new_deg_matrix.view()]).unwrap();
        }

        let new_deg_signal = AudioSignal::new(new_deg_matrix, deg_signal.sample_rate);
        (new_deg_signal, (best_lag / deg_signal.sample_rate as i64) as f64)
    }
}

