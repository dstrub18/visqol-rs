use ndarray::Array1;
pub struct AudioSignal
{
    // Using Array1 here since it can generalize for mono and multichannel
    pub data_matrix: Array1::<f64>,
    pub sample_rate: u32
}

impl AudioSignal
{
    pub fn new(data_matrix: Array1::<f64>, sample_rate: u32) -> AudioSignal     
    {   
        AudioSignal
        {
            data_matrix,
            sample_rate
        }
    }
    pub fn get_duration(&self) -> f64
    {
        (self.data_matrix.len() as f64 / self.sample_rate as f64) as f64
    }

    pub fn len(&self)-> usize     
    {
        self.data_matrix.len()
    }
}