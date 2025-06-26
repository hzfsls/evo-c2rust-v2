pub fn CmptlzDpTryCurAndLit(mut encCtx: Ptr<CmptLzEncCtx>, mut curPrice: u32, mut curState: CmptlzState, mut posState: u32, mut cur: u32, mut latestMatchByte: u8, mut curByte: u8) {
    let mut isLiteralState: bool = (curState < 7).as_bool();
    let mut isMatchMode: bool = !isLiteralState;
    let mut curAndLitPrice: u32 = curPrice + CmptPriceBit0(encCtx.cast(), encCtx.isMatch[curState][posState].cast()).cast() + CmptPriceLiteral(encCtx.cast(), isMatchMode.cast(), latestMatchByte.cast(), curByte.cast()).cast();
    if (curAndLitPrice < encCtx.opts[cur + 1].price).as_bool() {
        encCtx.opts[cur + 1].price = curAndLitPrice.cast();
        encCtx.opts[cur + 1].posPrev = cur.cast();
        encCtx.opts[cur + 1].backPrev = CMPTLZ_UINT32_MAX!();
    }
}
