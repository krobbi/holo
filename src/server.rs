use std::{fs, path::PathBuf};

use crate::{
    request::Request,
    response::{Content, Status},
};

/// Serve content using an HTTP request.
pub fn serve_content(request: &Request) -> Content {
    if !request.loopback() {
        return Content::Error(Status::Forbidden);
    }

    let path = PathBuf::from(request.url());

    if !path.is_file() {
        return Content::Error(Status::NotFound);
    }

    match fs::read(&path) {
        Ok(data) => {
            let mime = new_mime_guess::from_path(&path).first();
            Content::Page(mime, data)
        }
        Err(_) => Content::Error(Status::InternalServerError),
    }
}
