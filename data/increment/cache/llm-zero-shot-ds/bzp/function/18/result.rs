use std::cmp::min;
use std::mem::size_of;
use std::ptr::write_bytes;

pub fn bzp_huffman_main(huffman: &mut BzpHuffmanGroups) {
    let n_groups = bzp_get_huffman_groups(huffman.n_block);
    huffman.n_groups = n_groups;

    bzp_init_len_array(huffman);
    let mut st = 0;
    let mut ed;

    for _ in 0..BZP_MAX_ITER_NUM {
        for j in 0..n_groups {
            unsafe {
                write_bytes(
                    huffman.huffman_groups[j].weight.as_mut_ptr(),
                    0,
                    huffman.huffman_groups[j].weight.len(),
                );
            }
        }

        st = 0;
        huffman.n_select = 0;
        while st < huffman.n_block {
            ed = min(huffman.n_block, st + BZP_ELEMS_NUM_IN_ONE_GROUP as i32) - 1;

            bzp_calculate_cost(huffman, st, ed);

            let id = bzp_select_tree(huffman);

            for k in st..=ed {
                huffman.huffman_groups[id].weight[huffman.block[k as usize] as usize] += 1;
            }
            st = ed + 1;
        }

        for j in 0..n_groups {
            bzp_build_tree_balance_height(&mut huffman.huffman_groups[j]);
        }
    }

    bzp_generate_select_mtf(huffman);

    for i in 0..n_groups {
        bzp_get_huffman_table(&mut huffman.huffman_groups[i]);
    }
}
