pub fn cmptlz_match_skiper(mf: &mut CmptMfCtx, amount: u32) {
    mf.read_ahead += amount;
    let nice_len = mf.nice_len;
    let mut remaining_amount = amount;
    
    while remaining_amount > 0 {
        remaining_amount -= 1;
        
        let len_limit = mf.src_len - mf.read_pos;
        let len_limit = if nice_len <= len_limit {
            nice_len
        } else {
            mf.read_pos += 1;
            continue;
        };
        
        let cur = &mf.src_start[mf.read_pos..];
        let pos = mf.read_pos + mf.offset;
        
        let (temp, hash2_value, hash3_value, hash_value) = cmpt_hash_4_calc(mf, cur);
        let cur_match = mf.hash[CMPTLZ_FIX_4_HASH + hash_value];
        
        cmpt_hash_update(mf, hash2_value, hash3_value, hash_value, pos);
        cmpt_bt_skip(mf, len_limit, pos, cur, cur_match);
        cmpt_mf_move_pos(mf);
    }
}
