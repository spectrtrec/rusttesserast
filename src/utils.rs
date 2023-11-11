use std::{env::current_dir, path::PathBuf};

pub fn get_current_working_dir() -> PathBuf {
    let mut cur_dir = current_dir().unwrap();
    return current_dir().unwrap().to_owned();
}
