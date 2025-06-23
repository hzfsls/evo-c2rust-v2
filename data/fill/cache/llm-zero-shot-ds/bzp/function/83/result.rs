use std::cmp::min;

pub fn bzp_huffman_main(huffman: &mut BzpHuffmanGroups) {
    let n_groups = bzp_get_huffman_groups(huffman.n_block);
    huffman.n_groups = n_groups;
    bzp_init_len_array(huffman);
    
    let mut st;
    for _ in 0..BZP_MAX_ITER_NUM {
        for j in 0..n_groups {
            huffman.huffman_groups[j].weight.iter_mut().for_each(|w| *w = 0);
        }
        st = 0;
        huffman.n_select = 0;
        while st < huffman.n_block {
            let ed = min(huffman.n_block, st + BZP_ELEMS_NUM_IN_ONE_GROUP as i32) - 1;
            bzp_calculate_cost(huffman, st, ed);
            let id = bzp_select_tree(huffman);
            for k in st..=ed {
                let block_val = huffman.block[k as usize];
                huffman.huffman_groups[id as usize].weight[block_val as usize] += 1;
            }
            st = ed + 1;
        }
        for j in 0..n_groups {
            bzp_build_tree_balance_height(&mut huffman.huffman_groups[j as usize]);
        }
    }
    bzp_generate_select_mtf(huffman);
    for i in 0..n_groups {
        bzp_get_huffman_table(&mut huffman.huffman_groups[i as usize]);
    }
}
