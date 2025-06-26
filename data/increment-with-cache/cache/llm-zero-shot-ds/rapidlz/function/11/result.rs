use std::ptr;

pub struct RapidlzStreamCtx {
    current_offset: u32,
    dict: *const u8,
    dict_size: u32,
    hash_table: *mut u32,
    acceleration: i32,
}

pub unsafe fn rapidlz_comp_with_external_dict(
    strm_ctx: *mut RapidlzStreamCtx,
    src: *const u8,
    dest: *mut u8,
    src_size: i32,
    dest_size: i32,
) -> i32 {
    let cur_src = src;
    let cur_src_anchor = cur_src;
    let src_end = cur_src.offset(src_size as isize);
    let cur_dest = dest;
    let dest_end = cur_dest.offset(dest_size as isize);

    if src_size < RAPIDLZ_LAST_LITERAL_LENGTH {
        return rapidlz_enc_last_literals(cur_src_anchor, src_end, cur_dest, dest_end, dest);
    }

    let match_start_limit = src_end.offset(-(RAPIDLZ_MIN_COMPRESSED_SIZE as isize) + 1);
    let match_end_limit = src_end.offset(-(RAPIDLZ_LAST_LITERALS as isize));
    let start_index = (*strm_ctx).current_offset;
    let base = src.offset(-(start_index as isize));

    let dict = (*strm_ctx).dict;
    let dict_size = (*strm_ctx).dict_size;
    let dict_base = dict.offset(dict_size as isize - (*strm_ctx).current_offset as isize);
    let mut prefix_dict_start = ptr::null();
    let dict_end = dict.offset(dict_size as isize);
    (*strm_ctx).dict_size += src_size as u32;

    let prefix_dict_limit = start_index - dict_size;
    (*strm_ctx).current_offset += src_size as u32;

    let mut hash_value = rapidlz_hash4_calc_value(cur_src);
    rapidlz_hash4_put_pos(start_index, hash_value, (*strm_ctx).hash_table);
    let mut cur_src = cur_src.offset(1);
    let mut forward_hash_value = rapidlz_hash4_calc_value(cur_src);

    let mut match_ptr;
    let mut token;
    let acceleration = (*strm_ctx).acceleration;

    loop {
        let mut forward_pos = cur_src;
        let mut jump_step = 1;
        let mut search_match_nb = acceleration << RAPIDLZ_STEP_FORWARD_BASE;
        loop {
            hash_value = forward_hash_value;
            let current = (forward_pos as isize - base as isize) as u32;
            let match_offset = rapidlz_hash4_get_pos(hash_value, (*strm_ctx).hash_table);
            cur_src = forward_pos;
            forward_pos = forward_pos.offset(jump_step as isize);
            jump_step = search_match_nb >> RAPIDLZ_STEP_FORWARD_BASE;
            search_match_nb += 1;

            if unlikely!(forward_pos > match_start_limit) {
                return rapidlz_enc_last_literals(cur_src_anchor, src_end, cur_dest, dest_end, dest);
            }

            if match_offset < start_index {
                match_ptr = dict_base.offset(match_offset as isize);
                prefix_dict_start = dict;
            } else {
                match_ptr = base.offset(match_offset as isize);
                prefix_dict_start = src;
            }

            forward_hash_value = rapidlz_hash4_calc_value(forward_pos);
            rapidlz_hash4_put_pos(current, hash_value, (*strm_ctx).hash_table);
            if !rapidlz_continue_if_not_a_match(match_offset, prefix_dict_limit, current) {
                continue;
            }
            if ptr::read_unaligned(cur_src as *const u32) == ptr::read_unaligned(match_ptr as *const u32) {
                offset = current - match_offset;
                break;
            }
        }

        rapidlz_expand_forward(prefix_dict_start, match_ptr, cur_src, cur_src_anchor);

        token = cur_dest;
        if !rapidlz_stream_enc_literals(cur_src, cur_src_anchor, &mut cur_dest, dest_end) {
            return RAPIDLZ_ENC_NOT_OK;
        }

        // _OFFSET_AND_MATCH:
        ptr::write_unaligned(cur_dest as *mut u16, offset.to_le());
        cur_dest = cur_dest.offset(2);

        let match_len;
        let cur_src_match_end;

        if prefix_dict_start == dict {
            let src_limit_only_with_dict = cur_src.offset((dict_end as isize - match_ptr as isize));
            let adjusted_limit = if src_limit_only_with_dict > match_end_limit {
                match_end_limit
            } else {
                src_limit_only_with_dict
            };
            cur_src_match_end = rapidlz_compress_expand_backward(
                adjusted_limit,
                match_ptr.offset(RAPIDLZ_MIN_MATCH as isize),
                cur_src.offset(RAPIDLZ_MIN_MATCH as isize),
            );
            match_len = cur_src_match_end as isize - cur_src as isize - RAPIDLZ_MIN_MATCH as isize;
            cur_src = cur_src_match_end;

            if cur_src == adjusted_limit {
                cur_src_match_end = rapidlz_compress_expand_backward(
                    match_end_limit,
                    src,
                    adjusted_limit,
                );
                match_len += (cur_src_match_end as isize - cur_src as isize);
                cur_src = cur_src_match_end;
            }
        } else {
            cur_src_match_end = rapidlz_compress_expand_backward(
                match_end_limit,
                match_ptr.offset(RAPIDLZ_MIN_MATCH as isize),
                cur_src.offset(RAPIDLZ_MIN_MATCH as isize),
            );
            match_len = cur_src_match_end as isize - cur_src as isize - RAPIDLZ_MIN_MATCH as isize;
            cur_src = cur_src_match_end;
        }

        #[cfg(RAPIDLZ_DEBUG)]
        {
            if unlikely!(rapidlz_lit_and_match_copy_end(cur_dest, match_len as u32) > dest_end) {
                return RAPIDLZ_ENC_NOT_OK;
            }
        }

        cur_dest = cur_dest.offset(rapidlz_store_match_len(match_len as u32, token, cur_dest) as isize);
        cur_src_anchor = cur_src;
        if cur_src >= match_start_limit {
            break;
        }

        let hv2 = rapidlz_hash4_calc_value(cur_src.offset(-2));
        let index = (cur_src.offset(-2) as isize - base as isize) as u32;
        rapidlz_hash4_put_pos(index, hv2, (*strm_ctx).hash_table);

        hash_value = rapidlz_hash4_calc_value(cur_src);
        let current = (cur_src as isize - base as isize) as u32;
        let match_offset = rapidlz_hash4_get_pos(hash_value, (*strm_ctx).hash_table);
        if match_offset < start_index {
            match_ptr = dict_base.offset(match_offset as isize);
            prefix_dict_start = dict;
        } else {
            match_ptr = base.offset(match_offset as isize);
            prefix_dict_start = src;
        }

        rapidlz_hash4_put_pos(current, hash_value, (*strm_ctx).hash_table);

        if (match_offset >= prefix_dict_limit) && (match_offset + RAPIDLZ_MAX_OFFSET >= current) {
            if ptr::read_unaligned(cur_src as *const u32) == ptr::read_unaligned(match_ptr as *const u32) {
                token = cur_dest;
                *token = 0;
                cur_dest = cur_dest.offset(1);
                offset = current - match_offset;
                continue; // goto _OFFSET_AND_MATCH
            }
        }
        forward_hash_value = rapidlz_hash4_calc_value(cur_src.offset(1));
        cur_src = cur_src.offset(1);
    }

    rapidlz_enc_last_literals(cur_src_anchor, src_end, cur_dest, dest_end, dest)
}
