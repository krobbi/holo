use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    config::Config,
    request::Request,
    response::{Content, Response, Status},
};

/// Respond to a request.
pub fn respond(request: &Request, config: &Config) -> Response {
    let content = serve_content(request, config);
    let mut response = Response::new(content);

    if config.cross_origin_isolation() {
        response.enable_cross_origin_isolation();
    }

    response
}

/// Serve content using a request.
fn serve_content(request: &Request, config: &Config) -> Content {
    if !request.loopback() {
        return Content::Error(Status::Forbidden);
    }

    let root = config.root();
    let url = request.url();

    let Some(mut path) = resolve_path(root, url) else {
        return Content::Error(Status::NotFound);
    };

    if path.is_dir() {
        if !url.ends_with('/') {
            // TODO: Redirect to trailing slash.
            return Content::Error(Status::NotFound);
        }

        path.push("index.html");

        if !path.is_file() {
            return Content::Error(Status::NotFound);
        }
    } else if url.ends_with('/') {
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

/// Resolve a path using a server root directory and a request URL.
fn resolve_path(root: &Path, url: &str) -> Option<PathBuf> {
    let url = &percent_encoding::percent_decode_str(url).decode_utf8_lossy()[1..];

    let Ok(path) = root.join(url).canonicalize() else {
        return None;
    };

    if path.starts_with(root) {
        Some(path)
    } else {
        None
    }
}
