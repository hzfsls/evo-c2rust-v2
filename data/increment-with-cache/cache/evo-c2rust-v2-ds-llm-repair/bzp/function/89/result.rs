pub fn BzpProcessData(mut bzpInfo: Ptr<BzpAlgorithmInfo>, mut IsLastdata: bool) -> i32 {
    let mut bzpf: Ptr<BzpFile> = bzpInfo.compressFile.cast();
    let mut outData: Ptr<BzpOutComdata> = bzpInfo.outData.cast();
    let mut bwt: Ptr<BzpBwtInfo> = bzpInfo.bwt.cast();

    bzpf.state = BZP_INPUT_COMPRESS!();
    let mut ret: i32 = BZP_OK!();
    while (bzpf.state != BZP_RETUEN_COMPRESS!()).as_bool() {
        if (bzpf.state == BZP_OUTPUT_COMPRESS!()).as_bool() {
            ret = BzpBuffToStream(bzpf.cast(), outData.cast()).cast();

            BzpResetCompress(bwt.cast(), outData.cast());
            bzpf.state = BZP_INPUT_COMPRESS!();
            if IsLastdata.as_bool() && BZP_BUFF_READ_EMPTY!(bzpf).as_bool() {
                bzpf.state = BZP_RETUEN_COMPRESS!();
            }
        }
        if (bzpf.state == BZP_INPUT_COMPRESS!()).as_bool() {
            BzpBuffToBlockRLC(bzpf.cast(), bwt.cast(), IsLastdata.cast());

            if IsLastdata.as_bool() && BZP_BUFF_READ_EMPTY!(bzpf).as_bool() {
                ret = BzpCompressOneBlock(bzpInfo.cast(), outData.cast()).cast();

                BzpWriteFileEnd(outData.cast(), bwt.combinedCRC.cast());
                BzpFlushbuf(outData.cast());

                bzpf.state = BZP_OUTPUT_COMPRESS!();
            } else if BZP_BLOCK_FULL!(bwt).as_bool() {
                ret = BzpCompressOneBlock(bzpInfo.cast(), outData.cast()).cast();
                bzpf.state = BZP_OUTPUT_COMPRESS!();
            } else {
                bzpf.state = BZP_RETUEN_COMPRESS!();
            }
        }
        if (ret != BZP_OK!()).as_bool() {
            return ret;
        }
    }
    return ret.cast();
}
