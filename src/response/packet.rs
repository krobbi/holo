/// An HTTP response packet.
pub(super) trait Packet {
    /// Append a status line to the packet.
    fn append_status_line(&mut self, code: u16, reason: &str);

    /// Append a response header field to the packet.
    fn append_field(&mut self, key: &str, value: &str);

    /// Append a message body to the packet.
    fn append_body(&mut self, body: Vec<u8>);
}

impl Packet for Vec<u8> {
    fn append_status_line(&mut self, code: u16, reason: &str) {
        self.append(&mut format!("HTTP/1.1 {code} {reason}\r\n").into_bytes());
    }

    fn append_field(&mut self, key: &str, value: &str) {
        self.append(&mut format!("{key}: {value}\r\n").into_bytes());
    }

    fn append_body(&mut self, mut body: Vec<u8>) {
        self.extend_from_slice(b"\r\n");
        self.append(&mut body);
    }
}
