pub fn cmpt_encode_one_block(enc_ctx: &mut CmptLzEncCtx) -> Result<(), CmptError> {
    let mf = &mut enc_ctx.mf_ctx;
    let nowpos32 = enc_ctx.nowpos64 as u32;
    let startpos = nowpos32;
    
    loop {
        cmptlz_dp(enc_ctx, mf, nowpos32)?;
        let back_res = enc_ctx.back_res;
        let len_res = enc_ctx.len_res;
        
        #[cfg(feature = "printf_enc_process")]
        {
            println!(" now in CmptEncodeOneBlock process, backRes is {}, lenRes is {}", back_res, len_res);
            println!(" nowpos32 is {}", nowpos32);
        }

        match back_res {
            CMPTLZ_UINT32_MAX => {
                cmptlz_enc_lit(enc_ctx, mf, nowpos32)?;
            }
            0 => {
                cmpt_enc_short_or_rep0(enc_ctx, nowpos32, len_res)?;
            }
            1 => {
                cmptlz_enc_long_rep(enc_ctx, 1, nowpos32, len_res)?;
            }
            2 => {
                cmptlz_enc_long_rep(enc_ctx, 2, nowpos32, len_res)?;
            }
            3 => {
                cmptlz_enc_long_rep(enc_ctx, 3, nowpos32, len_res)?;
            }
            _ => {
                cmptlz_enc_normal_match(enc_ctx, nowpos32, back_res, len_res)?;
            }
        }

        let len_res = len_res as u32;
        enc_ctx.nowpos64 += len_res as u64;
        mf.mf_start += len_res;
        mf.read_ahead -= len_res;

        if mf.read_ahead == 0 {
            cmpt_price_check(enc_ctx)?;
            
            if mf.src_len <= mf.mf_start {
                break;
            }
            if nowpos32 - startpos >= CMPT_ONE_BLOCK_MAX_SIZE {
                enc_ctx.nowpos64 += (nowpos32 - startpos) as u64;
                return Ok(());
            }
        }
    }
    
    enc_ctx.nowpos64 += (nowpos32 - startpos) as u64;
    cmptlz_flush(enc_ctx)
}
