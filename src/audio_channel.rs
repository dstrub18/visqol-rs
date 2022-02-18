use pffft_rust;

#[allow(unused)]
pub struct AudioChannel
{
    size: usize,
    aligned_buffer: Vec::<f64>,
}
#[allow(unused)]
impl AudioChannel
{
    fn new(size: usize) -> Self
    {
        Self
        {
            size: size,
            aligned_buffer: Vec::new()
        }
    }

    fn clear(&mut self)
    {
        self.aligned_buffer.iter_mut().map(|x| *x = 0.0f64);
    }
}

impl std::ops::Index<usize> for AudioChannel
{
    type Output = f64;
    fn index(&self, index: usize) -> &f64
    {
        &(self.aligned_buffer[index])
    }
}