pub fn CmptLzDecByDistAndLen(mut decCtx: Ptr<CmptLzDecCtx>, mut matchDist: usize, mut matchLen: u32, mut dicPosLimit: usize) -> u32 {
    let mut dicCopyPos: usize;
    let mut dicPos: usize = decCtx.dictPos;
    let mut dictBufSize: usize = decCtx.dictBufSize;
    let mut remainDicLen: u32 = (dicPosLimit - dicPos).cast();
    let mut dict: Ptr<u8> = decCtx.dict;

    if (remainDicLen == 0) {
        return CMPT_ERROR_DATA!().cast::<u32>();
    }

    let mut decDicLen: u32 = if remainDicLen < matchLen { remainDicLen } else { matchLen };
    decCtx.processedPos += decDicLen;
    decCtx.dictPos += decDicLen.cast::<usize>();
    decCtx.remainLen = (matchLen - decDicLen);

    if (dicPos < matchDist) {
        dicCopyPos = dictBufSize - matchDist + dicPos;
    } else {
        dicCopyPos = dicPos - matchDist;
    }

    c_do!({
        dict[dicPos] = dict[dicCopyPos];
        dicPos += 1;
        if (dicCopyPos.prefix_plus_plus() == dictBufSize) {
            dicCopyPos = 0;
        }
    } while decDicLen.suffix_minus_minus() != 0);

    return CMPT_OK!();
}
