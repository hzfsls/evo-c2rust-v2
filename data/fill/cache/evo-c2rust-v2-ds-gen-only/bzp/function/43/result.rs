pub fn BzpBuffToBlockRLC(mut bzpf: Ptr<BzpFile>, mut bwt: Ptr<BzpBwtInfo>, mut IsLastdata: bool) {
    while (!BZP_BLOCK_FULL!(bwt).as_bool() && (!BZP_BUFF_READ_EMPTY!(bzpf)).as_bool() {
        let mut pos: i32 = bzpf.input.pos.cast();
        let mut ch: u8 = bzpf.input.buf[pos].cast::<u8>();
        let mut lasch: u8 = bzpf.lasChar.cast::<u8>();
        if (ch != lasch).as_bool() || (bzpf.num == BZP_RLC_NUM_UPPER_LIMIT!()).as_bool() {
            BzpAddCharToBlock(lasch.cast(), bzpf.num.cast(), bwt.cast());
            bzpf.lasChar = ch.cast();
            bzpf.num = 1;
        } else {
            bzpf.num += 1;
        }
        bzpf.input.pos += 1;
    }
    if IsLastdata.as_bool() && BZP_BUFF_READ_EMPTY!(bzpf).as_bool() {
        BzpAddCharToBlock(bzpf.lasChar.cast(), bzpf.num.cast(), bwt.cast());
        bzpf.lasChar = BZP_ASCII_SIZE!();
        bzpf.num = 0;
    }
}
