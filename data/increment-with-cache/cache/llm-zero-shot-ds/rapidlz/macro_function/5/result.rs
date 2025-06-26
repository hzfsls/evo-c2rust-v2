macro_rules! rapidlz_fast_safe_copy_by_bytes {
    ($curDest:expr, $matchSrc:expr, $len:expr) => {
        {
            let mut cur_dest = $curDest;
            let mut match_src = $matchSrc;
            let mut len = $len;

            while len > 2 {
                *cur_dest = *match_src;
                cur_dest = cur_dest.wrapping_add(1);
                match_src = match_src.wrapping_add(1);
                *cur_dest = *match_src;
                cur_dest = cur_dest.wrapping_add(1);
                match_src = match_src.wrapping_add(1);
                *cur_dest = *match_src;
                cur_dest = cur_dest.wrapping_add(1);
                match_src = match_src.wrapping_add(1);
                len -= 3;
            }
            if len > 0 {
                *cur_dest = *match_src;
                cur_dest = cur_dest.wrapping_add(1);
                match_src = match_src.wrapping_add(1);
                if len > 1 {
                    *cur_dest = *match_src;
                    cur_dest = cur_dest.wrapping_add(1);
                    match_src = match_src.wrapping_add(1);
                }
            }
        }
    };
}

pub(crate) use rapidlz_fast_safe_copy_by_bytes;
