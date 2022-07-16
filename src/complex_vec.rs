use num::complex::Complex64;
#[derive(Debug)]
pub struct ComplexVec
{
    pub v: Vec<Complex<f64>>,
}

impl ComplexVec
{
    pub fn new() -> Self
    {
        Self
        {
            v: Vec::new()
        }
    }
    pub fn from(other: &Self) -> Self
    {
        let mut out = Self::new_with_len(other.len());
        for i in 0..other.len()
        {
            out[i] = other[i];
        }
        out
    }

    pub fn new_with_len(size: usize) -> Self
    {
        Self
        {
            v: vec![Complex::<f64>::new(0.0, 0.0); size]
        }
    }

    pub fn populate_with(&mut self, element: Complex<f64>)
    {
        for i in self.v.iter_mut()
        {
            *i = element;
        }
    }

    pub fn new_from_complex_vector( vec: &Vec<Complex<f64>>) -> Self
    {
        let mut cv = Self::new_with_len(vec.len());
        for i in 0..vec.len()
        {
            cv[i] = vec[i];
        }
        cv
    }

    pub fn new_from_real_vector( vec: &Vec<f64>) -> Self
    {
        let mut cv = Self::new_with_len(vec.len());
        for i in 0..cv.len()
        {
            cv.v[i].re = vec[i];
        }
        cv
    }

    pub fn push(&mut self, new_element: Complex<f64>)
    {
        self.v.push(new_element)
    }

    pub fn len(&self) -> usize
    {
        self.v.len()
    }

    pub fn powf(&self, exponent: f64) -> Self
    {
        let mut out = Self::new_with_len(self.len());
        for i in 0..self.len()
        {
            out[i] = self[i].powf(exponent);
        }
        out
    }

    pub fn exp(&self) -> Self
    {
        let mut out = Self::new_with_len(self.len());
        for i in 0..self.len()
        {
            out[i] = self[i].exp();
        }
        out
    }
    
    pub fn sin(&self) -> Self
    {
        let mut out = Self::new_with_len(self.len());
        for i in 0..self.len()
        {
            out[i] = self[i].sin();
        }
        out
    }

    pub fn cos(&self) -> Self
    {
        let mut out = Self::new_with_len(self.len());
        for i in 0..self.len()
        {
            out[i] = self[i].cos();
        }
        out
    }

    pub fn abs(&self) -> Vec<f64>
    {
        let mut out = Vec::new();
        for i in 0..self.len()
        {
            out.push(self[i].norm());
        }
        out
    }

    pub fn to_real_vector(&self) -> Vec<f64>
    {
        let mut out = Vec::new();
        for i in 0..self.len()
        {
            out.push(self[i].re);
        }
        out
    }
}

impl std::ops::Index<usize> for ComplexVec
{
    type Output = Complex<f64>;
    fn index(&self, i: usize) -> &Complex<f64>
    {
        &self.v[i]
    }
}

impl std::ops::IndexMut<usize> for ComplexVec
{
    fn index_mut(&mut self, i: usize) -> &mut Complex<f64>
    {
        &mut self.v[i]
    }
}

impl std::ops::Add<f64> for ComplexVec
{
    type Output = Self;
    fn add(self, rhs: f64) -> Self
    {
        let mut cv = Self::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv.v[i].re = self[i].re + rhs;
            cv.v[i].im = self[i].im + rhs;
        }
        cv
    }
}

impl std::ops::Add<f64> for &ComplexVec
{
    type Output = ComplexVec;
    fn add(self, rhs: f64) -> ComplexVec
    {
        let mut cv = ComplexVec::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv.v[i].re = self[i].re + rhs;
            cv.v[i].im = self[i].im + rhs;
        }
        cv
    }
}

impl std::ops::Add<Self> for ComplexVec
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self
    {
        let mut cv = Self::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i] = self[i] + rhs[i];
        }
        cv
    }
}

impl std::ops::Add<&ComplexVec> for &ComplexVec
{
    type Output = ComplexVec;
    fn add(self, rhs: &ComplexVec) -> ComplexVec
    {
        let mut cv = ComplexVec::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i] = self[i] + rhs[i];
        }
        cv
    }
}

