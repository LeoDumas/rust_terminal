use std::io;
use std::path::PathBuf;

pub fn get_current_dir() -> io::Result<PathBuf> {
    std::env::current_dir()
}

pub fn change_directory(path: &str) -> io::Result<()> {
    let new_path = if path.starts_with('/') {
        PathBuf::from(path)
    } else {
        std::env::current_dir()?.join(path)
    };

    if new_path.is_dir() {
        std::env::set_current_dir(&new_path)?;
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Directory not found",
        ))
    }
}

pub fn list_directory() -> io::Result<()> {
    let entries = std::fs::read_dir(".")?;
    for entry in entries {
        if let Ok(entry) = entry {
            println!("{}", entry.path().display());
        }
    }
    Ok(())
}
