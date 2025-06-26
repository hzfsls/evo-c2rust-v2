pub unsafe fn BzpHuffmanGroupsFinish(huffman: *mut BzpHuffmanGroups) {
    if !huffman.is_null() {
        let huffman_ref = &mut *huffman;
        if !huffman_ref.select.is_null() {
            free(huffman_ref.select);
            huffman_ref.select = std::ptr::null_mut();
        }
        if !huffman_ref.selectMTF.is_null() {
            free(huffman_ref.selectMTF);
            huffman_ref.selectMTF = std::ptr::null_mut();
        }
        free(huffman);
    }
}
