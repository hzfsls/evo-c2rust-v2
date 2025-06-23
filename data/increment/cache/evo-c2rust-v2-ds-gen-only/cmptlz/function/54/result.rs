pub fn CmptPriceOneBitDirect(mut bit: u32) -> u32 {
    return (bit << CMPT_PRICE_BITS_MOVING_NUM!()).cast();
}
