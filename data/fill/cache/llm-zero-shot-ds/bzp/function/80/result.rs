use std::cmp;

pub struct BzpHuffmanGroup {
    pub len: Vec<i32>,
}

pub struct BzpHuffmanGroups {
    pub nGroups: i32,
    pub nBlock: i32,
    pub alphaSize: i32,
    pub mtfFreq: Vec<i32>,
    pub huffmanGroups: Vec<BzpHuffmanGroup>,
}

pub const BZP_HUFFMAN_LEN_MAX_COST: i32 = 15;

pub fn bzp_init_len_array(huffman: &mut BzpHuffmanGroups) {
    let n_groups = huffman.nGroups;
    let mut npart = n_groups;
    let mut all_freq_num = huffman.nBlock;
    let mut st = 0;
    let mut ed;

    while npart > 0 {
        let mut now_freq_num = 0;
        let freq_num_limit = all_freq_num / npart;
        ed = st - 1;

        while ed < huffman.alphaSize - 1 && now_freq_num < freq_num_limit {
            ed += 1;
            now_freq_num += huffman.mtfFreq[ed as usize];
        }

        if ed > st && npart != n_groups && npart != 1 && ((n_groups - npart) & 1) != 0 {
            now_freq_num -= huffman.mtfFreq[ed as usize];
            ed -= 1;
        }

        for i in 0..huffman.alphaSize {
            let idx = i as usize;
            if i >= st && i <= ed {
                huffman.huffmanGroups[(npart - 1) as usize].len[idx] = 0;
            } else {
                huffman.huffmanGroups[(npart - 1) as usize].len[idx] = BZP_HUFFMAN_LEN_MAX_COST;
            }
        }

        npart -= 1;
        st = ed + 1;
        all_freq_num -= now_freq_num;
    }
}
