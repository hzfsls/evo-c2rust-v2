pub fn CmptlzPriceInit(mut encCtx: Ptr<CmptLzEncCtx>) {
    CmptPriceGenRootTable(encCtx.cast());
    CmptPriceGenDistTable(encCtx.cast());
    CmptPriceGenAlignTable(encCtx.cast());
}
