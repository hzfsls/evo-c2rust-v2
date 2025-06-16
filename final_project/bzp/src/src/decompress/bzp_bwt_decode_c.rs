use crate::translation_utils::*;
pub use crate::src::decompress::bzp_bwt_decode_h::*;

pub fn BzpBwtDecodeInit(mut blockSize: i32) -> Ptr<BzpBwtDecodeInfo> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return NULL!();
    }
    let mut bwt: Ptr<BzpBwtDecodeInfo> = c_malloc!(c_sizeof!(BzpBwtDecodeInfo));
    if (bwt == NULL!()).as_bool() {
        return NULL!();
    }
    let mut spaceSize: i32 = BZP_BASE_BLOCK_SIZE!() * blockSize;
    bwt.block = c_malloc!(spaceSize * c_sizeof!(u8));
    bwt.deCode = c_malloc!(spaceSize * c_sizeof!(u8));
    bwt.sorted = c_malloc!(spaceSize * c_sizeof!(i32));
    if (bwt.block == NULL!()).as_bool() || (bwt.sorted == NULL!()).as_bool() || (bwt.deCode == NULL!()).as_bool() {
        BzpBwtDecodeFinish(bwt.cast());
        return NULL!();
    }
    bwt.nBlock = 0;
    bwt.oriPtr = 0;
    return bwt.cast();
}


pub fn BzpBwtDecode(mut bwt: Ptr<BzpBwtDecodeInfo>) {
    let mut ftab: Array<i32, 257> = Default::default();
    c_memset_s!(ftab, c_sizeofval!(ftab), 0, c_sizeofval!(ftab)).cast::<Void>();
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        ftab[bwt.block[i] + 1] += 1;
    });
    c_for!(let mut i: i32 = 1; i <= BZP_ASCII_SIZE!(); i.suffix_plus_plus(); {
        ftab[i] += ftab[i - 1];
    });
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        let mut ch: u8 = bwt.block[i].cast();
        bwt.sorted[ftab[ch]] = i.cast();
        ftab[ch] += 1;
    });
    let mut cnt: i32 = 0;
    let mut pos: i32 = bwt.oriPtr.cast();
    while (cnt < bwt.nBlock).as_bool() {
        pos = bwt.sorted[pos].cast();
        let mut ch: u8 = bwt.block[pos].cast();
        bwt.deCode[cnt] = ch.cast();
        cnt += 1;
    }
}


pub fn BzpBwtDecodeFinish(mut bwt: Ptr<BzpBwtDecodeInfo>) {
    if (bwt != NULL!()).as_bool() {
        if (bwt.block != NULL!()).as_bool() {
            c_free!(bwt.block);
            bwt.block = NULL!();
        }
        if (bwt.deCode != NULL!()).as_bool() {
            c_free!(bwt.deCode);
            bwt.deCode = NULL!();
        }
        if (bwt.sorted != NULL!()).as_bool() {
            c_free!(bwt.sorted);
            bwt.sorted = NULL!();
        }
        c_free!(bwt);
        bwt = NULL!();
    }
}


