pub fn CmptPriceGenAlignTable(mut encCtx: Ptr<CmptLzEncCtx>) {
    c_for!(let mut i: u32 = 0; i < (1 << CMPTLZ_ALIGN_BITS!()); i.suffix_plus_plus(); {
        encCtx.priceAlignTable[i] = CmptPriceSymbolReverse(encCtx.cast(), encCtx.probAlign.cast(), CMPTLZ_ALIGN_BITS!(), i.cast()).cast();
    });
}
