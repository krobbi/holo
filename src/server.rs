use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{
    config::Config,
    request::Request,
    response::{Page, Response, Status},
};

/// Respond to a request.
pub fn respond(request: &Request, config: &Config) -> Response {
    let page = serve_page(request, config);
    Response::from_page(page)
}

/// Serve a page using a request.
fn serve_page(request: &Request, config: &Config) -> Page {
    if !request.loopback() {
        return Page::Error(Status::Forbidden);
    }

    let root = config.root();
    let url = request.url();

    let Some(mut path) = resolve_path(root, url) else {
        return Page::Error(Status::NotFound);
    };

    if path.is_dir() {
        if !url.ends_with('/') {
            return Page::Redirect(format!("{url}/"));
        }

        path.push("index.html");

        if !path.is_file() {
            return if config.index() {
                Page::Index(url.to_string())
            } else {
                Page::Error(Status::NotFound)
            };
        }
    } else if url.ends_with('/') {
        return Page::Error(Status::NotFound);
    }

    match fs::read(&path) {
        Ok(body) => {
            let media_type = new_mime_guess::from_path(&path).first_raw();
            Page::File(media_type, body)
        }
        Err(error) => serve_internal_error(&error),
    }
}

/// Serve an internal server error page using an I/O error.
fn serve_internal_error(error: &io::Error) -> Page {
    eprintln!("{error}");
    Page::Error(Status::InternalServerError)
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
