use std::{fs, path::PathBuf};

use crate::{
    request::Request,
    response::{Content, Status},
};

/// Route an HTTP request to content.
pub fn route_request(request: &Request) -> Result<Content, Status> {
    if !request.loopback() {
        return Err(Status::Forbidden);
    }

    let path = PathBuf::from(request.url());
    read_file(path)
}

/// Read a file's content from its path.
fn read_file(path: PathBuf) -> Result<Content, Status> {
    if !path.is_file() {
        return Err(Status::NotFound);
    }

    match fs::read(&path) {
        Ok(data) => Ok(Content::new(path, data)),
        Err(_) => Err(Status::InternalServerError),
    }
}
