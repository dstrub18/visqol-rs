pub struct SimilarityResult 
{
    pub moslqo: f64,
    pub vnsim: f64,
    pub fnsim: Vec<f64>,
    pub fstdnsim: Vec<f64>,
    pub fvdegenergy: Vec<f64>,
    pub center_freq_bands: Vec<f64>,
}

impl SimilarityResult 
{
    pub fn new(
        moslqo: f64,
        vnsim: f64,
        fnsim: Vec<f64>,
        fstdnsim: Vec<f64>,
        fvdegenergy: Vec<f64>,
        center_freq_bands: Vec<f64>,
    ) -> Self 
    {
        Self {
            moslqo,
            vnsim,
            fnsim,
            fstdnsim,
            fvdegenergy,
            center_freq_bands,
        }
    }
}
