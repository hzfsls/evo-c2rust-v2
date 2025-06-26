use std::cmp::min;

const CMPT_EMPTY_HASH_VALUE: u32 = 0xFFFFFFFF;

#[derive(Debug)]
struct CmptlzMatchPair {
    len: u32,
    dist: u32,
}

struct CmptMfCtx {
    depth: u32,
    son: Vec<u32>,
    src_start: *const u8,
    read_pos: u32,
    nice_len: u32,
    cycle_pos: u32,
    cycle_size: u32,
    offset: u32,
}

unsafe fn cmpt_bt_find(
    mf: &mut CmptMfCtx,
    cur_match: u32,
    matches: &mut [CmptlzMatchPair],
    mut longest_len: u32,
) -> &mut [CmptlzMatchPair] {
    let mut depth = mf.depth;
    let son = &mut mf.son;
    let cur = mf.src_start.add(mf.read_pos as usize);
    let nice_len = mf.nice_len;
    let cycle_pos = mf.cycle_pos;
    let cycle_size = mf.cycle_size;
    let pos = mf.read_pos + mf.offset;
    let ptr0 = &mut son[(cycle_pos as usize * 2) + 1] as *mut u32;
    let ptr1 = &mut son[cycle_pos as usize * 2] as *mut u32;
    let mut len0 = 0;
    let mut len1 = 0;
    let mut matches_ptr = matches.as_mut_ptr();
    let mut matches_len = matches.len();

    loop {
        let delta = pos - cur_match;
        if depth == 0 || delta >= cycle_size {
            *ptr0 = CMPT_EMPTY_HASH_VALUE;
            *ptr1 = CMPT_EMPTY_HASH_VALUE;
            return std::slice::from_raw_parts_mut(matches_ptr, matches_len);
        }
        depth -= 1;

        let pair_idx = (cycle_pos - delta + if delta > cycle_pos { cycle_size } else { 0 }) as usize * 2;
        let pair = &mut son[pair_idx..pair_idx + 2];
        let pb = cur.sub(delta as usize);

        let len = min(len0, len1);
        if *pb.add(len as usize) == *cur.add(len as usize) {
            let len = cmpt_mem_cmp_len_safe(pb, cur, len + 1, nice_len);
            if longest_len < len {
                longest_len = len;
                (*matches_ptr).len = len;
                (*matches_ptr).dist = delta - 1;
                matches_ptr = matches_ptr.add(1);
                matches_len -= 1;
                if len == nice_len {
                    *ptr1 = pair[0];
                    *ptr0 = pair[1];
                    return std::slice::from_raw_parts_mut(matches_ptr, matches_len);
                }
            }
        }

        if *pb.add(len as usize) < *cur.add(len as usize) {
            // CMPT_MF_LEFT_SON_UPDATE(ptr1, pair, cur_match, len1, len)
            *ptr1 = cur_match;
            len1 = len;
            ptr1 = &mut pair[0] as *mut u32;
        } else {
            // CMPT_MF_RIGHT_SON_UPDATE(ptr0, pair, cur_match, len0, len)
            *ptr0 = cur_match;
            len0 = len;
            ptr0 = &mut pair[1] as *mut u32;
        }
    }
}

unsafe fn cmpt_mem_cmp_len_safe(a: *const u8, b: *const u8, mut len: u32, max_len: u32) -> u32 {
    let mut a_ptr = a;
    let mut b_ptr = b;
    let end_len = len + (max_len - len).min(8); // Compare up to 8 bytes at a time
    
    while len < end_len && *a_ptr == *b_ptr {
        len += 1;
        a_ptr = a_ptr.add(1);
        b_ptr = b_ptr.add(1);
    }
    
    len
}
