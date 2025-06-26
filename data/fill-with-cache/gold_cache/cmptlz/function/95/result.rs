pub fn CmptPriceGenDistTable(mut encCtx: Ptr<CmptLzEncCtx>) {
    let mut distState: u32 = 0;
    loop {
        let mut tmpPriceDistSlot: Ptr<u32> = encCtx.priceDistSlotTable[distState].cast();
        c_for!(let mut i = 0; i < encCtx.distTableSize; i += 1; {
            tmpPriceDistSlot[i] = CmptPriceSymbol(encCtx, encCtx.probDistSlot[distState].cast(), CMPTLZ_DIST_SLOT_BITS!(), i);
        });
        c_for!(let mut i = 14; i < encCtx.distTableSize; i += 1; {
            tmpPriceDistSlot[i] += CmptPriceOneBitDirect((i >> 1) - 1 - CMPTLZ_ALIGN_BITS!());
        });
        c_for!(let mut i = 0; i < 4; i += 1; {
            encCtx.priceDistTable[distState][i] = tmpPriceDistSlot[i];
        });
        distState += 1;
        if distState >= CMPTLZ_DIST_STATE_TOTAL!() {
            break;
        }
    }
    c_for!(let mut i = 4; i < 128; i += 1; {
        let mut distSlot: u32 = PosSloter(i);
        let mut footerBits: u32 = (distSlot >> 1) - 1;
        let mut base: u32 = (2 | (distSlot & 1)) << footerBits;
        let mut price: u32 = CmptPriceSymbolReverse(encCtx, encCtx.probDistSpecial.cast::<Ptr<u16>>() + base - distSlot - 1, footerBits, i - base);
        c_for!(let mut distState = 0; distState < 4; distState += 1; {
            encCtx.priceDistTable[distState][i] = price + encCtx.priceDistSlotTable[distState][distSlot];
        });
    });
    encCtx.matchPriceCount = 0;
}