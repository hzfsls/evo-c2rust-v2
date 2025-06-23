use std::cmp::min;

const CMPT_EMPTY_HASH_VALUE: u32 = 0; // Assuming this is the correct value

fn cmpt_bt_skip(
    mf: &mut CmptMfCtx,
    len_limit: u32,
    pos: u32,
    cur: &[u8],
    cur_match: u32,
) {
    let mut depth = mf.depth;
    let son = &mut mf.son;
    let cycle_pos = mf.cycle_pos;
    let cycle_size = mf.cycle_size;
    let ptr0 = &mut son[(cycle_pos << 1) + 1];
    let ptr1 = &mut son[(cycle_pos << 1)];
    let mut len0 = 0;
    let mut len1 = 0;

    loop {
        let delta = pos - cur_match;
        if depth == 0 || delta >= cycle_size {
            *ptr0 = CMPT_EMPTY_HASH_VALUE;
            *ptr1 = CMPT_EMPTY_HASH_VALUE;
            return;
        }
        depth -= 1;

        let pair_index = (cycle_pos - delta + if delta > cycle_pos { cycle_size } else { 0 }) << 1;
        let pair = &son[pair_index..pair_index + 2];
        let pb = &cur[delta as usize..];
        let len = min(len0, len1);
        
        if pb[len as usize] == cur[len as usize] {
            let new_len = cmpt_mem_cmp_len_safe(pb, cur, len + 1, len_limit);
            if new_len == len_limit {
                *ptr1 = pair[0];
                *ptr0 = pair[1];
                return;
            }
        }

        if pb[len as usize] < cur[len as usize] {
            // CMPT_MF_LEFT_SON_UPDATE equivalent
            *ptr1 = cur_match;
            len1 = len;
        } else {
            // CMPT_MF_RIGHT_SON_UPDATE equivalent
            *ptr0 = cur_match;
            len0 = len;
        }
    }
}
