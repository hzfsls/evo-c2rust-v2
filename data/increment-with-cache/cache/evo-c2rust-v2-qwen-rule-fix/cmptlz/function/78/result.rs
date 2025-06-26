pub fn CmptPriceDistWithLen(mut encCtx: Ptr<CmptLzEncCtx>, mut dist: u32, mut len: u32, mut posState: u32) -> u32 {
    let mut distState: u32 = CMPT_GET_DIST_STATE!(len);
    let mut price: u32 = Default::default();
    if (dist < 128) {
        price = encCtx.priceDistTable[distState][dist];
    } else {
        let mut distSlot: u32 = PosSloter(dist);
        price = encCtx.priceDistSlotTable[distState][distSlot].cast::<u32>() + encCtx.priceAlignTable[(dist & ((1 << CMPTLZ_ALIGN_BITS!()) - 1))].cast::<u32>();
    }
    price += CmptPriceLen(c_ref!(encCtx.matchLenEncoder), len, posState).cast::<u32>();
    return price;
}