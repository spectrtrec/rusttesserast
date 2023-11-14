use std::{env::current_dir, path::PathBuf};

pub fn get_current_working_dir() -> PathBuf {
    return current_dir().unwrap().to_owned();
}
