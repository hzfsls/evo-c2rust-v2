use std::ptr;

pub fn rapidlz_comp_with_prefix_dict(
    strm_ctx: &mut RapidlzStreamCtx,
    src: &[u8],
    dest: &mut [u8],
    src_size: usize,
    dest_size: usize,
) -> i32 {
    let mut cur_src = src.as_ptr();
    let mut cur_src_anchor = cur_src;
    let src_end = unsafe { cur_src.add(src_size) };
    let mut cur_dest = dest.as_mut_ptr();
    let dest_end = unsafe { cur_dest.add(dest_size) };

    if src_size < RAPIDLZ_LAST_LITERAL_LENGTH {
        return rapidlz_enc_last_literals(
            cur_src_anchor,
            src_end,
            cur_dest,
            dest_end,
            dest.as_mut_ptr(),
        );
    }

    let match_start_limit = unsafe { src_end.sub(RAPIDLZ_MIN_COMPRESSED_SIZE - 1) };
    let match_end_limit = unsafe { src_end.sub(RAPIDLZ_LAST_LITERALS) };
    let start_index = strm_ctx.current_offset;
    let base = unsafe { src.as_ptr().sub(start_index as usize) };

    let dict_size = strm_ctx.dict_size;
    let prefix_dict_start = unsafe { src.as_ptr().sub(dict_size as usize) };
    strm_ctx.dict_size += src_size as u32;
    let prefix_dict_limit = start_index - dict_size;
    strm_ctx.current_offset += src_size as u32;

    let mut hash_value = rapidlz_hash4_calc_value(unsafe { &*cur_src });
    rapidlz_hash4_put_pos(start_index, hash_value, &mut strm_ctx.hash_table);
    cur_src = unsafe { cur_src.add(1) };
    let mut forward_hash_value = rapidlz_hash4_calc_value(unsafe { &*cur_src });

    let acceleration = strm_ctx.acceleration;
    loop {
        let mut forward_pos = cur_src;
        let mut step = 1;
        let mut search_match_nb = acceleration << RAPIDLZ_STEP_FORWARD_BASE;
        let mut match_ptr;
        loop {
            hash_value = forward_hash_value;
            let current = unsafe { forward_pos.offset_from(base) } as u32;
            let match_offset = rapidlz_hash4_get_pos(hash_value, &strm_ctx.hash_table);
            cur_src = forward_pos;
            forward_pos = unsafe { forward_pos.add(step) };
            step = search_match_nb >> RAPIDLZ_STEP_FORWARD_BASE;
            search_match_nb += 1;

            if unlikely!(forward_pos > match_start_limit) {
                return rapidlz_enc_last_literals(
                    cur_src_anchor,
                    src_end,
                    cur_dest,
                    dest_end,
                    dest.as_mut_ptr(),
                );
            }

            match_ptr = unsafe { base.add(match_offset as usize) };
            forward_hash_value = rapidlz_hash4_calc_value(unsafe { &*forward_pos });
            rapidlz_hash4_put_pos(current, hash_value, &mut strm_ctx.hash_table);

            if match_offset < prefix_dict_limit {
                continue;
            }
            if (match_offset + RAPIDLZ_MAX_OFFSET) < current {
                continue;
            }
            if unsafe { ptr::read_unaligned(cur_src as *const u32) }
                == unsafe { ptr::read_unaligned(match_ptr as *const u32) }
            {
                break;
            }
        }

        rapidlz_expand_forward(prefix_dict_start, match_ptr, cur_src, &mut cur_src_anchor);

        let token = cur_dest;
        if rapidlz_stream_enc_literals(
            cur_src,
            cur_src_anchor,
            &mut cur_dest,
            dest_end,
        ) != RAPIDLZ_ENC_OK {
            return RAPIDLZ_ENC_NOT_OK;
        }

        // _OFFSET_AND_MATCH:
        unsafe {
            ptr::write_unaligned(cur_dest as *mut u16, (cur_src as usize - match_ptr as usize) as u16);
            cur_dest = cur_dest.add(2);
        }

        let cur_src_match_end = rapidlz_compress_expand_backward(
            match_end_limit,
            unsafe { match_ptr.add(RAPIDLZ_MIN_MATCH) },
            unsafe { cur_src.add(RAPIDLZ_MIN_MATCH) },
        );
        let match_len = unsafe { cur_src_match_end.offset_from(cur_src) } as u32 - RAPIDLZ_MIN_MATCH as u32;
        cur_src = cur_src_match_end;

        #[cfg(debug_assertions)]
        if unlikely!(
            unsafe { cur_dest.add(rapidlz_store_match_len(match_len, token, cur_dest)) }
                > dest_end
        ) {
            return RAPIDLZ_ENC_NOT_OK;
        }

        cur_dest = unsafe { cur_dest.add(rapidlz_store_match_len(match_len, token, cur_dest)) };

        cur_src_anchor = cur_src;
        if cur_src >= match_start_limit {
            break;
        }

        let hv = rapidlz_hash4_calc_value(unsafe { &*cur_src.sub(2) });
        let index = unsafe { cur_src.sub(2).offset_from(base) } as u32;
        rapidlz_hash4_put_pos(index, hv, &mut strm_ctx.hash_table);

        hash_value = rapidlz_hash4_calc_value(unsafe { &*cur_src });
        let current = unsafe { cur_src.offset_from(base) } as u32;
        let match_offset = rapidlz_hash4_get_pos(hash_value, &strm_ctx.hash_table);

        match_ptr = unsafe { base.add(match_offset as usize) };

        rapidlz_hash4_put_pos(current, hash_value, &mut strm_ctx.hash_table);
        if (match_offset >= prefix_dict_limit) && ((match_offset + RAPIDLZ_MAX_OFFSET) >= current) {
            if unsafe { ptr::read_unaligned(cur_src as *const u32) }
                == unsafe { ptr::read_unaligned(match_ptr as *const u32) }
            {
                let token = cur_dest;
                unsafe {
                    *cur_dest = 0;
                    cur_dest = cur_dest.add(1);
                }
                // goto _OFFSET_AND_MATCH
                unsafe {
                    ptr::write_unaligned(cur_dest as *mut u16, (cur_src as usize - match_ptr as usize) as u16);
                    cur_dest = cur_dest.add(2);
                }

                let cur_src_match_end = rapidlz_compress_expand_backward(
                    match_end_limit,
                    unsafe { match_ptr.add(RAPIDLZ_MIN_MATCH) },
                    unsafe { cur_src.add(RAPIDLZ_MIN_MATCH) },
                );
                let match_len = unsafe { cur_src_match_end.offset_from(cur_src) } as u32 - RAPIDLZ_MIN_MATCH as u32;
                cur_src = cur_src_match_end;

                #[cfg(debug_assertions)]
                if unlikely!(
                    unsafe { cur_dest.add(rapidlz_store_match_len(match_len, token, cur_dest)) }
                        > dest_end
                ) {
                    return RAPIDLZ_ENC_NOT_OK;
                }

                cur_dest = unsafe { cur_dest.add(rapidlz_store_match_len(match_len, token, cur_dest)) };

                cur_src_anchor = cur_src;
                if cur_src >= match_start_limit {
                    break;
                }
            }
        }
        forward_hash_value = rapidlz_hash4_calc_value(unsafe { &*cur_src.add(1) });
        cur_src = unsafe { cur_src.add(1) };
    }

    rapidlz_enc_last_literals(
        cur_src_anchor,
        src_end,
        cur_dest,
        dest_end,
        dest.as_mut_ptr(),
    )
}
