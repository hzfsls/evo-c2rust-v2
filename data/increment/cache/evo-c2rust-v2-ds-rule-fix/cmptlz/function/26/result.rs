pub fn CmptlzDpReverse(mut encCtx: Ptr<CmptLzEncCtx>, mut cur: u32) {
    encCtx.optEndIndex = cur;
    let mut posTmp: u32 = encCtx.opts[cur].posPrev;
    let mut backTmp: u32 = encCtx.opts[cur].backPrev;
    let mut posPrev: u32;
    let mut backCurPacket: u32;
    c_do!({
        posPrev = posTmp;
        backCurPacket = backTmp;

        backTmp = encCtx.opts[posPrev].backPrev;
        posTmp = encCtx.opts[posPrev].posPrev;

        encCtx.opts[posPrev].backPrev = backCurPacket;
        encCtx.opts[posPrev].posPrev = cur;
        cur = posPrev;
    } while cur != 0);
    encCtx.lenRes = encCtx.opts[0].posPrev;
    encCtx.backRes = encCtx.opts[0].backPrev;
    encCtx.optsCurIndex = encCtx.opts[0].posPrev;
}
