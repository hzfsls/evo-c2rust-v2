pub fn CmptPriceGenAlignTable(mut encCtx: Ptr<CmptLzEncCtx>) {
    c_for!(let mut i: u32 = 0; i < (1 << CMPTLZ_ALIGN_BITS!()); i += 1; {
        encCtx.priceAlignTable[i] = CmptPriceSymbolReverse(encCtx, encCtx.probAlign.cast(), CMPTLZ_ALIGN_BITS!(), i);
    });
}