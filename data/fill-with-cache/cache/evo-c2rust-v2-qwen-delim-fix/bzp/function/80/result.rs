pub fn BzpInitLenArray(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = huffman.nGroups.cast();
    let mut npart: i32 = nGroups.cast();
    let mut AllFreqNum: i32 = huffman.nBlock.cast();
    let mut st: i32 = 0;
    let mut ed: i32;
    while (npart > 0).as_bool() {
        let mut NowFreqNum: i32 = 0;
        let mut FreqNumLimit: i32 = AllFreqNum / npart;
        ed = st - 1;
        while (ed < huffman.alphaSize - 1).as_bool() && (NowFreqNum < FreqNumLimit).as_bool() {
            ed += 1;
            NowFreqNum += huffman.mtfFreq[ed];
        }
        if (ed > st).as_bool() && (npart != nGroups).as_bool() && (npart != 1).as_bool() && (((nGroups - npart) & 1) != 0).as_bool() {
            NowFreqNum -= huffman.mtfFreq[ed];
            ed -= 1;
        }
        c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
            if (i >= st).as_bool() && (i <= ed).as_bool() {
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