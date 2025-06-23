use std::ptr;

pub fn vos_sha256_hash(puc_data: &[u8], pst_ctx: &mut VOS_SHA256_CTX) {
    let mut ui_len_tmp = puc_data.len() as u32;
    let mut puc_src = puc_data.as_ptr();
    
    if puc_src.is_null() || ui_len_tmp == 0 || pst_ctx.corrupted == 1 || 
       pst_ctx.computed == 1 || vos_sha256_ctx_prepare(pst_ctx, ui_len_tmp) != SHA256_OK {
        return;
    }
    
    let mut ui_blc_len = 0;
    if pst_ctx.blocklen != 0 {
        if vos_sha256_last_padding(puc_src, ui_len_tmp, pst_ctx, &mut ui_blc_len) == SHA256_OK {
            unsafe {
                puc_src = puc_src.add(ui_blc_len as usize);
            }
            ui_len_tmp -= ui_blc_len;
        } else {
            return;
        }
    }
    
    vos_sha256_hash_by_blc_multi(puc_src, ui_len_tmp, pst_ctx);
}
