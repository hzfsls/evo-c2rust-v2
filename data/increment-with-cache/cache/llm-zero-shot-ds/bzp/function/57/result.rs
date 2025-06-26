pub fn bzp_de_code_to_stream(in_data: &mut InDeComdata, debwt: &BzpBwtDecodeInfo) -> i32 {
    let mut ret = BZP_OK;
    for i in 0..debwt.nBlock {
        let ch = debwt.deCode[i];
        if in_data.num == BZP_RLC_NUM_4 {
            for j in 0..(ch as i32) {
                bzp_update_crc(&mut in_data.blockCRC, in_data.lasChar as u8);
                ret |= bzp_write_char(in_data.lasChar, in_data);
            }
            in_data.lasChar = BZP_ASCII_SIZE;
            in_data.num = 0;
        } else if ch == in_data.lasChar as u8 {
            bzp_update_crc(&mut in_data.blockCRC, ch);
            ret = bzp_write_char(ch as i32, in_data);
            in_data.num += 1;
        } else {
            bzp_update_crc(&mut in_data.blockCRC, ch);
            ret = bzp_write_char(ch as i32, in_data);
            in_data.lasChar = ch as i32;
            in_data.num = 1;
        }
        if ret != BZP_OK {
            break;
        }
    }
    ret
}
