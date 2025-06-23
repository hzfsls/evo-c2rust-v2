pub fn VOS_MD5Calc(mut output: Ptr<u8>, mut input: Ptr<u8>, mut inputLen: u32) {
    VOS_MD5CalcEx(output.cast(), MD5_DIGEST_LEN!(), input.cast(), inputLen.cast());
}
