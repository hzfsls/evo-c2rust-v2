pub fn bzp_add_char_to_block(lasch: u8, num: i32, bwt: &mut BzpBwtInfo) {
    if num < BZP_RLC_NUM_LOWER_LIMIT || num > BZP_RLC_NUM_UPPER_LIMIT {
        return;
    }
    
    for _ in 0..num {
        BZP_UPDATE_CRC(bwt.blockCRC, lasch);
    }

    let val = BZP_MIN_FUN(num, BZP_RLC_NUM_4 as i32);
    match val {
        BZP_RLC_NUM_4 => {
            bwt.block[bwt.nBlock] = lasch;
            bwt.nBlock += 1;
        }
        BZP_RLC_NUM_3 => {
            bwt.block[bwt.nBlock] = lasch;
            bwt.nBlock += 1;
        }
        BZP_RLC_NUM_2 => {
            bwt.block[bwt.nBlock] = lasch;
            bwt.nBlock += 1;
        }
        BZP_RLC_NUM_1 => {
            bwt.block[bwt.nBlock] = lasch;
            bwt.nBlock += 1;
        }
        _ => {}
    }

    if num >= BZP_RLC_NUM_4 as i32 {
        bwt.block[bwt.nBlock] = (num - BZP_RLC_NUM_4 as i32) as u8;
        bwt.nBlock += 1;
        bwt.inUse[(num - BZP_RLC_NUM_4 as i32) as usize] = true;
    }

    bwt.inUse[lasch as usize] = true;
}
