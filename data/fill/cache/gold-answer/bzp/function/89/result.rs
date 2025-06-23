pub fn BzpGetOneTable(mut huffman: Ptr<BzpHuffmanDecode>, mut t: i32) {
    let mut vec: i32 = 0;
    let mut cnt: i32 = 0;
    let mut mi: i32 = huffman.len[t][0];
    let mut mx: i32 = huffman.len[t][0];
    c_for!(let mut i = 0; i < huffman.alphaSize; i += 1; {
        mi = BZP_MIN_FUN!(mi, huffman.len[t][i]);
        mx = BZP_MAX_FUN!(mx, huffman.len[t][i]);
    });
    huffman.minLens[t] = mi;
    c_for!(let mut i = mi; i <= mx; i += 1; {
        c_for!(let mut j = 0; j < huffman.alphaSize; j += 1; {
            if huffman.len[t][j] == i {
                huffman.perm[t][cnt] = j;
                cnt += 1;
            }
        });
    });
    c_for!(let mut i = 0; i < huffman.alphaSize; i += 1; {
        let index = huffman.len[t][i] + 1;
        huffman.base[t][index] += 1;
    });
    c_for!(let mut i = 1; i <= mx + 1; i += 1; {
        huffman.base[t][i] += huffman.base[t][i - 1];
    });
    c_for!(let mut i = mi; i <= mx; i += 1; {
        vec += huffman.base[t][i + 1] - huffman.base[t][i];
        huffman.limit[t][i] = vec - 1;
        vec <<= 1;
    });
    c_for!(let mut i = mi + 1; i <= mx; i += 1; {
        huffman.base[t][i] = ((huffman.limit[t][i - 1] + 1) << 1) - huffman.base[t][i];
    });
}