pub fn BzpInitLenArray(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = huffman.nGroups;
    let mut npart: i32 = nGroups;
    let mut AllFreqNum: i32 = huffman.nBlock;
    let mut st: i32 = 0;
    let mut ed: i32;
    while npart > 0 {
        let mut NowFreqNum: i32 = 0;
        let mut FreqNumLimit: i32 = AllFreqNum / npart;
        ed = st - 1;
        while ed < huffman.alphaSize - 1 && NowFreqNum < FreqNumLimit {
            ed += 1;
            NowFreqNum += huffman.mtfFreq[ed];
        }
        if ed > st && npart != nGroups && npart != 1 && ((nGroups - npart) & 1) != 0 {
            NowFreqNum -= huffman.mtfFreq[ed];
            ed -= 1;
        }
        c_for!(let mut i = 0; i < huffman.alphaSize; i += 1; {
            if i >= st && i <= ed {
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