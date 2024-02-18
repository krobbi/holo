use std::{fs, path::Path};

use crate::{
    config::Config,
    request::Request,
    response::{Content, Response, Status},
};

/// Respond to a request.
pub fn respond(request: &Request, config: &Config) -> Response {
    let content = serve_request(request, config);
    let mut response = Response::new(content);

    if config.cross_origin_isolation() {
        response.enable_cross_origin_isolation();
    }

    response
}

/// Serve content using a request.
fn serve_request(request: &Request, config: &Config) -> Content {
    if !request.loopback() {
        return Content::Error(Status::Forbidden);
    }

    let root = config.root();

    let Ok(path) = root.join(request.url()).canonicalize() else {
        return Content::Error(Status::NotFound);
    };

    if path.starts_with(root) {
        serve_path(&path)
    } else {
        Content::Error(Status::NotFound)
    }
}

/// Serve content using a path.
fn serve_path(path: &Path) -> Content {
    if !path.is_file() {
        return Content::Error(Status::NotFound);
    }

    match fs::read(path) {
        Ok(data) => {
            let mime = new_mime_guess::from_path(path).first();
            Content::Page(mime, data)
        }
        Err(_) => Content::Error(Status::InternalServerError),
    }
}
