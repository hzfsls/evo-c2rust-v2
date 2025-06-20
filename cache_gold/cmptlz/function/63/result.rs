pub fn CmptlzDpTryCurAndLong(mut encCtx: Ptr<CmptLzEncCtx>, mut prefixPrice: u32, mut cur: u32, mut mainRepIndex: u32, mut lenEqual: u32, mut posState: u32) {
    loop {
        let curLongRepPrice = prefixPrice + CmptPriceLen(c_ref!(encCtx.repLenEncoder), lenEqual, posState);
        if curLongRepPrice < encCtx.opts[cur + lenEqual].price {
            encCtx.opts[cur + lenEqual].price = curLongRepPrice;
            encCtx.opts[cur + lenEqual].posPrev = cur;
            encCtx.opts[cur + lenEqual].backPrev = mainRepIndex;
        }
        lenEqual -= 1;
        if lenEqual < CMPTLZ_MATCH_LEN_MIN!() {
            break;
        }        
    }
}