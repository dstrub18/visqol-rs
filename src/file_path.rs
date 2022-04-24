use std::path::PathBuf;
pub struct FilePath
{
    path: PathBuf
}

impl FilePath
{
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

// static methods
pub fn current_working_dir() -> String
{
    std::env::current_dir().unwrap().into_os_string().into_string().unwrap()
}
#[allow(unused)]
struct ReferenceDegradedPathPair
{
    reference: FilePath,
    degraded: FilePath
}