pub fn BzpHuffmanDecodeInit(mut blockSize: i32) -> Ptr<BzpHuffmanDecode> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize) {
        return NULL!();
    }
    let mut huffman: Ptr<BzpHuffmanDecode> = c_malloc!(c_sizeof!(BzpHuffmanDecode));
    if huffman == NULL!() {
        return NULL!();
    }
    let mut spaceSize: i32 = BZP_BASE_BLOCK_SIZE!() * blockSize / BZP_ELEMS_NUM_IN_ONE_GROUP!();
    huffman.select = c_malloc!(spaceSize * c_sizeof!(i32));
    if huffman.select == NULL!() {
        BzpHuffmanDecodeFinish(huffman.cast());
    }
    c_memset_s!(huffman.base, c_sizeofval!(huffman.base), 0, c_sizeofval!(huffman.base)).cast::<Void>();
    c_memset_s!(huffman.perm, c_sizeofval!(huffman.perm), 0, c_sizeofval!(huffman.perm)).cast::<Void>();
    c_memset_s!(huffman.limit, c_sizeofval!(huffman.limit), 0, c_sizeofval!(huffman.limit)).cast::<Void>();
    huffman.selectCnt = 0;
    huffman.deCodeNum = 0;
    return huffman.cast();
}
