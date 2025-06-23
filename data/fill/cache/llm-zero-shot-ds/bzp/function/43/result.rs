use std::os::raw::{c_int, c_uint8};

// Assuming the following types are defined elsewhere in the Rust code:
// - BzpFile
// - BzpBwtInfo
// - BZP_BLOCK_FULL(bwt: &BzpBwtInfo) -> bool
// - BZP_BUFF_READ_EMPTY(bzpf: &BzpFile) -> bool
// - BzpAddCharToBlock(ch: u8, num: i32, bwt: &mut BzpBwtInfo)
// - BZP_RLC_NUM_UPPER_LIMIT: i32
// - BZP_ASCII_SIZE: u8

pub fn bzp_buff_to_block_rlc(bzpf: &mut BzpFile, bwt: &mut BzpBwtInfo, is_last_data: bool) {
    while !BZP_BLOCK_FULL(bwt) && !BZP_BUFF_READ_EMPTY(bzpf) {
        let pos = bzpf.input.pos;
        let ch = bzpf.input.buf[pos] as u8;
        let last_ch = bzpf.las_char as u8;
        
        if ch != last_ch || bzpf.num == BZP_RLC_NUM_UPPER_LIMIT {
            BzpAddCharToBlock(last_ch, bzpf.num, bwt);
            bzpf.las_char = ch as c_uint8;
            bzpf.num = 1;
        } else {
            bzpf.num += 1;
        }
        bzpf.input.pos += 1;
    }
    
    if is_last_data && BZP_BUFF_READ_EMPTY(bzpf) {
        BzpAddCharToBlock(bzpf.las_char as u8, bzpf.num, bwt);
        bzpf.las_char = BZP_ASCII_SIZE as c_uint8;
        bzpf.num = 0;
    }
}
