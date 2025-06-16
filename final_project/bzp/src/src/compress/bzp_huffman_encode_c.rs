use crate::translation_utils::*;
pub use crate::src::compress::bzp_huffman_encode_h::*;

pub fn BzpHuffmanInit(mut alphaSize: i32, mut huffman: Ptr<BzpHuffmanInfo>) {
    c_memset_s!(huffman.len, c_sizeofval!(huffman.len), 0, c_sizeofval!(huffman.len)).cast::<Void>();
    huffman.nHeap = 0;
    huffman.nWeight = 0;
    huffman.alphaSize = alphaSize.cast();
}


pub fn BzpHuffmanInitArray(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut i: i32;
    huffman.nHeap = 0;
    huffman.nWeight = huffman.alphaSize.cast();

    c_for!(i = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        huffman.parent[i] = -1;
    });
}


pub fn BzpHeapAdjustUp(mut heap: Ptr<i32>, mut weight: Ptr<i32>, mut pos: i32) {
    let mut tmpw: i32 = weight[heap[pos]].cast();
    let mut tmpv: i32 = heap[pos].cast();
    while (pos > 1).as_bool() {
        if (tmpw < weight[heap[pos >> 1]]).as_bool() {
            heap[pos] = heap[pos >> 1].cast();
            pos >>= 1;
        } else {
            break;
        }
    }
    heap[pos] = tmpv.cast();
}


pub fn BzpHeapAdjustDown(mut heap: Ptr<i32>, mut weight: Ptr<i32>, mut nHeap: i32) {
    let mut pos: i32 = 1;
    let mut chpos: i32 = pos << 1;
    let mut tmpid: i32 = heap[pos];
    let mut tmpv: i32 = weight[tmpid];
    while (chpos <= nHeap).as_bool() {
        if ((chpos | 1) <= nHeap).as_bool() && (weight[heap[chpos]] > weight[heap[chpos | 1]]).as_bool() {
            chpos |= 1;
        }
        if (tmpv < weight[heap[chpos]]).as_bool() {
            break;
        }
        heap[pos] = heap[chpos].cast();
        pos = chpos.cast();
        chpos = pos << 1;
    }
    heap[pos] = tmpid.cast();
}


pub fn BzpHeapInit(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut i: i32 = 0;
    c_for!(i = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        huffman.nHeap += 1;
        let tmp0 = huffman.nHeap;
        huffman.heap[tmp0] = i;
        BzpHeapAdjustUp(huffman.heap.as_ptr(), huffman.weight.cast(), huffman.nHeap);
    });
}


pub fn BzpHuffmanWeightAdd(mut w1: i32, mut w2: i32) -> i32 {
    return ((w1 & 0xffffff00) + (w2 & 0xffffff00)) | (BZP_MAX_FUN!(w1 & 0x000000ff, w2 & 0x000000ff) + 1);
}


pub fn BzpBuildHuffmanTree(mut huffman: Ptr<BzpHuffmanInfo>) {
    BzpHuffmanInitArray(huffman);
    BzpHeapInit(huffman);
    let mut idx1: i32;
    let mut idx2: i32;
    while (huffman.nHeap > 1) {
        idx1 = huffman.heap[1];
        let tmp0 = 1;
        huffman.heap[tmp0];
        huffman.nHeap -= 1;
        BzpHeapAdjustDown(huffman.heap, huffman.weight.cast(), huffman.nHeap);
        idx2 = huffman.heap[1];
        huffman.heap[1] = huffman.heap[huffman.nHeap];
        huffman.nHeap -= 1;
        BzpHeapAdjustDown(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap);
        huffman.weight[huffman.nWeight] = BzpHuffmanWeightAdd(huffman.weight[idx1], huffman.weight[idx2]);
        huffman.parent[idx1] = huffman.nWeight;
        huffman.parent[idx2] = huffman.nWeight;
        huffman.parent[huffman.nWeight] = -1;
        huffman.nHeap += 1;
        huffman.heap[huffman.nHeap] = huffman.nWeight;
        huffman.nWeight += 1;
        BzpHeapAdjustUp(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap);
    }
}


pub fn BzpGetCodeLen(mut huffman: Ptr<BzpHuffmanInfo>) -> i32 {
    let mut maxlen: i32 = 0;
    BzpBuildHuffmanTree(huffman.cast());
    let mut i: i32;
    maxlen = 0;
    c_for!(i = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        let mut x: i32 = i.cast();
        let mut tlen: i32 = 0;
        while (huffman.parent[x] >= 0).as_bool() {
            x = huffman.parent[x].cast();
            tlen += 1;
        }
        huffman.len[i] = tlen.cast();
        maxlen = BZP_MAX_FUN!(maxlen, tlen);
    });
    return maxlen.cast();
}


