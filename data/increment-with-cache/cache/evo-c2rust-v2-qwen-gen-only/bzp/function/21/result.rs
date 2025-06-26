pub fn BzpAddCharToBlock(mut lasch: u8, mut num: i32, mut bwt: Ptr<BzpBwtInfo>) {
    if (num < BZP_RLC_NUM_LOWER_LIMIT!().cast() || num > BZP_RLC_NUM_UPPER_LIMIT!().cast()).as_bool() {
        return;
    }
    c_for!(let mut i: i32 = 0; i < num.cast(); i.suffix_plus_plus(); {
        BZP_UPDATE_CRC!(bwt.blockCRC, lasch);
    });
    let mut val: i32 = BZP_MIN_FUN!(num, BZP_RLC_NUM_4!().cast());
    if val == BZP_RLC_NUM_4!().cast() {
        bwt.block[bwt.nBlock.suffix_plus_plus()] = lasch.cast();
    }
    if val == BZP_RLC_NUM_3!().cast() {
        bwt.block[bwt.nBlock.suffix_plus_plus()] = lasch.cast();
    }
    if val == BZP_RLC_NUM_2!().cast() {
        bwt.block[bwt.nBlock.suffix_plus_plus()] = lasch.cast();
    }
    if val == BZP_RLC_NUM_1!().cast() {
        bwt.block[bwt.nBlock.suffix_plus_plus()] = lasch.cast();
    }
    if (num >= BZP_RLC_NUM_4!().cast()).as_bool() {
        bwt.block[bwt.nBlock.suffix_plus_plus()] = (num - BZP_RLC_NUM_4!().cast()).cast();
        bwt.inUse[(num - BZP_RLC_NUM_4!().cast()).cast()] = true.cast();
    }
    bwt.inUse[lasch.cast()] = true.cast();
}