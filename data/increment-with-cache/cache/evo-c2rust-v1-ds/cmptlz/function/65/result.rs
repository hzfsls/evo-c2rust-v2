pub fn CmptPriceGenDistTable(mut encCtx: Ptr<CmptLzEncCtx>) {
    let mut distState: u32 = 0;

    c_do!({
        let mut tmpPriceDistSlot: Ptr<u32> = encCtx.priceDistSlotTable[distState].cast();

        c_for!(let mut i: u32 = 0; i < encCtx.distTableSize; i.suffix_plus_plus(); {
            tmpPriceDistSlot[i] = CmptPriceSymbol(encCtx.cast(), encCtx.probDistSlot[distState].cast(), CMPTLZ_DIST_SLOT_BITS!(), i.cast()).cast();
        });

        c_for!(let mut i: u32 = 14; i < encCtx.distTableSize; i.suffix_plus_plus(); {
            tmpPriceDistSlot[i] += CmptPriceOneBitDirect(((i >> 1) - 1 - CMPTLZ_ALIGN_BITS!()).cast()).cast();
        });

        c_for!(let mut i: u32 = 0; i < 4; i.suffix_plus_plus(); {
            encCtx.priceDistTable[distState][i] = tmpPriceDistSlot[i].cast();
        });

        distState.suffix_plus_plus();
    } while distState < CMPTLZ_DIST_STATE_TOTAL!());

    c_for!(let mut i: u32 = 4; i < 128; i.suffix_plus_plus(); {
        let mut distSlot: u32 = PosSloter(i.cast()).cast();
        let mut footerBits: u32 = ((distSlot >> 1) - 1).cast();
        let mut base: u32 = ((2 | (distSlot & 1)) << footerBits).cast();
        let mut price: u32 = CmptPriceSymbolReverse(encCtx.cast(), (encCtx.probDistSpecial + base - distSlot - 1).cast(), footerBits.cast(), (i - base).cast()).cast();

        c_for!(distState = 0; distState < 4; distState.suffix_plus_plus(); {
            encCtx.priceDistTable[distState][i] = (price + encCtx.priceDistSlotTable[distState][distSlot]).cast();
        });
    });

    encCtx.matchPriceCount = 0;
}
