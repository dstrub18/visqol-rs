#[allow(unused)]
pub struct RmsVad
{
    pub voice_activity_present: f64,
    pub voice_activity_absent: f64,
    pub silent_chunk_count: usize,
    pub rms_threshold: f64,
    pub each_chunk_result: Vec<f64>,
    pub vad_results: Vec<f64>,
}
#[allow(unused)]
impl RmsVad
{
    pub fn default() -> Self
    {
        let silent_chunk_count = 3;
        let mut rms_vad = RmsVad
        {
            voice_activity_present: 1.0,
            voice_activity_absent: 0.0,
            silent_chunk_count: silent_chunk_count,
            rms_threshold: 5000.0,
            each_chunk_result: Vec::new(),
            vad_results: Vec::new()
        };
        for _ in 0 .. rms_vad.silent_chunk_count - 1
        {
            rms_vad.vad_results.push(1.0);
        }

        rms_vad
    }

    pub fn process_chunk(&mut self, chunk: &Vec<i16>) -> f64
    {
        let rms = self.calc_root_mean_square(chunk);
        if rms < self.rms_threshold
        {
            self.each_chunk_result.push(self.voice_activity_absent);
        }
        else
        {
            self.each_chunk_result.push(self.voice_activity_present);
        }
        rms
    }

    pub fn get_vad_results(&mut self) -> Vec<f64>
    {
        for i in self.silent_chunk_count - 1 .. self.each_chunk_result.len()
        {
            if self.each_chunk_result[i] == 0.0 && self.check_previous_chunks_for_silence(&i)
            {
                self.vad_results.push(self.voice_activity_absent);
            }
            else
            {
                self.vad_results.push(self.voice_activity_present);
            }
        }
        self.vad_results.clone()
    }

    fn calc_root_mean_square(&self, chunk: &Vec<i16>) -> f64
    {
        let mut square: i64 = 0;
        for elem in chunk.iter()
        {
            square += (*elem as i64).pow(2);
        }
        (square as f64 / chunk.len() as f64).sqrt()
    }

    fn check_previous_chunks_for_silence(&self, idx: &usize) -> bool
    {
        let mut previous_chunks_silent = true;
        for j in 1..self.silent_chunk_count
        {
            if self.each_chunk_result[idx - j] == self.voice_activity_present
            {
                previous_chunks_silent = false;
                break;
            }
        }
        previous_chunks_silent
    }
}