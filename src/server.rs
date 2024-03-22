use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{
    config::Config,
    request::Request,
    response::{Page, Response, Status},
};

/// Reserved HTTP characters, excluding slashes.
const RESERVED_CHARS: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
    .add(b' ')
    .add(b'!')
    .add(b'"')
    .add(b'#')
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'*')
    .add(b'+')
    .add(b',')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b']');

/// Percent encode a URL component.
pub fn percent_encode(component: &str) -> String {
    percent_encoding::utf8_percent_encode(component, RESERVED_CHARS).to_string()
}

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
                let path = path.parent().unwrap();
                let include_parent = url != "/";

                let names = match list_dir(path, include_parent) {
                    Ok(names) => names,
                    Err(error) => return serve_internal_error(&error),
                };

                let url = percent_encoding::percent_decode_str(url)
                    .decode_utf8_lossy()
                    .to_string();

                Page::Index(url, names)
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

/// List a directory, sorted by entry kind and name.
fn list_dir(path: &Path, include_parent: bool) -> io::Result<Vec<String>> {
    let mut dir_names = Vec::new();
    let mut file_names = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let mut file_name = entry.file_name().to_string_lossy().to_string();

        if file_type.is_dir() {
            file_name.push('/');
            dir_names.push(file_name);
        } else if file_type.is_file() {
            file_names.push(file_name);
        }
    }

    let mut names = Vec::new();

    if include_parent {
        names.push("..".to_string());
    }

    dir_names.sort_unstable();
    file_names.sort_unstable();
    names.append(&mut dir_names);
    names.append(&mut file_names);
    Ok(names)
}
