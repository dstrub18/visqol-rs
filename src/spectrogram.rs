use ndarray::{Array2, Axis};
use ndarray_stats::QuantileExt;
pub struct Spectrogram
{
    pub data: Array2::<f64>,
    pub center_freq_bands: Vec::<f64>
}

impl Spectrogram
{
    pub fn new(data: Array2::<f64>, center_freq_bands: Vec::<f64>) -> Self
    {
        Self
        {
            data,
            center_freq_bands
        }
    }

    pub fn convert_to_db(&mut self)
    {
        // Closure to convert single element to db scale.
        let sample_to_db = |element: f64|
        {
            let sample: f64 = if element == 0.0
            {
                f64::EPSILON
            }
            else
            {
                element.abs()
            };
            10.0 * (sample.log10())
        };
        self.data.mapv_inplace(sample_to_db);
    }

    pub fn get_minimum(&self) -> f64
    {
        *self.data.min().expect("Failed to compute minimum for spectrogram")
    }

    pub fn subtract_floor(&mut self, floor: f64)
    {
        self.data -= floor;
    }

    pub fn raise_floor(&mut self, new_floor: f64)
    {
        self.data.mapv_inplace(|element|{new_floor.max(element)});
    }

    pub fn raise_floor_per_frame(&mut self, noise_threshold: f64, other: &mut Self)
    {
        let min_columns = self.data.ncols().min(other.data.ncols());
    
        for index in 0..min_columns
        {
            let our_frame = &mut self.data.index_axis_mut(Axis(1), index);
            let other_frame = &mut other.data.index_axis_mut(Axis(1), index);
            let our_max = our_frame.max().expect("Failed to raise level for spectrogram!");
            let other_max = other_frame.max().expect("Failed to raise level for spectrogram!");
            let any_max = our_max.max(*other_max);
            let floor_db = any_max - noise_threshold;
            our_frame.mapv_inplace(|element|{floor_db.max(element)});
            other_frame.mapv_inplace(|element|{floor_db.max(element)});
        }
    }
}

impl std::ops::Index<(usize, usize)> for Spectrogram
{
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output 
    {
        &self.data[index]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Spectrogram 
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output 
    {
        &mut self.data[index]
    }
}