pub fn BzpHuffmanDecodeReset(mut huffman: Ptr<BzpHuffmanDecode>) {
    c_memset_s!(huffman.base, c_sizeofval!(huffman.base), 0, c_sizeofval!(huffman.base)).cast::<Void>();
    c_memset_s!(huffman.perm, c_sizeofval!(huffman.perm), 0, c_sizeofval!(huffman.perm)).cast::<Void>();
    c_memset_s!(huffman.limit, c_sizeofval!(huffman.limit), 0, c_sizeofval!(huffman.limit)).cast::<Void>();

    huffman.selectCnt = 0;
    huffman.deCodeNum = 0;
}
