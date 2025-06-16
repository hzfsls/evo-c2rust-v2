pub fn BzpAddCharToBlock(mut lasch: u8, mut num: i32, mut bwt: Ptr<BzpBwtInfo>) {
    if (num < BZP_RLC_NUM_LOWER_LIMIT!()).as_bool() || (num > BZP_RLC_NUM_UPPER_LIMIT!()).as_bool() {
        return;
    }
    c_for!(let mut i: i32 = 0; i < num; i.suffix_plus_plus(); {
        BZP_UPDATE_CRC!(bwt.blockCRC, lasch);
    });

    let mut val: i32 = BZP_MIN_FUN!(num, BZP_RLC_NUM_4!().cast()).cast();
    c_switch!(val, {
        BZP_RLC_NUM_4!() => {
            bwt.block[bwt.nBlock.suffix_plus_plus()] = lasch.cast();
            break;
        },
        BZP_RLC_NUM_3!() => {
            bwt.block[bwt.nBlock.suffix_plus_plus()] = lasch.cast();
            break;
        },
        BZP_RLC_NUM_2!() => {
            bwt.block[bwt.nBlock.suffix_plus_plus()] = lasch.cast();
            break;
        },
        BZP_RLC_NUM_1!() => {
            bwt.block[bwt.nBlock.suffix_plus_plus()] = lasch.cast();
            break;
        },
        _ => {
            break;
        },
    });
    if (num >= BZP_RLC_NUM_4!()).as_bool() {
        bwt.block[bwt.nBlock.suffix_plus_plus()] = (num - BZP_RLC_NUM_4!()).cast::<u8>();
        bwt.inUse[num - BZP_RLC_NUM_4!()] = true;
    }

    bwt.inUse[lasch] = true;
}
