pub fn CmptPriceCheck(mut encCtx: Ptr<CmptLzEncCtx>) {
    if encCtx.matchPriceCount >= CMPT_PRICE_COUNT!() {
        CmptPriceGenDistTable(encCtx);
        CmptPriceGenAlignTable(encCtx);
        CmptPriceGenLenTable(encCtx, c_ref!(encCtx.matchLenEncoder));
    }
    if encCtx.repLenPriceCount <= 0 {
        encCtx.repLenPriceCount = CMPT_PRICE_COUNT!();
        CmptPriceGenLenTable(encCtx, c_ref!(encCtx.repLenEncoder));
    }
}