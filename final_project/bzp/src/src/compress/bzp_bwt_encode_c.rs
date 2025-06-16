use crate::translation_utils::*;
pub use crate::src::compress::bzp_bwt_encode_h::*;

pub fn BzpBlockSortInit(mut blockSize: i32) -> Ptr<BzpBwtInfo> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return NULL!();
    }
    let mut bwt: Ptr<BzpBwtInfo> = c_malloc!(c_sizeof!(BzpBwtInfo));
    if (bwt == NULL!()).as_bool() {
        return NULL!();
    }

    c_memset_s!(bwt, c_sizeof!(BzpBwtInfo), 0, c_sizeof!(BzpBwtInfo)).cast::<Void>();

    let mut spaceSize: i32 = blockSize * BZP_BASE_BLOCK_SIZE!();
    bwt.nBlockMax = (spaceSize - BZP_BLOCK_RESERVED_SPACE_SIZE!()).cast();
    bwt.block = c_malloc!(spaceSize * c_sizeof!(u8));
    bwt.sortBlock = c_malloc!(spaceSize * c_sizeof!(i32));
    bwt.idx = c_malloc!(spaceSize * c_sizeof!(i32));
    bwt.isStartPos = c_malloc!(spaceSize * c_sizeof!(i32));
    if (bwt.block == NULL!()).as_bool() || (bwt.sortBlock == NULL!()).as_bool() || (bwt.idx == NULL!()).as_bool() || (bwt.isStartPos == NULL!()).as_bool() {
        BzpBwtFinish(bwt.cast());
        return NULL!();
    }

    c_memset_s!(bwt.isStartPos, spaceSize * c_sizeof!(i32), 0, spaceSize * c_sizeof!(i32)).cast::<Void>();
    bwt.blockCRC = BZP_INIT_BLOCK_CRC!();
    return bwt.cast();
}


pub fn BzpShellSort(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut l: i32, mut r: i32) {
    let mut increments: Array<i32, 2> = arr![BZP_SHELL_SORT_INCREMENT1!(), BZP_SHELL_SORT_INCREMENT0!()];
    let mut i: i32 = Default::default();
    let mut j: i32 = Default::default();
    if (l >= r).as_bool() {
        return;
    }
    c_for!(let mut id: i32 = 0; id < BZP_SHELL_SORT_INCREMENT_NUMS!(); id.suffix_plus_plus(); {
        let mut H: i32 = increments[id];
        if (r - l + 1 <= H).as_bool() {
            continue;
        }
        c_for!(i = l + H; i <= r; i.suffix_plus_plus(); {
            let mut tmpIdx: i32 = sortBlock[i];
            let mut tmpVal: i32 = idx[tmpIdx];
            c_for!(j = i - H; j >= l && idx[sortBlock[j]] > tmpVal; j -= H; {
                sortBlock[j + H] = sortBlock[j];
            });
            sortBlock[j + H] = tmpIdx;
        });
    });
}


pub fn BzpSwap2Elem(mut sortBlock: Ptr<i32>, mut lPos: i32, mut rPos: i32) {
    let mut value: i32 = sortBlock[lPos].cast();
    sortBlock[lPos] = sortBlock[rPos].cast();
    sortBlock[rPos] = value.cast();
}


pub fn BzpSwap3Elem(mut sortBlock: Ptr<i32>, mut lPos: i32, mut ePos: i32, mut rPos: i32) {
    let mut value: i32 = sortBlock[lPos].cast();
    sortBlock[lPos] = sortBlock[rPos].cast();
    sortBlock[rPos] = sortBlock[ePos].cast();
    sortBlock[ePos] = value.cast();
}


pub fn BzpSelectMidVal(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut l: i32, mut r: i32) -> i32 {
    let mut mid: i32 = (l + r) >> 1;
    let mut vl: i32 = idx[sortBlock[l]].cast();
    let mut vmid: i32 = idx[sortBlock[mid]].cast();
    let mut vr: i32 = idx[sortBlock[r]].cast();
    if (vl > vr).as_bool() {
        let mut tmp: i32 = l.cast();
        l = r.cast();
        r = tmp.cast();
        vl = idx[sortBlock[l]].cast();
        vr = idx[sortBlock[r]].cast();
    }
    if (vmid <= vl).as_bool() {
        return vl.cast();
    } else if (vmid <= vr).as_bool() {
        return vmid.cast();
    } else {
        return vr.cast();
    }
}


pub fn BzpQSortSingle(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut stack: Ptr<BzpQSortInfo>) {
    let mut tl: i32 = stack.tl;
    let mut tr: i32 = stack.tr;
    let mut value: i32 = BzpSelectMidVal(sortBlock, idx, tl, tr);
    let mut lPos: i32 = tl;
    let mut rPos: i32 = tr;
    let mut ePos: i32 = tl;

    while (ePos <= rPos) {
        if (idx[sortBlock[ePos]] < value) {
            BzpSwap2Elem(sortBlock, ePos, lPos);
            ePos += 1;
            lPos += 1;
        } else if (idx[sortBlock[ePos]] == value) {
            ePos += 1;
        } else {
            while (rPos >= ePos) && (idx[sortBlock[rPos]] > value) {
                rPos -= 1;
            }
            if (rPos < ePos) {
                break;
            }
            if (idx[sortBlock[rPos]] == value) {
                BzpSwap2Elem(sortBlock, ePos, rPos);
            } else if (lPos == ePos) {
                BzpSwap2Elem(sortBlock, ePos, rPos);
                lPos += 1;
            } else {
                BzpSwap3Elem(sortBlock, lPos, ePos, rPos);
                lPos += 1;
            }
            ePos += 1;
            rPos -= 1;
        }
    }

    if (lPos - tl > tr - rPos) {
        let tmp0 = stack.cnt;
        stack.stackL[tmp0] = tl;
        let tmp0 = stack.cnt;
        stack.stackR[tmp0] = (lPos - 1);
        stack.cnt += 1;
        let tmp0 = stack.cnt;
        stack.stackL[tmp0] = (rPos + 1);
        let tmp0 = stack.cnt;
        stack.stackR[tmp0] = tr;
        stack.cnt += 1;
    } else {
        let tmp0 = stack.cnt;
        stack.stackL[tmp0] = (rPos + 1);
        let tmp0 = stack.cnt;
        stack.stackR[tmp0] = tr;
        stack.cnt += 1;
        let tmp0 = stack.cnt;
        stack.stackL[tmp0] = tl;
        let tmp0 = stack.cnt;
        stack.stackR[tmp0] = (lPos - 1);
        stack.cnt += 1;
    }
}


