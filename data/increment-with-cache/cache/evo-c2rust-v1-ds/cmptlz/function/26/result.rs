pub fn CmptlzDpReverse(mut encCtx: Ptr<CmptLzEncCtx>, mut cur: u32) {
    encCtx.optEndIndex = cur.cast();
    let mut posTmp: u32 = encCtx.opts[cur].posPrev.cast();
    let mut backTmp: u32 = encCtx.opts[cur].backPrev.cast();
    let mut posPrev: u32 = Default::default();
    let mut backCurPacket: u32 = Default::default();
    c_do!({
        posPrev = posTmp.cast();
        backCurPacket = backTmp.cast();

        backTmp = encCtx.opts[posPrev].backPrev.cast();
        posTmp = encCtx.opts[posPrev].posPrev.cast();

        encCtx.opts[posPrev].backPrev = backCurPacket.cast();
        encCtx.opts[posPrev].posPrev = cur.cast();
        cur = posPrev.cast();
    } while cur != 0);
    encCtx.lenRes = encCtx.opts[0].posPrev.cast();
    encCtx.backRes = encCtx.opts[0].backPrev.cast();
    encCtx.optsCurIndex = encCtx.opts[0].posPrev.cast();
}
