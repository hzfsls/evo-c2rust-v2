pub fn bzp_stream_finish(stream: Option<Box<BzpStream>>) {
    if let Some(stream) = stream {
        // The Box will be automatically dropped (freed) when it goes out of scope
        // No need to explicitly set to null, Rust's ownership system handles this
    }
}