pub fn BzpQuickSort(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut l: i32, mut r: i32) {
    let mut stack: BzpQSortInfo = Default::default();
    stack.cnt = 0;
    stack.stackL[stack.cnt] = l.cast();
    stack.stackR[stack.cnt] = r.cast();
    stack.cnt += 1;
    while (stack.cnt > 0).as_bool() {
        stack.cnt -= 1;
        let mut tl: i32 = stack.stackL[stack.cnt].cast();
        let mut tr: i32 = stack.stackR[stack.cnt].cast();
        if (tl >= tr).as_bool() {
            continue;
        }
        if (tr - tl < BZP_THRESHOLD_SHELL_SORT!()).as_bool() {
            BzpShellSort(sortBlock.cast(), idx.cast(), tl.cast(), tr.cast());
            continue;
        }
        stack.tl = tl.cast();
        stack.tr = tr.cast();
        BzpQSortSingle(sortBlock.cast(), idx.cast(), c_ref!(stack).cast());
    }
}


pub fn BzpUpdateflag(mut bwt: Ptr<BzpBwtInfo>, mut l: i32, mut r: i32) {
    let mut tmpst: i32 = -1;
    c_for!(let mut i: i32 = l; i <= r; i.suffix_plus_plus(); {
        let mut tmpnow: i32 = bwt.idx[bwt.sortBlock[i]];
        if (tmpst != tmpnow).as_bool() {
            bwt.isStartPos[i] = 1;
            tmpst = tmpnow;
        }
    });
}


pub fn BzpBinaryLiftingSort(mut bwt: Ptr<BzpBwtInfo>) {
    let mut ftab: Array<i32, { BZP_ASCII_SIZE!() }> = Default::default();
    c_memset_s!(ftab, c_sizeofval!(ftab), 0, c_sizeofval!(ftab)).cast::<Void>();
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        ftab[bwt.block[i]] += 1;
    });
    c_for!(let mut i: i32 = 1; i < BZP_ASCII_SIZE!().cast(); i.suffix_plus_plus(); {
        ftab[i] += ftab[i - 1];
    });
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        let mut ch: i32 = bwt.block[i].cast();
        ftab[ch] -= 1;
        bwt.sortBlock[ftab[ch]] = i.cast();
    });
    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!().cast(); i.suffix_plus_plus(); {
        bwt.isStartPos[ftab[i]] = 1;
    });
    let mut M: i32 = 1;
    let mut sortflag: bool = true;
    while (M < bwt.nBlock).as_bool() && sortflag.as_bool() {
        let mut st: i32 = 0;
        sortflag = false;
        c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
            if bwt.isStartPos[i].as_bool() {
                st = i.cast();
            }
            let mut pos: i32 = bwt.sortBlock[i] - M;
            if (pos < 0).as_bool() {
                pos += bwt.nBlock;
            }
            bwt.idx[pos] = st.cast();
        });
        let mut l: i32 = 0;
        let mut r: i32 = 1;
        while (l < bwt.nBlock).as_bool() {
            while (r < bwt.nBlock).as_bool() && (!bwt.isStartPos[r].as_bool()).as_bool() {
                r += 1;
            }
            r -= 1;
            if (l < r).as_bool() {
                sortflag = true;
                BzpQuickSort(bwt.sortBlock.cast(), bwt.idx.cast(), l.cast(), r.cast());
                BzpUpdateflag(bwt.cast(), l.cast(), r.cast());
            }
            l = r + 1;
            r = l + 1;
        }
        M <<= 1;
    }
}


pub fn BzpBlockSortMain(mut bwt: Ptr<BzpBwtInfo>) {
    BzpBinaryLiftingSort(bwt.cast());
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        if (bwt.sortBlock[i] == 0).as_bool() {
            bwt.oriPtr = i.cast();
            break;
        }
    });
}


pub fn BzpBwtFinish(mut bwt: Ptr<BzpBwtInfo>) {
    if (bwt != NULL!()).as_bool() {
        if (bwt.block != NULL!()).as_bool() {
            c_free!(bwt.block);
            bwt.block = NULL!();
        }
        if (bwt.sortBlock != NULL!()).as_bool() {
            c_free!(bwt.sortBlock);
            bwt.sortBlock = NULL!();
        }
        if (bwt.idx != NULL!()).as_bool() {
            c_free!(bwt.idx);
            bwt.idx = NULL!();
        }
        if (bwt.isStartPos != NULL!()).as_bool() {
            c_free!(bwt.isStartPos);
            bwt.isStartPos = NULL!();
        }
        c_free!(bwt);
        bwt = NULL!();
    }
}


