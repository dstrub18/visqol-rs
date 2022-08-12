use serde::{Deserialize, Serialize};

// todo: Ditch this effin FilePath. You made the alignment and xcorr work :) Be proud of yourself :)
#[derive(Deserialize, Serialize)]
pub struct ReferenceDegradedPathPair
{
    pub reference: String,
    pub degraded: String
}
impl ReferenceDegradedPathPair
{
    pub fn new(reference: &str, degraded: &str)
    -> Self     
    {
        Self
        {
            reference: String::from(reference),
            degraded: String::from(degraded)
        }
    }
}