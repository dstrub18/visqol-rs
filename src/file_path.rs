use std::path::PathBuf;
use serde::Deserialize;
pub struct FilePath
{
    pub path: PathBuf
}

impl FilePath
{

    pub fn new(path_str: String)
-> FilePath     {

        let mut path = PathBuf::new();
        path.push(path_str);
        Self
        {
            path,
        }
    }
    pub fn from(other: &Self) -> Self
    {
        Self
        {
            path: other.path.clone()
        }
    }

    pub fn exists(&self) -> bool
    {
        self.path.exists()
    }
    
    pub fn path(&self) -> PathBuf
    {
        self.path.clone()
    }
}

pub fn current_working_dir() -> String
{
    std::env::current_dir().unwrap().into_os_string().into_string().unwrap()
}

// todo: Ditch this effin FilePath. You made the alignment and xcorr work :) Be proud of yourself :)
#[derive(Deserialize)]
pub struct ReferenceDegradedPathPair
{
    pub reference: PathBuf,
    pub degraded: PathBuf
}
impl ReferenceDegradedPathPair
{
    pub fn new(reference: &String, degraded: &String)
    -> Self     
    {
        Self
        {
            reference: PathBuf::from(reference),
            degraded: PathBuf::from(degraded)
        }
    }
}