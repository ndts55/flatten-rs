use std::path::{Path, PathBuf};
use std::{self, env, fs, io};

pub fn determine_path(input: Option<&String>) -> io::Result<PathBuf> {
    let default_dir = String::from(".");
    let dir = input.unwrap_or(&default_dir);
    let path = Path::new(dir);
    path.canonicalize()
}

pub fn flatten_old(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let path_name = path.file_name().unwrap().to_str().unwrap().to_owned();
            if path.is_dir() {
                env::set_current_dir(&path)?;
                for child in fs::read_dir(&path)? {
                    let child = child?;
                    let file_name = child.file_name();
                    let name = file_name.to_str().unwrap();
                    let new_name = path_name.clone() + " - " + name;
                    fs::rename(name, "../".to_string() + &new_name)?;
                }
            }
        }
    }

    Ok(())
}