pub fn BzpBuildTreeBalanceHeight(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut maxlen: i32 = 0;
    c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        if (huffman.weight[i] == 0).as_bool() {
            huffman.weight[i] = 1 << BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!();
        } else {
            huffman.weight[i] <<= BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!();
        }
    });
    c_do!({
        maxlen = BzpGetCodeLen(huffman.cast()).cast();
        if (maxlen > BZP_MAX_TREE_HEIGHT_ENCODE!()).as_bool() {
            c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
                let mut w: i32 = (huffman.weight[i] >> BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!()).cast();
                w = ((w >> 1) + 1).cast();
                huffman.weight[i] = (w << BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!()).cast();
            });
        }
    } while maxlen > BZP_MAX_TREE_HEIGHT_ENCODE!());
}


pub fn BzpGetHuffmanTable(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut vec: i32 = 0;
    let mut mi: i32 = huffman.len[0].cast();
    let mut mx: i32 = huffman.len[0].cast();
    c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        mi = BZP_MIN_FUN!(mi, huffman.len[i]).cast();
        mx = BZP_MAX_FUN!(mx, huffman.len[i]).cast();
    });
    c_for!(let mut i: i32 = mi; i <= mx; i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < huffman.alphaSize; j.suffix_plus_plus(); {
            if (huffman.len[j] == i).as_bool() {
                huffman.table[j] = vec.cast();
                vec += 1;
            }
        });
        vec <<= 1;
    });
}


pub fn BzpHuffmanGroupsReset(mut huffman: Ptr<BzpHuffmanGroups>, mut alphaSize: i32) -> i32 {
    if BZP_INVALID_ALPHA_SIZE!(alphaSize).as_bool() {
        return BZP_ERROR_PARAM!();
    }

    huffman.alphaSize = alphaSize;
    huffman.block = NULL!();
    huffman.mtfFreq = NULL!();
    huffman.nSelect = 0;
    huffman.nGroups = 0;

    c_for!(let mut i: i32 = 0; i < BZP_MAX_GROUPS_NUM!(); i.suffix_plus_plus(); {
        BzpHuffmanInit(alphaSize.cast(), c_ref!(huffman.huffmanGroups[i]).cast());
    });
    return BZP_OK!();
}


pub fn BzpHuffmanGroupsInit(mut blockSize: i32) -> Ptr<BzpHuffmanGroups> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return NULL!();
    }
    let mut huffmanGroups: Ptr<BzpHuffmanGroups> = c_malloc!(c_sizeof!(BzpHuffmanGroups));
    if (huffmanGroups == NULL!()).as_bool() {
        return NULL!();
    }
    huffmanGroups.select = NULL!();
    huffmanGroups.selectMTF = NULL!();
    let mut spaceSize: i32 = blockSize * BZP_BASE_BLOCK_SIZE!() / BZP_ELEMS_NUM_IN_ONE_GROUP!();
    huffmanGroups.select = c_malloc!(spaceSize * c_sizeof!(i32));
    huffmanGroups.selectMTF = c_malloc!(spaceSize * c_sizeof!(i32));
    if (huffmanGroups.select == NULL!()).as_bool() || (huffmanGroups.selectMTF == NULL!()).as_bool() {
        BzpBzpHuffmanGroupsFinish(huffmanGroups.cast());
        return NULL!();
    }
    huffmanGroups.alphaSize = 0;
    huffmanGroups.block = NULL!();
    huffmanGroups.mtfFreq = NULL!();
    huffmanGroups.nSelect = 0;
    huffmanGroups.nGroups = 0;
    c_for!(let mut i: i32 = 0; i < BZP_MAX_GROUPS_NUM!().cast(); i.suffix_plus_plus(); {
        BzpHuffmanInit(0, c_ref!(huffmanGroups.huffmanGroups[i]).cast());
    });
    return huffmanGroups.cast();
}


pub fn BzpBzpHuffmanGroupsFinish(mut huffman: Ptr<BzpHuffmanGroups>) {
    if (huffman != NULL!()).as_bool() {
        if (huffman.select != NULL!()).as_bool() {
            c_free!(huffman.select);
            huffman.select = NULL!();
        }
        if (huffman.selectMTF != NULL!()).as_bool() {
            c_free!(huffman.selectMTF);
            huffman.selectMTF = NULL!();
        }
        c_free!(huffman);
        huffman = NULL!();
    }
}


pub fn BzpGetHuffmanGroups(mut nBlock: i32) -> i32 {
    let mut nGroups: i32 = 1;
    if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT0!()).as_bool() {
        nGroups = BZP_NGROUPS_NUM_0!();
    } else if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT1!()).as_bool() {
        nGroups = BZP_NGROUPS_NUM_1!();
    } else if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT2!()).as_bool() {
        nGroups = BZP_NGROUPS_NUM_2!();
    } else if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT3!()).as_bool() {
        nGroups = BZP_NGROUPS_NUM_3!();
    } else {
        nGroups = BZP_NGROUPS_NUM_4!();
    }
    return nGroups.cast();
}


