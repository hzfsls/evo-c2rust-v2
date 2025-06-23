pub fn BzpProcessData(mut bzpInfo: Ptr<BzpAlgorithmInfo>, mut IsLastdata: bool) -> i32 {
    let mut bzpf: Ptr<BzpFile> = bzpInfo.compressFile;
    let mut outData: Ptr<BzpOutComdata> = bzpInfo.outData;
    let mut bwt: Ptr<BzpBwtInfo> = bzpInfo.bwt;
    bzpf.state = BZP_INPUT_COMPRESS!();
    let mut ret: i32 = BZP_OK!();
    while bzpf.state != BZP_RETUEN_COMPRESS!() {
        if bzpf.state == BZP_OUTPUT_COMPRESS!() {
            ret = BzpBuffToStream(bzpf, outData);
            BzpResetCompress(bwt, outData);
            bzpf.state = BZP_INPUT_COMPRESS!();
            if IsLastdata && BZP_BUFF_READ_EMPTY!(bzpf) {
                bzpf.state = BZP_RETUEN_COMPRESS!();
            }
        }
        if bzpf.state == BZP_INPUT_COMPRESS!() {
            BzpBuffToBlockRLC(bzpf, bwt, IsLastdata);
            if IsLastdata && BZP_BUFF_READ_EMPTY!(bzpf) {
                ret = BzpCompressOneBlock(bzpInfo, outData);
                BzpWriteFileEnd(outData, bwt.combinedCRC.cast());
                BzpFlushbuf(outData);
                bzpf.state = BZP_OUTPUT_COMPRESS!();
            } else if BZP_BLOCK_FULL!(bwt) {
                ret = BzpCompressOneBlock(bzpInfo, outData);
                bzpf.state = BZP_OUTPUT_COMPRESS!();
            } else {
                bzpf.state = BZP_RETUEN_COMPRESS!();
            }
        }
        if ret != BZP_OK!() {
            return ret;
        }
    }
    return ret;
}