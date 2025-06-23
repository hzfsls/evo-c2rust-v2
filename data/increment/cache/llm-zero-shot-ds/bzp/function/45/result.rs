pub fn bzp_stream_finish(stream: Option<Box<BzpStream>>) {
    if let Some(stream) = stream {
        drop(stream); // Explicitly dropping the Box will deallocate the memory
    }
}
