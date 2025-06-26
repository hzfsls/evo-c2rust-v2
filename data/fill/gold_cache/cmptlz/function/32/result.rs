pub fn CmptLzDecByDistAndLen(mut decCtx: Ptr<CmptLzDecCtx>, mut matchDist: usize, mut matchLen: u32, mut dicPosLimit: usize) -> u32 {
    let mut dicCopyPos: usize;
    let mut dicPos: usize = decCtx.dictPos;
    let mut dictBufSize: usize = decCtx.dictBufSize;
    let mut remainDicLen: u32 = (dicPosLimit - dicPos).cast();
    let mut dict: Ptr<u8> = decCtx.dict.cast();
    if remainDicLen == 0 {
        return CMPT_ERROR_DATA!();
    }
    let mut decDicLen: u32 = if remainDicLen < matchLen { remainDicLen } else { matchLen };
    decCtx.processedPos += decDicLen;
    decCtx.dictPos += decDicLen as usize;
    decCtx.remainLen = matchLen - decDicLen;
    if dicPos < matchDist {
        dicCopyPos = dictBufSize - matchDist + dicPos;
    } else {
        dicCopyPos = dicPos - matchDist;
    }
    c_do!({
        dict[dicPos.plus_plus()] = dict[dicCopyPos];
        if plus_plus!(dicCopyPos) == dictBufSize {
            dicCopyPos = 0;
        }       
    } while minus_minus!(decDicLen) != 0);
    return CMPT_OK!();
}