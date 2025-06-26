use std::ptr::copy_nonoverlapping;

pub fn vos_sha256_hash_by_blc_multi(puc_data: &[u8], ui_len: u32, pst_ctx: &mut VOS_SHA256_CTX) {
    let mut ui_len_tmp = ui_len;
    let mut puc_src = puc_data.as_ptr();
    let ui_blc_len = ui_len_tmp / SHA256_BLOCK_SIZE as u32;
    
    if ui_blc_len > 0 {
        vos_sha256_compress_mul(pst_ctx, puc_src, ui_blc_len);
        let processed_len = ui_blc_len * SHA256_BLOCK_SIZE as u32;
        puc_src = unsafe { puc_src.add(processed_len as usize) };
        ui_len_tmp -= processed_len;
    }
    
    if ui_len_tmp != 0 {
        pst_ctx.blocklen = ui_len_tmp;
        unsafe {
            copy_nonoverlapping(
                puc_src,
                pst_ctx.block.as_mut_ptr(),
                ui_len_tmp as usize
            );
        }
    }
}
