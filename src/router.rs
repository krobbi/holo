use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    http::{self, Request, Status},
    page::Page,
};

/// Finds a [`Page`] to return as a response to an HTTP [`Request`].
pub fn find_page(request: &Request) -> Page {
    if !request.is_local() {
        return Page::Error(Status::Forbidden);
    }

    let config = request.config();
    let uri = request.uri();

    let Some(mut path) = resolve_path(config.root(), uri) else {
        return Page::Error(Status::NotFound);
    };

    let is_dir_uri = uri.ends_with('/');

    if path.is_dir() {
        if !is_dir_uri {
            let mut uri = http::encode_uri(uri);
            uri.push('/');
            return Page::Redirect(uri);
        }

        if config.is_serving_index_pages() {
            return Page::Index(uri.into());
        }

        path.push("index.html");

        if !path.is_file() {
            return Page::Error(Status::NotFound);
        }
    } else if is_dir_uri {
        return Page::Error(Status::NotFound);
    }

    match fs::read(&path) {
        Ok(contents) => {
            let media_type = mime_guess::from_path(&path).first_raw();
            Page::File(media_type, contents)
        }
        Err(_) => Page::Error(Status::InternalServerError),
    }
}

/// Resolves a path from a root [`Path`] and a URI. Returns [`None`] if the
/// resolved path is not an existing descendant of the root path.
fn resolve_path(root: &Path, uri: &str) -> Option<PathBuf> {
    let path = root.join(uri.trim_start_matches('/')).canonicalize().ok()?;
    path.starts_with(root).then_some(path)
}
