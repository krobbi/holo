use std::{fs, path::PathBuf};

use crate::response::Status;

/// Read a file's content using its path or return a status.
pub fn read_file(path: &PathBuf) -> Result<Vec<u8>, Status> {
    if !path.is_file() {
        return Err(Status::NotFound);
    }

    match fs::read(path) {
        Ok(content) => Ok(content),
        Err(_) => Err(Status::InternalServerError),
    }
}
