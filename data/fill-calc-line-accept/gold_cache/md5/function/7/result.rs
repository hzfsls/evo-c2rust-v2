pub fn VOS_MD5Calc(mut output: Ptr<u8>, mut input: Ptr<u8>, mut inputLen: u32) {
    VOS_MD5CalcEx(output, MD5_DIGEST_LEN!(), input, inputLen);
}