pub fn BzpGenerateSelectMTF(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = huffman.nGroups;
    let mut list: Vec<i32> = vec![0; nGroups as usize];
    c_for!(let mut i: i32 = 0; i < nGroups; i.suffix_plus_plus(); {
        list[i as usize] = i;
    });
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i.suffix_plus_plus(); {
        let mut pos: i32 = 0;
        c_for!(let mut j: i32 = 0; j < nGroups; j.suffix_plus_plus(); {
            if (huffman.select[i] == list[j as usize]) {
                pos = j;
                break;
            }
        });
        c_for!(let mut j: i32 = pos; j > 0; j.suffix_minus_minus(); {
            list[j as usize] = list[(j - 1) as usize];
        });
        let tmp0 = 0;
        list[tmp0 as usize];
        huffman.selectMTF[i] = pos;
    });
}


pub fn BzpInitLenArray(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = huffman.nGroups;
    let mut npart: i32 = nGroups;
    let mut AllFreqNum: i32 = huffman.nBlock;
    let mut st: i32 = 0;
    let mut ed: i32 = Default::default();
    while (npart > 0) {
        let mut NowFreqNum: i32 = 0;
        let mut FreqNumLimit: i32 = AllFreqNum / npart;
        ed = st - 1;
        while (ed < huffman.alphaSize - 1) && (NowFreqNum < FreqNumLimit) {
            ed += 1;
            NowFreqNum += huffman.mtfFreq[ed];
        }
        if (ed > st) && (npart != nGroups) && (npart != 1) && ((nGroups - npart) & 1).as_bool() {
            NowFreqNum -= huffman.mtfFreq[ed];
            ed -= 1;
        }
        c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
            if (i >= st) && (i <= ed) {
                huffman.huffmanGroups[npart - 1].len[i] = 0;
            } else {
                huffman.huffmanGroups[npart - 1].len[i] = BZP_HUFFMAN_LEN_MAX_COST!();
            }
        });
        npart -= 1;
        st = ed + 1;
        AllFreqNum -= NowFreqNum;
    }
}


pub fn BzpCalculateCost(mut huffman: Ptr<BzpHuffmanGroups>, mut st: i32, mut ed: i32) {
    c_memset_s!(huffman.cost, c_sizeofval!(huffman.cost), 0, c_sizeofval!(huffman.cost)).cast::<Void>();
    let mut nGroups: i32 = huffman.nGroups;
    c_for!(let mut k: i32 = st; k <= ed; k.suffix_plus_plus(); {
        c_for!(let mut t: i32 = 0; t < nGroups; t.suffix_plus_plus(); {
            huffman.cost[t] += huffman.huffmanGroups[t].len[huffman.block[k]];
        });
    });
}


pub fn BzpSelectTree(mut huffman: Ptr<BzpHuffmanGroups>) -> i32 {
    let mut id: i32 = 0;
    let mut nGroups: i32 = huffman.nGroups;
    c_for!(let mut k: i32 = 0; k < nGroups; k.suffix_plus_plus(); {
        let tmp0 = k;
        if (huffman.cost[tmp0]) {
            id = k;
        }
    });
    huffman.select[huffman.nSelect] = id;
    huffman.nSelect += 1;
    return id;
}


pub fn BzpHuffmanMain(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = BzpGetHuffmanGroups(huffman.nBlock);
    huffman.nGroups = nGroups;

    BzpInitLenArray(huffman);
    let mut st: i32 = 0;
    let mut ed: i32 = Default::default();

    c_for!(let mut i: i32 = 0; i < BZP_MAX_ITER_NUM!(); i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < nGroups; j.suffix_plus_plus(); {
            c_memset_s!(huffman.huffmanGroups[j].weight, c_sizeofval!(huffman.huffmanGroups[j].weight), 0, c_sizeofval!(huffman.huffmanGroups[j].weight)).cast::<Void>();
        });

        st = 0;
        huffman.nSelect = 0;
        while (st < huffman.nBlock) {
            ed = BZP_MIN_FUN!(huffman.nBlock, st + BZP_ELEMS_NUM_IN_ONE_GROUP!()).cast::<i32>() - 1;

            BzpCalculateCost(huffman, st, ed);

            let mut id: i32 = BzpSelectTree(huffman);

            c_for!(let mut k: i32 = st; k <= ed; k.suffix_plus_plus(); {
                let block_k = huffman.block[k];
                huffman.huffmanGroups[id].weight[block_k] += 1;
            });
            st = ed + 1;
        }

        c_for!(let mut j: i32 = 0; j < nGroups; j.suffix_plus_plus(); {
            BzpBuildTreeBalanceHeight(c_ref!(huffman.huffmanGroups[j]));
        });
    });

    BzpGenerateSelectMTF(huffman);

    c_for!(let mut i: i32 = 0; i < nGroups; i.suffix_plus_plus(); {
        BzpGetHuffmanTable(c_ref!(huffman.huffmanGroups[i]));
    });
}


