pub fn vos_md5_final(digest: &mut [u8; 16], context: &mut MD5_CTX) {
    vos_md5_final_ex(digest, MD5_DIGEST_LEN, context);
}
