use std::ptr;

pub struct BzpHuffmanDecode {
    select: *mut u8, // Assuming select is a pointer to some data; adjust type if needed
}

pub fn bzp_huffman_decode_finish(huffman: *mut BzpHuffmanDecode) {
    if !huffman.is_null() {
        unsafe {
            let huffman_ref = &mut *huffman;
            if !huffman_ref.select.is_null() {
                // Free the select pointer's memory
                let _ = Box::from_raw(huffman_ref.select);
                huffman_ref.select = ptr::null_mut();
            }
            // Free the huffman struct itself
            let _ = Box::from_raw(huffman);
        }
    }
}
