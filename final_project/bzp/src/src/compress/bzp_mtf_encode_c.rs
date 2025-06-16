use crate::translation_utils::*;
pub use crate::src::compress::bzp_mtf_encode_h::*;

pub fn BzpMtfInit(mut blockSize: i32) -> Ptr<BzpMtfInfo> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return NULL!();
    }
    let mut mtf: Ptr<BzpMtfInfo> = c_malloc!(c_sizeof!(BzpMtfInfo));
    if (mtf == NULL!()).as_bool() {
        return NULL!();
    }
    mtf.mtfV = NULL!();
    mtf.mtfV = c_malloc!(blockSize * BZP_BASE_BLOCK_SIZE!() * c_sizeof!(i32));
    if (mtf.mtfV == NULL!()).as_bool() {
        c_free!(mtf);
        mtf = NULL!();
        return NULL!();
    }
    mtf.nUse = 0;
    mtf.nMtf = 0;
    mtf.block = NULL!();
    mtf.map = NULL!();
    mtf.inUse = NULL!();
    return mtf.cast();
}


pub fn BzpMtfReSet(mut mtf: Ptr<BzpMtfInfo>) {
    mtf.nUse = 0;
    mtf.nMtf = 0;
    mtf.block = NULL!();
    mtf.map = NULL!();
    mtf.inUse = NULL!();
}


pub fn BzpMapInputChar(mut mtf: Ptr<BzpMtfInfo>, mut list: Ptr<u8>, mut lenList: i32) {
    if (BZP_ASCII_SIZE!() > lenList).as_bool() {
        return;
    }
    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!().cast(); i.suffix_plus_plus(); {
        if mtf.inUse[i].as_bool() {
            list[mtf.nUse] = i.cast::<u8>();
            mtf.nUse += 1;
        }
    });
}


pub fn BzpNumEncode(mut mtf: Ptr<BzpMtfInfo>, mut num: i32) {
    num <<= 1;
    c_do!({
        num >>= 1;
        num -= 1;
        if (num & 1) {
            let tmp0 = mtf.nMtf;
            mtf.mtfV[tmp0] = BZP_MTF_ENCODE1!();
            mtf.nMtf += 1;
            mtf.mtfFreq[BZP_MTF_ENCODE1!()] += 1;
        } else {
            mtf.mtfV[mtf.nMtf] = BZP_MTF_ENCODE0!();
            mtf.nMtf += 1;
            mtf.mtfFreq[BZP_MTF_ENCODE0!()] += 1;
        }
    } while num >= BZP_MTF_ENCODE_BASE!());
}


pub fn BzpMtfMain(mut mtf: Ptr<BzpMtfInfo>) {
    let mut list: Array<u8, { BZP_MAX_ALPHA_SIZE!() }> = Default::default();
    let mut EOB: i32 = Default::default();
    let mut num: i32 = 0;
    BzpMapInputChar(mtf, list.as_mut_ptr(), BZP_MAX_ALPHA_SIZE!());
    EOB = (mtf.nUse + 1);
    c_for!(let mut i: i32 = 0; i <= EOB; i.suffix_plus_plus(); {
        mtf.mtfFreq[i] = 0;
    });
    c_for!(let mut i: i32 = 0; i < mtf.nBlock; i.suffix_plus_plus(); {
        let mut pos: i32 = (mtf.map[i] - 1);
        if (pos < 0) {
            pos += mtf.nBlock;
        }
        let mut ch: u8 = mtf.block[pos];
        if (ch == list[0]) {
            num += 1;
        } else {
            if (num > 0) {
                BzpNumEncode(mtf, num);
                num = 0;
            }
            let mut pos_: i32 = 1;
            while (ch != list[pos_]) && (pos_ < mtf.nUse) {
                pos_ += 1;
            }
            c_for!(let mut j: i32 = pos_; j > 0; j.suffix_minus_minus(); {
                list[j] = list[j - 1];
            });
            list[0] = ch;
            let tmp0 = mtf.nMtf;
            mtf.mtfV[tmp0] = (pos_ + 1);
            mtf.mtfFreq[pos_ + 1] += 1;
            mtf.nMtf += 1;
        }
    });
    if (num > 0) {
        BzpNumEncode(mtf, num);
    }
    mtf.mtfV[mtf.nMtf] = EOB;
    mtf.mtfFreq[EOB] += 1;
    mtf.nMtf += 1;
}


pub fn BzpMtfFinish(mut mtf: Ptr<BzpMtfInfo>) {
    if (mtf != NULL!()).as_bool() {
        if (mtf.mtfV != NULL!()).as_bool() {
            c_free!(mtf.mtfV);
            mtf.mtfV = NULL!();
        }
        c_free!(mtf);
        mtf = NULL!();
    }
}


