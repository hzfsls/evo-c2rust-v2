pub fn BzpAddCharToBlock(mut lasch: u8, mut num: i32, mut bwt: Ptr<BzpBwtInfo>) {
    if num < BZP_RLC_NUM_LOWER_LIMIT!() || num > BZP_RLC_NUM_UPPER_LIMIT!() {
        return;
    }
    c_for!(let mut i = 0; i < num; i += 1; {
        BZP_UPDATE_CRC!(bwt.blockCRC, lasch);
    });
    let val = BZP_MIN_FUN!(num, BZP_RLC_NUM_4!());
    c_switch!(val;
        BZP_RLC_NUM_4!() => {
            index!(bwt.block, bwt.nBlock, lasch);
            bwt.nBlock += 1;
        },
        BZP_RLC_NUM_3!() => {
            // bwt.block[bwt.nBlock] = lasch;
            index!(bwt.block, bwt.nBlock, lasch);
            bwt.nBlock += 1;
        },
        BZP_RLC_NUM_2!() => {
            index!(bwt.block, bwt.nBlock, lasch);
            bwt.nBlock += 1;
        },
        BZP_RLC_NUM_1!() => {
            index!(bwt.block, bwt.nBlock, lasch);
            bwt.nBlock += 1;
        },
    );
    if num >= BZP_RLC_NUM_4!() {
        index!(bwt.block, bwt.nBlock, (num - BZP_RLC_NUM_4!()).cast());
        bwt.nBlock += 1;
        bwt.inUse[num - BZP_RLC_NUM_4!()] = true;
    }
    bwt.inUse[lasch as usize] = true;
}