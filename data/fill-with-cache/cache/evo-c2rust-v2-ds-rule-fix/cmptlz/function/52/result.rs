pub fn CmptPriceCheck(mut encCtx: Ptr<CmptLzEncCtx>) {
    if (encCtx.matchPriceCount >= CMPT_PRICE_COUNT!()).as_bool() {
        CmptPriceGenDistTable(encCtx.cast());
        CmptPriceGenAlignTable(encCtx.cast());
        CmptPriceGenLenTable(encCtx.cast(), c_ref!(encCtx.matchLenEncoder).cast());
    }
    if (encCtx.repLenPriceCount <= 0).as_bool() {
        encCtx.repLenPriceCount = CMPT_PRICE_COUNT!();
        CmptPriceGenLenTable(encCtx.cast(), c_ref!(encCtx.repLenEncoder).cast());
    }
}
