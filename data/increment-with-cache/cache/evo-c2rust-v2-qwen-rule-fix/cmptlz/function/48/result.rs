pub fn CmptLzDecByDistAndLen(mut decCtx: Ptr<CmptLzDecCtx>, mut matchDist: usize, mut matchLen: u32, mut dicPosLimit: usize) -> i32 {
    let mut dicCopyPos: usize = Default::default();
    let mut dicPos: usize = decCtx.dictPos;
    let mut dictBufSize: usize = decCtx.dictBufSize;
    let mut remainDicLen: u32 = (dicPosLimit - dicPos).cast();
    let mut dict: Ptr<u8> = decCtx.dict;

    if remainDicLen == 0 {
        return CMPT_ERROR_DATA!();
    }

    let mut decDicLen: u32 = if remainDicLen < matchLen { remainDicLen } else { matchLen };
    decCtx.processedPos += decDicLen;
    decCtx.dictPos += decDicLen.cast();
    decCtx.remainLen = matchLen - decDicLen;

    if dicPos < matchDist {
        dicCopyPos = dictBufSize - matchDist + dicPos;
    } else {
        dicCopyPos = dicPos - matchDist;
    }

    c_do!({
        dict[dicPos] = dict[dicCopyPos];
        dicPos.suffix_plus_plus();
        if dicCopyPos.suffix_plus_plus() == dictBufSize {
            dicCopyPos = 0;
        }
    } while decDicLen.suffix_minus_minus() != 0);

    return CMPT_OK!();
}