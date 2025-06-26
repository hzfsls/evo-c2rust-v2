pub fn bzp_huffman_decode_finish(huffman: Option<&mut BzpHuffmanDecode>) {
    if let Some(huffman) = huffman {
        if huffman.select.is_some() {
            unsafe {
                // Assuming select is a raw pointer that needs to be freed
                let select_ptr = huffman.select.take().unwrap();
                libc::free(select_ptr as *mut libc::c_void);
            }
        }
        
        // Note: In Rust, we can't free the huffman object itself here because
        // we only have a mutable reference, not ownership. The caller would need
        // to handle freeing the main huffman object if needed.
        // If huffman was passed as a Box, the caller should use Box::from_raw()
    }
}
