use crate::translation_utils::*;
pub use crate::src::decompress::bzp_huffman_decode_h::*;

pub fn BzpHuffmanDecodeInit(mut blockSize: i32) -> Ptr<BzpHuffmanDecode> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return NULL!();
    }
    let mut huffman: Ptr<BzpHuffmanDecode> = c_malloc!(c_sizeof!(BzpHuffmanDecode));
    if (huffman == NULL!()).as_bool() {
        return NULL!();
    }
    let mut spaceSize: i32 = BZP_BASE_BLOCK_SIZE!() * blockSize / BZP_ELEMS_NUM_IN_ONE_GROUP!();
    huffman.select = c_malloc!(spaceSize * c_sizeof!(i32));
    if (huffman.select == NULL!()).as_bool() {
        BzpHuffmanDecodeFinish(huffman.cast());
    }
    c_memset_s!(huffman.base, c_sizeofval!(huffman.base), 0, c_sizeofval!(huffman.base)).cast::<Void>();
    c_memset_s!(huffman.perm, c_sizeofval!(huffman.perm), 0, c_sizeofval!(huffman.perm)).cast::<Void>();
    c_memset_s!(huffman.limit, c_sizeofval!(huffman.limit), 0, c_sizeofval!(huffman.limit)).cast::<Void>();
    huffman.selectCnt = 0;
    huffman.deCodeNum = 0;
    return huffman.cast();
}


pub fn BzpHuffmanDecodeReset(mut huffman: Ptr<BzpHuffmanDecode>) {
    c_memset_s!(huffman.base, c_sizeofval!(huffman.base), 0, c_sizeofval!(huffman.base)).cast::<Void>();
    c_memset_s!(huffman.perm, c_sizeofval!(huffman.perm), 0, c_sizeofval!(huffman.perm)).cast::<Void>();
    c_memset_s!(huffman.limit, c_sizeofval!(huffman.limit), 0, c_sizeofval!(huffman.limit)).cast::<Void>();

    huffman.selectCnt = 0;
    huffman.deCodeNum = 0;
}


pub fn BzpGetOneTable(mut huffman: Ptr<BzpHuffmanDecode>, mut t: i32) {
    let mut vec: i32 = 0;
    let mut cnt: i32 = 0;
    let mut mi: i32 = huffman.len[t][0];
    let mut mx: i32 = huffman.len[t][0];
    c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        mi = BZP_MIN_FUN!(mi, huffman.len[t][i]);
        mx = BZP_MAX_FUN!(mx, huffman.len[t][i]);
    });
    huffman.minLens[t] = mi;
    c_for!(let mut i: i32 = mi; i <= mx; i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < huffman.alphaSize; j.suffix_plus_plus(); {
            if (huffman.len[t][j] == i) {
                huffman.perm[t][cnt] = j;
                cnt += 1;
            }
        });
    });
    c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        let len = huffman.len[t][i];
        huffman.base[t][len + 1] += 1;
    });
    c_for!(let mut i: i32 = 1; i <= mx + 1; i.suffix_plus_plus(); {
        let tmp0 = t;
        huffman.base[tmp0][i - 1];
    });
    c_for!(let mut i: i32 = mi; i <= mx; i.suffix_plus_plus(); {
        vec += (huffman.base[t][i + 1] - huffman.base[t][i]);
        huffman.limit[t][i] = (vec - 1);
        vec <<= 1;
    });
    c_for!(let mut i: i32 = mi + 1; i <= mx; i.suffix_plus_plus(); {
        huffman.base[t][i] = (((huffman.limit[t][i - 1] + 1) << 1) - huffman.base[t][i]);
    });
}


pub fn BzpGenerateDecodeTable(mut huffman: Ptr<BzpHuffmanDecode>) {
    c_for!(let mut t: i32 = 0; t < huffman.nGroups; t.suffix_plus_plus(); {
        BzpGetOneTable(huffman.cast(), t.cast());
    });
}


pub fn BzpHuffmanDecodeFinish(mut huffman: Ptr<BzpHuffmanDecode>) {
    if (huffman != NULL!()).as_bool() {
        if (huffman.select != NULL!()).as_bool() {
            c_free!(huffman.select);
            huffman.select = NULL!();
        }
        c_free!(huffman);
        huffman = NULL!();
    }
}


