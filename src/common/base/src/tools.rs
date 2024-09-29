use std::{
    fs,
    path::{self, Path},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::errors::RobustMQError;

pub fn create_fold(fold: &str) -> Result<(), RobustMQError> {
    if !Path::new(fold).exists() {
        fs::create_dir_all(fold)?
    }
    return Ok(());
}

pub fn read_file(path: &str) -> Result<String, RobustMQError> {
    if !path::Path::new(path).exists() {
        return Err(RobustMQError::CommonError(format!(
            "File {} does not exist",
            path
        )));
    }

    return Ok(fs::read_to_string(&path)?);
}

pub fn file_exist(path: &str) -> bool {
    return Path::new(path).exists();
}

pub fn now_second() -> u64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
}
