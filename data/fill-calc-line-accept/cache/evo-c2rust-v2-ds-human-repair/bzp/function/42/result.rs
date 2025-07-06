pub fn BzpAddCharToBlock(mut lasch: u8, mut num: i32, mut bwt: Ptr<BzpBwtInfo>) {
    if (num < BZP_RLC_NUM_LOWER_LIMIT!()) || (num > BZP_RLC_NUM_UPPER_LIMIT!()) {
        return;
    }
    c_for!(let mut i: i32 = 0; i < num; i.suffix_plus_plus(); {
        BZP_UPDATE_CRC!(bwt.blockCRC, lasch);
    });
    let mut val: i32 = BZP_MIN_FUN!(num, BZP_RLC_NUM_4!());
    c_switch!(val;
        BZP_RLC_NUM_4!() => {
            let tmp0 = bwt.nBlock.suffix_plus_plus();
            bwt.block[tmp0] = lasch;
        },
        BZP_RLC_NUM_3!() => {
            let tmp0 = bwt.nBlock.suffix_plus_plus();
            bwt.block[tmp0] = lasch;
        },
        BZP_RLC_NUM_2!() => {
            let tmp0 = bwt.nBlock.suffix_plus_plus();
            bwt.block[tmp0] = lasch;
        },
        BZP_RLC_NUM_1!() => {
            let tmp0 = bwt.nBlock.suffix_plus_plus();
            bwt.block[tmp0] = lasch;
        },
    );
    if (num >= BZP_RLC_NUM_4!()) {
        let tmp0 = bwt.nBlock.suffix_plus_plus();
        bwt.block[tmp0] = (num - BZP_RLC_NUM_4!()).cast();
        bwt.inUse[num - BZP_RLC_NUM_4!()] = true;
    }
    bwt.inUse[lasch] = true;
}
