pub fn VOS_MD5CalcEx(mut output: Ptr<u8>, mut outputLen: u32, mut input: Ptr<u8>, mut inputLen: u32) {
    let mut context: MD5_CTX = Default::default();

    if outputLen < MD5_DIGEST_LEN!() {
        return;
    }

    VOS_MD5Init(c_ref!(context).cast());
    VOS_MD5Update(c_ref!(context).cast(), input.cast::<Ptr<u8>>(), inputLen.cast());
    VOS_MD5FinalEx(output.cast(), outputLen.cast(), c_ref!(context).cast());
}
