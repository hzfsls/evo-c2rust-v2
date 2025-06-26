pub fn CmptLzDecByDistAndLen(mut decCtx: Ptr<CmptLzDecCtx>, mut matchDist: usize, mut matchLen: u32, mut dicPosLimit: usize) -> u32 {
    let mut dicCopyPos: usize;
    let mut dicPos: usize = decCtx.dictPos.cast();
    let mut dictBufSize: usize = decCtx.dictBufSize.cast();
    let mut remainDicLen: u32 = (dicPosLimit - dicPos).cast();
    let mut dict: Ptr<u8> = decCtx.dict.cast();

    if (remainDicLen == 0).as_bool() {
        return CMPT_ERROR_DATA!();
    }

    let mut decDicLen: u32 = if remainDicLen < matchLen { remainDicLen } else { matchLen };
    decCtx.processedPos += decDicLen.cast();
    decCtx.dictPos += decDicLen.cast();
    decCtx.remainLen = (matchLen - decDicLen).cast();

    if (dicPos < matchDist).as_bool() {
        dicCopyPos = dictBufSize - matchDist + dicPos;
    } else {
        dicCopyPos = dicPos - matchDist;
    }

    c_do!({
        dict[dicPos] = dict[dicCopyPos].cast();
        dicPos += 1;
        if (dicCopyPos.prefix_plus_plus() == dictBufSize).as_bool() {
            dicCopyPos = 0;
        }
    } while decDicLen.suffix_minus_minus() != 0);

    return CMPT_OK!();
}
