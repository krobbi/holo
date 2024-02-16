use std::{fs, path::PathBuf};

use crate::response::{Content, Status};

/// Read a file's content from its path.
pub fn read_file(path: PathBuf) -> Result<Content, Status> {
    if !path.is_file() {
        return Err(Status::NotFound);
    }

    match fs::read(&path) {
        Ok(data) => Ok(Content::new(path, data)),
        Err(_) => Err(Status::InternalServerError),
    }
}