impl std::ops::Sub<f64> for ComplexVec
{
    type Output = Self;
    fn sub(self, rhs: f64) -> Self
    {
        let mut cv = Self::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i].re = self[i].re - rhs;
            cv[i].im = self[i].im - rhs;
        }
        cv
    }
}

impl std::ops::Sub<Self> for ComplexVec
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self
    {
        let mut cv = Self::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i] = self[i] - rhs[i];
        }
        cv
    }
}

impl std::ops::Sub<&ComplexVec> for &ComplexVec
{
    type Output = ComplexVec;
    fn sub(self, rhs: &ComplexVec) -> ComplexVec
    {
        let mut cv = ComplexVec::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i] = self[i] - rhs[i];
        }
        cv
    }
}

impl std::ops::Neg for ComplexVec
{
    type Output = Self;
    fn neg(self) -> Self
    {
        let mut cv = Self::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i] = -self[i];
        }
        cv
    }
}

impl std::ops::Mul<f64> for ComplexVec
{
    type Output = Self;
    fn mul(self, rhs: f64) -> Self
    {
        let mut cv = Self::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i].re = self[i].re * rhs;
            cv[i].im = self[i].im * rhs;
        }
        cv
    }
}
impl std::ops::Mul<f64> for &ComplexVec
{
    type Output = ComplexVec;
    fn mul(self, rhs: f64) -> ComplexVec
    {
        let mut cv = ComplexVec::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i].re = self[i].re * rhs;
            cv[i].im = self[i].im * rhs;
        }
        cv
    }
}

impl std::ops::Mul<Self> for &ComplexVec
{
    type Output = ComplexVec;
    fn mul(self, rhs: Self) -> ComplexVec
    {
        let mut cv = ComplexVec::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i] = self[i] * rhs[i];
        }
        cv
    }
}

impl std::ops::Mul<&ComplexVec> for ComplexVec
{
    type Output = ComplexVec;
    fn mul(self, rhs: &Self) -> ComplexVec
    {
        let mut cv = ComplexVec::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i] = self[i] * rhs[i];
        }
        cv
    }
}

impl std::ops::Mul<Self> for ComplexVec
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self
    {
        let mut cv = Self::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i] = self[i] * rhs[i];
        }
        cv
    }
}

impl std::ops::Mul<Complex<f64>> for ComplexVec
{
    type Output = Self;
    fn mul(self, rhs: Complex<f64>) -> Self
    {
        let mut cv = Self::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i] = self[i] * rhs;
        }
        cv
    }
}


impl std::ops::Div<f64> for ComplexVec
{
    type Output = Self;
    fn div(self, rhs: f64) -> Self
    {
        let mut cv = Self::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i].re = self[i].re / rhs;
            cv[i].im = self[i].im / rhs;
        }
        cv
    }
}

impl std::ops::Div<&Self> for ComplexVec
{
    type Output = Self;
    fn div(self, rhs: &Self) -> Self
    {
        let mut cv = Self::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i] = self[i].re / rhs[i];
        }
        cv
    }
}

impl std::ops::Div<&ComplexVec> for &ComplexVec
{
    type Output = ComplexVec;
    fn div(self, rhs: &ComplexVec) -> ComplexVec
    {
        let mut cv = ComplexVec::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i] = self[i].re / rhs[i];
        }
        cv
    }
}

impl std::ops::Div<Self> for ComplexVec
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self
    {
        let mut cv = Self::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i] = self[i] / rhs[i];
        }
        cv
    }
}

impl std::ops::Div<f64> for &ComplexVec
{
    type Output = ComplexVec;
    fn div(self, rhs: f64) -> ComplexVec
    {
        let mut cv = ComplexVec::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i].re = self[i].re / rhs;
            cv[i].im = self[i].im / rhs;
        }
        cv
    }
}

impl std::ops::Div<Complex<f64>> for ComplexVec
{
    type Output = Self;
    fn div(self, rhs: Complex<f64>) -> Self
    {
        let mut cv = Self::new_with_len(self.len());
        for i in 0..cv.len()
        {
            cv[i] = self[i] / rhs;
        }
        cv
    }
}