use std::ptr::write_bytes;

pub fn bzp_calculate_cost(huffman: &mut BzpHuffmanGroups, st: i32, ed: i32) {
    // Zero out the cost array
    unsafe {
        write_bytes(huffman.cost.as_mut_ptr(), 0, huffman.cost.len());
    }
    
    let n_groups = huffman.n_groups;
    for k in st..=ed {
        for t in 0..n_groups {
            let block_val = huffman.block[k as usize];
            huffman.cost[t as usize] += huffman.huffman_groups[t as usize].len[block_val as usize];
        }
    }
}
