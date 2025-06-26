pub fn CmptPriceLiteral(mut encCtx: Ptr<CmptLzEncCtx>, mut matchMode: bool, mut matchByte: u32, mut symbol: u32) -> u32 {
    let mut pos: u32 = encCtx.litMarcov.pos;
    let mut prevByte: u32 = encCtx.litMarcov.prevByte;
    let mut litCtx: u32 = encCtx.litMarcov.lcBits;
    let mut lpMask: u32 = encCtx.litMarcov.posMask;
    let mut subCoder: Ptr<CmptlzProb> = CMPT_LIT_SUBCODER!(encCtx.litMarcov.literal, litCtx, lpMask, pos, prevByte).cast::<Ptr<CmptlzProb>>();
    let mut price: u32 = 0;
    if !matchMode {
        price = CmptPriceSymbol(encCtx, subCoder, 8, symbol);
    } else {
        let mut offset: u32 = 0x100;
        symbol += 1 << 8;
        c_do!({
            matchByte <<= 1;
            let mut matchBit: u32 = (matchByte & offset);
            let mut subCoderIndex: u32 = (offset + matchBit + (symbol >> 8));
            let mut bit: u32 = ((symbol >> 7) & 1);
            price += CmptPriceOneBit(encCtx, subCoder[subCoderIndex], bit);
            symbol <<= 1;
            offset &= !(matchByte ^ symbol);
        } while symbol < (1 << 16));
    }
    return price;
}
