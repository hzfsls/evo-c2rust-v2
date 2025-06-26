use std::ptr;

pub struct BzpHuffmanDecode {
    pub base: [u32; 256],
    pub perm: [u32; 256],
    pub limit: [u32; 256],
    pub select_cnt: u32,
    pub de_code_num: u32,
}

pub fn bzp_huffman_decode_reset(huffman: &mut BzpHuffmanDecode) {
    unsafe {
        ptr::write_bytes(huffman.base.as_mut_ptr(), 0, huffman.base.len());
        ptr::write_bytes(huffman.perm.as_mut_ptr(), 0, huffman.perm.len());
        ptr::write_bytes(huffman.limit.as_mut_ptr(), 0, huffman.limit.len());
    }
    huffman.select_cnt = 0;
    huffman.de_code_num = 0;
}
