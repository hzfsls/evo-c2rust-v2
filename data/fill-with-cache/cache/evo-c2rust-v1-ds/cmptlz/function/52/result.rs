pub fn CmptPriceCheck(mut encCtx: Ptr<CmptLzEncCtx>) {
    if encCtx.matchPriceCount >= CMPT_PRICE_COUNT!() {
        CmptPriceGenDistTable(encCtx.cast());
        CmptPriceGenAlignTable(encCtx.cast());
        CmptPriceGenLenTable(encCtx.cast(), c_ref!(encCtx.matchLenEncoder).cast());
    }
    if encCtx.repLenPriceCount <= 0 {
        encCtx.repLenPriceCount = CMPT_PRICE_COUNT!();
        CmptPriceGenLenTable(encCtx.cast(), c_ref!(encCtx.repLenEncoder).cast());
    }
}
