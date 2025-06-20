pub fn CmptPriceGenLenTable(mut encCtx: Ptr<CmptLzEncCtx>, mut lenEncoder: Ptr<CmptLenEncoder>) {
    let numPosStates: u32 = 1 << encCtx.posBits;
    let mut b: u32;
    let mut prob: u32 = lenEncoder.low[0] as u32;
    let mut a: u32;
    let mut c: u32;
    let mut posState: u32;
    b = CmptPriceBit1(encCtx, prob as u16);
    a = CmptPriceBit0(encCtx, prob as u16);
    c = b + CmptPriceBit0(encCtx, lenEncoder.low[1 << CMPT_LEN_LOW_BITS!()]);
    c_for!(posState = 0; posState < numPosStates; posState += 1; {
        let mut prices: Ptr<u32> = lenEncoder.prices[posState].cast();
        let mut probs: Ptr<CmptlzProb> = lenEncoder.low.cast::<Ptr<u16>>() + (posState << (1 + CMPT_LEN_LOW_BITS!()));
        CmptPriceSet(encCtx, probs, a, prices);
        CmptPriceSet(encCtx, probs + (1 << CMPT_LEN_LOW_BITS!()), c, prices + (1 << CMPT_LEN_LOW_BITS!()));
    });
    let mut i: u32 = lenEncoder.tableSize;
    if i > (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!() {
        let mut probs: Ptr<CmptlzProb> = lenEncoder.high.cast();
        let mut prices: Ptr<u32> = lenEncoder.prices[0].cast::<Ptr<u32>>()  + (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!();
        i -= (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!() - 1;
        i >>= 1;
        b += CmptPriceBit1(encCtx, lenEncoder.low[1 << CMPT_LEN_LOW_BITS!()]);
        loop {
            i -= 1;
            let mut sym: u32 = i + (1 << (CMPT_LEN_HIGH_BITS!() - 1));
            let mut price: u32 = b;
            loop {
                let mut bit: u32 = sym & 1;
                sym >>= 1;
                price += CmptPriceOneBit(encCtx, probs[sym], bit);
                if sym < 2 {
                    break;
                }
            }
            prob = probs[(i + (1 << (CMPT_LEN_HIGH_BITS!() - 1))) as usize] as u32;
            prices[(i * CMPT_DOUBLE!()) as usize] = price + CmptPriceBit0(encCtx, prob as u16);
            prices[(i * CMPT_DOUBLE!() + 1) as usize] = price + CmptPriceBit1(encCtx, prob as u16);
            if i == 0 {
                break;
            }
        }
        let mut num: u32 = (lenEncoder.tableSize - (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!()) as u32 * c_sizeofval!(lenEncoder.prices[0][0]);
        c_for!(posState = 1; posState < numPosStates; posState += 1; {
            c_memcpy_s!(lenEncoder.prices[posState].cast::<Ptr<u32>>() + (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!(), CMPT_MF_LONGEST_MATCH!() - 1,
                        lenEncoder.prices[0].cast::<Ptr<u32>>() + (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!(), num);
        });
    }
}