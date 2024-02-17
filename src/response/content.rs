use std::path::PathBuf;

use new_mime_guess::Mime;

/// A file's content.
pub struct Content {
    /// The path.
    path: PathBuf,

    /// The data.
    data: Vec<u8>,
}

impl Content {
    /// Create new content from a path and data.
    pub fn new(path: PathBuf, data: Vec<u8>) -> Content {
        Content { path, data }
    }

    /// Get the MIME type.
    pub(super) fn mime(&self) -> Option<Mime> {
        new_mime_guess::from_path(&self.path).first()
    }

    /// Get the data's length.
    pub(super) fn len(&self) -> usize {
        self.data.len()
    }

    /// Get the data as a byte slice.
    pub(super) fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}
