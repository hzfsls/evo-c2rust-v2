pub fn cmpt_price_dist_with_len(
    enc_ctx: &CmptLzEncCtx,
    dist: u32,
    len: u32,
    pos_state: u32,
) -> u32 {
    let dist_state = cmpt_get_dist_state(len);
    let price = if dist < 128 {
        enc_ctx.price_dist_table[dist_state as usize][dist as usize]
    } else {
        let dist_slot = pos_sloter(dist);
        enc_ctx.price_dist_slot_table[dist_state as usize][dist_slot as usize]
            + enc_ctx.price_align_table[(dist & ((1 << CMPTLZ_ALIGN_BITS) - 1)) as usize]
    };

    price + cmpt_price_len(&enc_ctx.match_len_encoder, len, pos_state)
}
