pub fn bzp_qsort_single(sort_block: &mut [i32], idx: &[i32], stack: &mut BzpQSortInfo) {
    let tl = stack.tl;
    let tr = stack.tr;
    let value = bzp_select_mid_val(sort_block, idx, tl, tr);
    let mut l_pos = tl;
    let mut r_pos = tr;
    let mut e_pos = tl;
    
    while e_pos <= r_pos {
        if idx[sort_block[e_pos as usize] as i32 < value {
            bzp_swap_2_elem(sort_block, e_pos, l_pos);
            e_pos += 1;
            l_pos += 1;
        } else if idx[sort_block[e_pos as usize] as i32 == value {
            e_pos += 1;
        } else {
            while r_pos >= e_pos && idx[sort_block[r_pos as usize] as i32 > value {
                r_pos -= 1;
            }
            if r_pos < e_pos {
                break;
            }
            if idx[sort_block[r_pos as usize] as i32] == value {
                bzp_swap_2_elem(sort_block, e_pos, r_pos);
            } else if l_pos == e_pos {
                bzp_swap_2_elem(sort_block, e_pos, r_pos);
                l_pos += 1;
            } else {
                bzp_swap_3_elem(sort_block, l_pos, e_pos, r_pos);
                l_pos += 1;
            }
            e_pos += 1;
            r_pos -= 1;
        }
    }
    
    if l_pos - tl > tr - r_pos {
        stack.stack_l[stack.cnt as usize] = tl;
        stack.stack_r[stack.cnt as usize] = l_pos - 1;
        stack.cnt += 1;
        stack.stack_l[stack.cnt as usize] = r_pos + 1;
        stack.stack_r[stack.cnt as usize] = tr;
        stack.cnt += 1;
    } else {
        stack.stack_l[stack.cnt as usize] = r_pos + 1;
        stack.stack_r[stack.cnt as usize] = tr;
        stack.cnt += 1;
        stack.stack_l[stack.cnt as usize] = tl;
        stack.stack_r[stack.cnt as usize] = l_pos - 1;
        stack.cnt += 1;
    }
}
