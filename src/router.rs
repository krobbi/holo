use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    error::{Error, Result},
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
            return match list_dir(&path) {
                Ok(names) => Page::Index(uri.into(), names),
                Err(error) => error_page(&error),
            };
        }

        path.push("index.html");

        if !path.is_file() {
            return Page::Error(Status::NotFound);
        }
    } else if is_dir_uri {
        return Page::Error(Status::NotFound);
    }

    match fs::read(&path).map_err(Error::FileRead) {
        Ok(contents) => {
            let media_type = mime_guess::from_path(&path).first_raw();
            Page::File(media_type, contents)
        }
        Err(error) => error_page(&error),
    }
}

/// Resolves a path from a root [`Path`] and a URI. Returns [`None`] if the
/// resolved path is not an existing descendant of the root path.
fn resolve_path(root: &Path, uri: &str) -> Option<PathBuf> {
    let path = root.join(uri.trim_start_matches('/')).canonicalize().ok()?;
    path.starts_with(root).then_some(path)
}

/// Returns a sorted [`Vec`] of directory and file names from a directory
/// [`Path`].
fn list_dir(path: &Path) -> Result<Vec<String>> {
    let mut dir_names = Vec::new();
    let mut file_names = Vec::new();

    for entry in path.read_dir().map_err(Error::DirRead)? {
        let entry = entry.map_err(Error::DirRead)?;
        let file_type = entry.file_type().map_err(Error::DirRead)?;
        let mut name = entry.file_name().to_string_lossy().to_string();

        if file_type.is_dir() {
            name.push('/');
            dir_names.push(name);
        } else if file_type.is_file() {
            file_names.push(name);
        }
    }

    dir_names.sort_unstable();
    file_names.sort_unstable();
    dir_names.append(&mut file_names);
    Ok(dir_names)
}

/// Prints an [`Error`] and returns an internal server error [`Page`].
fn error_page(error: &Error) -> Page {
    error.print();
    Page::Error(Status::InternalServerError)
}
