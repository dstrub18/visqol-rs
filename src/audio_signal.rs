use ndarray::Array2;
#[allow(unused)]
pub struct AudioSignal
{
    // Using Array2 here since it can generalize for mono and multichannel
    pub data_matrix: Array2::<f64>,
    pub sample_rate: u32
}
#[allow(unused)]
impl AudioSignal
{
    pub fn get_duration(&self) -> f64
    {
        (self.data_matrix.nrows() as f64 / self.sample_rate as f64) as f64
    }


}