use std::ptr::copy_nonoverlapping;

pub fn vos_sha256_hash_by_blc_multi(data: &[u8], ctx: &mut VOS_SHA256_CTX) {
    let mut len_tmp = data.len() as u32;
    let mut src_ptr = data.as_ptr();
    
    let blc_len = len_tmp / SHA256_BLOCK_SIZE as u32;
    if blc_len > 0 {
        vos_sha256_compress_mul(ctx, src_ptr, blc_len);
        let processed_len = blc_len * SHA256_BLOCK_SIZE as u32;
        src_ptr = unsafe { src_ptr.add(processed_len as usize) };
        len_tmp -= processed_len;
    }

    if len_tmp != 0 {
        ctx.blocklen = len_tmp;
        unsafe {
            copy_nonoverlapping(src_ptr, ctx.block.as_mut_ptr(), len_tmp as usize);
        }
    }
}
