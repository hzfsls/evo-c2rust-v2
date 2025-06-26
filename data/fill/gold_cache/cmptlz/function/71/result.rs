pub fn CmptlzPriceInit(mut encCtx: Ptr<CmptLzEncCtx>) {
    CmptPriceGenRootTable(encCtx);
    CmptPriceGenDistTable(encCtx);
    CmptPriceGenAlignTable(encCtx);
}