pub fn bzp_buff_to_block_rlc(bzpf: &mut BzpFile, bwt: &mut BzpBwtInfo, is_last_data: bool) {
    while !bzp_block_full(bwt) && !bzp_buff_read_empty(bzpf) {
        let pos = bzpf.input.pos;
        let ch = bzpf.input.buf[pos] as u8;
        let lasch = bzpf.las_char as u8;
        
        if ch != lasch || bzpf.num == BZP_RLC_NUM_UPPER_LIMIT {
            bzp_add_char_to_block(lasch, bzpf.num, bwt);
            bzpf.las_char = ch as i32; // Assuming las_char is i32 in Rust
            bzpf.num = 1;
        } else {
            bzpf.num += 1;
        }

        bzpf.input.pos += 1;
    }

    if is_last_data && bzp_buff_read_empty(bzpf) {
        bzp_add_char_to_block(bzpf.las_char as u8, bzpf.num, bwt);
        bzpf.las_char = BZP_ASCII_SIZE as i32; // Assuming BZP_ASCII_SIZE is u8/u32 and las_char is i32
        bzpf.num = 0;
    }
}
