use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    http::{Request, Status},
    page::Page,
};

/// Finds a [`Page`] to return as a response to an HTTP [`Request`].
pub fn find_page(request: &Request) -> Page {
    if !request.is_local() {
        return Page::Error(Status::Forbidden);
    }

    let Some(path) = resolve_path(request.config().root(), request.uri()) else {
        return Page::Error(Status::NotFound);
    };

    if !path.is_file() {
        return Page::Error(Status::NotFound);
    }

    match fs::read(&path) {
        Ok(contents) => Page::File(contents),
        Err(_) => Page::Error(Status::InternalServerError),
    }
}

/// Resolves a path from a root [`Path`] and a URI. Returns [`None`] if the
/// resolved path is not an existing descendant of the root path.
fn resolve_path(root: &Path, uri: &str) -> Option<PathBuf> {
    let path = root.join(uri.trim_start_matches('/')).canonicalize().ok()?;
    path.starts_with(root).then_some(path)
}
