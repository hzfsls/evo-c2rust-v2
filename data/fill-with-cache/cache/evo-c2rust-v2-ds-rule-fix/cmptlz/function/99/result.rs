pub fn CmptPriceGenLenTable(mut encCtx: Ptr<CmptLzEncCtx>, mut lenEncoder: Ptr<CmptLenEncoder>) {
    let numPosStates: u32 = 1 << encCtx.posBits;
    let mut b: u32;
    let mut prob: u32 = lenEncoder.low[0];
    let mut a: u32;
    let mut c: u32;
    let mut posState: u32;
    b = CmptPriceBit1(encCtx, prob.cast());
    a = CmptPriceBit0(encCtx, prob.cast());
    c = (b + CmptPriceBit0(encCtx, lenEncoder.low[1 << CMPT_LEN_LOW_BITS!()]));
    c_for!(posState = 0; posState < numPosStates; posState.suffix_plus_plus(); {
        let mut prices: Ptr<u32> = lenEncoder.prices[posState].cast();
        let mut probs: Ptr<CmptlzProb> = (lenEncoder.low + (posState << (1 + CMPT_LEN_LOW_BITS!())));
        CmptPriceSet(encCtx, probs, a, prices);
        CmptPriceSet(encCtx, (probs + (1 << CMPT_LEN_LOW_BITS!())), c, (prices + (1 << CMPT_LEN_LOW_BITS!())));
    });
    let mut i: u32 = lenEncoder.tableSize;
    if (i > (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!()) {
        let mut probs: Ptr<CmptlzProb> = lenEncoder.high.cast();
        let mut prices: Ptr<u32> = (lenEncoder.prices[0] + (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!());
        i -= (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!() - 1;
        i >>= 1;
        b += CmptPriceBit1(encCtx, lenEncoder.low[(1 << CMPT_LEN_LOW_BITS!())]);
        c_do!({
            let mut sym: u32 = (i.prefix_minus_minus() + (1 << (CMPT_LEN_HIGH_BITS!() - 1)));
            let mut price: u32 = b;
            c_do!({
                let mut bit: u32 = (sym & 1);
                sym >>= 1;
                price += CmptPriceOneBit(encCtx, probs[sym], bit);
            } while sym >= 2);
            prob = probs[(i + (1 << (CMPT_LEN_HIGH_BITS!() - 1)))].cast();
            prices[(i * CMPT_DOUBLE!())] = (price + CmptPriceBit0(encCtx, prob.cast()));
            prices[(i * CMPT_DOUBLE!() + 1)] = (price + CmptPriceBit1(encCtx, prob.cast()));
        } while i != 0);
        let mut num: usize = (lenEncoder.tableSize - (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!()) * c_sizeofval!(lenEncoder.prices[0][0]);
        c_for!(posState = 1; posState < numPosStates; posState.suffix_plus_plus(); {
            let tmp0 = posState;
            c_memcpy_s!((lenEncoder.prices[tmp0] + (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!()), num);
        });
    }
}
