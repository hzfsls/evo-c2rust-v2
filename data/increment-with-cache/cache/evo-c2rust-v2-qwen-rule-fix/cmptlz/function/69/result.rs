pub fn CmptPriceGenRootTable(mut encCtx: Ptr<CmptLzEncCtx>) {
    let mut rootTable: Ptr<u32> = encCtx.priceRootTable.cast();
    let expandCycleNum: u32 = 4;
    let bitsTotalModeNum: u32 = 11;
    let valueForNormal: u32 = 15;
    let wTopBoarder: u32 = 1 << 16;
    c_for!(let mut i: u32 = 0; i < ((1 << bitsTotalModeNum) >> CMPT_PRICE_BITS_MOVING_NUM!()).cast(); i.suffix_plus_plus(); {
        let mut w: u32 = (i << CMPT_PRICE_BITS_MOVING_NUM!()) + (1 << (CMPT_PRICE_BITS_MOVING_NUM!() - 1));
        let mut dummyNormalizeCnt: u32 = 0;
        c_for!(let mut j: u32 = 0; j < expandCycleNum; j.suffix_plus_plus(); {
            w = w * w;
            dummyNormalizeCnt <<= 1;
            while (w >= wTopBoarder).as_bool() {
                w >>= 1;
                dummyNormalizeCnt += 1;
            }
        });
        rootTable[i] = ((bitsTotalModeNum << expandCycleNum) - valueForNormal - dummyNormalizeCnt).cast();
    });
}