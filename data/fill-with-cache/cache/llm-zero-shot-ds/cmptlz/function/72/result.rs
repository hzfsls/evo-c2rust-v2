pub fn cmptlz_enc_prepare(enc_ctx: &mut CmptLzEncCtx) {
    enc_ctx.enc_need_finish = false;
    enc_ctx.cmptlz_response = 0;
    enc_ctx.nowpos64 = 0;
    enc_ctx.state = 0;
    enc_ctx.pb_mask = (1 << enc_ctx.pos_bits) - 1;
    enc_ctx.lp_mask = ((0x100u32) << enc_ctx.lit_pos) - ((0x100u32) >> enc_ctx.lit_ctx);
    enc_ctx.pos_mask = (1 << enc_ctx.pos_bits) - 1;
    
    for rep in enc_ctx.reps.iter_mut() {
        *rep = 0;
    }
    
    enc_ctx.opts_cur_index = 0;
    enc_ctx.opt_end_index = 0;
    
    for opt in enc_ctx.opts.iter_mut().take(CMPT_DP_OPTMAX) {
        opt.price = CMPT_INFINITY_PRICE;
    }
    
    for i in 0..CMPTLZ_NUM_STATES {
        for j in 0..CMPTLZ_NUM_PB_STATES_MAX {
            enc_ctx.is_match[i][j] = CMPTLZ_PROB_INIT;
            enc_ctx.is_rep0_long[i][j] = CMPTLZ_PROB_INIT;
        }
        enc_ctx.is_rep[i] = CMPTLZ_PROB_INIT;
        enc_ctx.is_rep_g0[i] = CMPTLZ_PROB_INIT;
        enc_ctx.is_rep_g1[i] = CMPTLZ_PROB_INIT;
        enc_ctx.is_rep_g2[i] = CMPTLZ_PROB_INIT;
    }
    
    for i in 0..CMPTLZ_DIST_STATE_TOTAL {
        for j in 0..(1 << CMPTLZ_DIST_SLOT_BITS) {
            enc_ctx.prob_dist_slot[i][j] = CMPTLZ_PROB_INIT;
        }
    }
    
    for i in 0..CMPT_DIST_LIMIT_2 {
        enc_ctx.prob_dist_special[i] = CMPTLZ_PROB_INIT;
    }
    
    for i in 0..(1 << CMPTLZ_ALIGN_BITS) {
        enc_ctx.prob_align[i] = CMPTLZ_PROB_INIT;
    }
    
    enc_ctx.lit_marcov.lc_bits = enc_ctx.lit_ctx;
    enc_ctx.lit_marcov.pos_mask = (1 << enc_ctx.lit_pos) - 1;
    
    for i in 0..(1 << CMPTLZ_LCLP_MAX) {
        for j in 0..CMPTLZ_LIT_MAX_SIZE {
            enc_ctx.lit_marcov.literal[i][j] = CMPTLZ_PROB_INIT;
        }
    }
    
    for i in 0..(1 << CMPT_LEN_HIGH_BITS) {
        enc_ctx.match_len_encoder.high[i] = CMPTLZ_PROB_INIT;
        enc_ctx.rep_len_encoder.high[i] = CMPTLZ_PROB_INIT;
        enc_ctx.match_len_encoder.low[i] = CMPTLZ_PROB_INIT;
        enc_ctx.rep_len_encoder.low[i] = CMPTLZ_PROB_INIT;
    }
    
    cmptlz_price_init(enc_ctx);
    enc_ctx.rep_len_encoder.table_size = enc_ctx.num_fast_bytes - 1;
    enc_ctx.match_len_encoder.table_size = enc_ctx.num_fast_bytes - 1;
    cmpt_price_gen_len_table(enc_ctx, &mut enc_ctx.match_len_encoder);
    cmpt_price_gen_len_table(enc_ctx, &mut enc_ctx.rep_len_encoder);
}
