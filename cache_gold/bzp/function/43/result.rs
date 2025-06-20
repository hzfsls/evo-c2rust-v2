pub fn BzpBuffToBlockRLC(mut bzpf: Ptr<BzpFile>, mut bwt: Ptr<BzpBwtInfo>, mut IsLastdata: bool) {
    while !BZP_BLOCK_FULL!(bwt) && !BZP_BUFF_READ_EMPTY!(bzpf) {
        let mut pos: i32 = bzpf.input.pos;
        let mut ch: u8 = bzpf.input.buf[pos].cast();
        let mut lasch: u8 = bzpf.lasChar.cast();
        if ch != lasch || bzpf.num == BZP_RLC_NUM_UPPER_LIMIT!() {
            BzpAddCharToBlock(lasch, bzpf.num, bwt);
            bzpf.lasChar = ch.cast();
            bzpf.num = 1;
        } else {
            bzpf.num += 1;
        }
        bzpf.input.pos += 1;
    }
    if IsLastdata && BZP_BUFF_READ_EMPTY!(bzpf) {
        BzpAddCharToBlock(bzpf.lasChar.cast(), bzpf.num, bwt);
        bzpf.lasChar = BZP_ASCII_SIZE!();
        bzpf.num = 0;
    }
}