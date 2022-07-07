#[derive(Clone)]
pub struct AudioChannel<T>
{
    pub aligned_buffer: Vec::<T>
}

impl<T> AudioChannel<T>
where T: num::Zero
{
    pub fn clear(&mut self)
    {
        self.aligned_buffer.iter_mut().for_each(|x|{*x = T::zero()});
    }
}

impl<T> AudioChannel<T>
where T: num::Zero + std::clone::Clone
{
    pub fn new(size: usize) -> Self
    {
        Self
        {
            aligned_buffer: vec![T::zero(); size]
        }
    }

    pub fn get_size(&self) -> usize
    {
        self.aligned_buffer.len()
    }
    
}

impl<T> std::ops::Index<usize> for AudioChannel<T>
{
    type Output = T;
    fn index(&self, index: usize) -> &T
    {
        &(self.aligned_buffer[index])
    }
}

impl<T> std::ops::IndexMut<usize> for AudioChannel<T>
{
    fn index_mut(&mut self, index: usize) -> &mut T
    {
        &mut (self.aligned_buffer[index])
    }
}
