use std::{ env, io };
use std::path::PathBuf;

pub fn exe_dir() -> io::Result<PathBuf> {
    env::current_exe()?
        .parent()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Executable has no parent directory"))
}