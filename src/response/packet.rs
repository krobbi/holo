/// An HTTP response packet.
pub(super) trait Packet {
    /// Put a status line to the packet.
    fn put_status_line(&mut self, code: u16, reason: &str) {
        self.put_content(format!("HTTP/1.1 {code} {reason}\r\n").into_bytes());
    }

    /// Put a header to the packet.
    fn put_header(&mut self, key: &str, value: &str) {
        self.put_content(format!("{key}: {value}\r\n").into_bytes());
    }

    /// Put an end-of-headers marker to the packet.
    fn put_end_of_headers(&mut self) {
        self.put_bytes(b"\r\n");
    }

    /// Put content to the packet.
    fn put_content(&mut self, content: Vec<u8>);

    /// Put a byte slice to the packet.
    fn put_bytes(&mut self, bytes: &[u8]);
}

impl Packet for Vec<u8> {
    fn put_content(&mut self, mut content: Vec<u8>) {
        self.append(&mut content);
    }

    fn put_bytes(&mut self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }
}
