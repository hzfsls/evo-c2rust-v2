pub fn CmptlzDpTryCurAndLit(mut encCtx: Ptr<CmptLzEncCtx>, mut curPrice: u32, mut curState: CmptlzState,
                            mut posState: u32, mut cur: u32, mut latestMatchByte: u8, mut curByte: u8) {
    let isLiteralState: bool = curState < 7;
    let isMatchMode: bool = !isLiteralState;
    let curAndLitPrice: u32 = curPrice + CmptPriceBit0(encCtx, encCtx.isMatch[curState][posState]) +
                              CmptPriceLiteral(encCtx, isMatchMode, latestMatchByte as u32, curByte as u32);
    if curAndLitPrice < encCtx.opts[cur + 1].price {
        encCtx.opts[cur + 1].price = curAndLitPrice;
        encCtx.opts[cur + 1].posPrev = cur;
        encCtx.opts[cur + 1].backPrev = CMPTLZ_UINT32_MAX!();
    }
}