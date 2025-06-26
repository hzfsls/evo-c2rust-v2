pub fn VOS_MD5Final(mut digest: Ptr<u8>, mut context: Ptr<MD5_CTX>) {
    VOS_MD5FinalEx(digest.cast(), MD5_DIGEST_LEN!(), context.cast());
}
