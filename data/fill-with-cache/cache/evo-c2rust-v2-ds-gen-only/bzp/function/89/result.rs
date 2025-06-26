pub fn BzpGetOneTable(mut huffman: Ptr<BzpHuffmanDecode>, mut t: i32) {
    let mut vec: i32 = 0;
    let mut cnt: i32 = 0;
    let mut mi: i32 = huffman.len[t][0].cast();
    let mut mx: i32 = huffman.len[t][0].cast();
    c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        mi = BZP_MIN_FUN!(mi, huffman.len[t][i]);
        mx = BZP_MAX_FUN!(mx, huffman.len[t][i]);
    });
    huffman.minLens[t] = mi.cast();
    c_for!(let mut i: i32 = mi; i <= mx; i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < huffman.alphaSize; j.suffix_plus_plus(); {
            if (huffman.len[t][j] == i).as_bool() {
                huffman.perm[t][cnt] = j.cast();
                cnt += 1;
            }
        });
    });
    c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        huffman.base[t][huffman.len[t][i] + 1] += 1;
    });
    c_for!(let mut i: i32 = 1; i <= mx + 1; i.suffix_plus_plus(); {
        huffman.base[t][i] += huffman.base[t][i - 1];
    });
    c_for!(let mut i: i32 = mi; i <= mx; i.suffix_plus_plus(); {
        vec += (huffman.base[t][i + 1] - huffman.base[t][i]);
        huffman.limit[t][i] = (vec - 1).cast();
        vec <<= 1;
    });
    c_for!(let mut i: i32 = mi + 1; i <= mx; i.suffix_plus_plus(); {
        huffman.base[t][i] = (((huffman.limit[t][i - 1] + 1) << 1) - huffman.base[t][i]).cast();
    });
}
