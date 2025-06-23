pub fn CmptlzDpTryCurAndLong(mut encCtx: Ptr<CmptLzEncCtx>, mut prefixPrice: u32, mut cur: u32, mut mainRepIndex: u32, mut lenEqual: u32, mut posState: u32) {
    c_do!({
        let mut curLongRepPrice: u32 = (prefixPrice + CmptPriceLen(c_ref!(encCtx.repLenEncoder).cast(), lenEqual.cast(), posState.cast())).cast();
        if (curLongRepPrice < encCtx.opts[cur + lenEqual].price).as_bool() {
            encCtx.opts[cur + lenEqual].price = curLongRepPrice.cast();
            encCtx.opts[cur + lenEqual].posPrev = cur.cast();
            encCtx.opts[cur + lenEqual].backPrev = mainRepIndex.cast();
        }
    } while lenEqual.prefix_minus_minus() >= CMPTLZ_MATCH_LEN_MIN!());
}
