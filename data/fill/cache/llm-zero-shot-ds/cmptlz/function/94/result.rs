pub fn cmpt_price_gen_root_table(enc_ctx: &mut CmptLzEncCtx) {
    let root_table = &mut enc_ctx.price_root_table;
    const EXPAND_CYCLE_NUM: u32 = 4;
    const BITS_TOTAL_MODE_NUM: u32 = 11;
    const VALUE_FOR_NORMAL: u32 = 15;
    const W_TOP_BOARDER: u32 = 1 << 16;
    
    for i in 0..((1u32 << BITS_TOTAL_MODE_NUM) >> CMPT_PRICE_BITS_MOVING_NUM) {
        let mut w = (i << CMPT_PRICE_BITS_MOVING_NUM) + (1 << (CMPT_PRICE_BITS_MOVING_NUM - 1));
        let mut dummy_normalize_cnt = 0;
        
        for _ in 0..EXPAND_CYCLE_NUM {
            w = w * w;
            dummy_normalize_cnt <<= 1;
            
            while w >= W_TOP_BOARDER {
                w >>= 1;
                dummy_normalize_cnt += 1;
            }
        }
        
        root_table[i as usize] = (BITS_TOTAL_MODE_NUM << EXPAND_CYCLE_NUM) - VALUE_FOR_NORMAL - dummy_normalize_cnt;
    }
}
