pub fn RapidlzCompressStream(mut strmCtx: Ptr<RapidlzStreamCtx>, mut src: Ptr<u8>, mut dst: Ptr<u8>, mut srcSize: i32, mut dstSize: i32) -> i32 {
    let mut rapidlzEncFunc: RapidlzCompressFunc = NULL!();
    RAPIDLZ_RETURN_IF_NOT_TRUE!(!(srcSize > RAPIDLZ_MAX_INPUT_SIZE!()).as_bool(), RAPIDLZ_ENC_NOT_OK!());
    RAPIDLZ_RETURN_IF_NOT_TRUE!(!((src == NULL!() && srcSize != 0).as_bool() || (dstSize <= 0).as_bool() || (dst == NULL!()).as_bool(), RAPIDLZ_ENC_NOT_OK!());
    if (srcSize == 0).as_bool() {
        return RapidlzZeroBytesEncode(dst.cast(), dstSize.cast());
    }
    let mut dictEnd: Ptr<u8> = if (strmCtx.dictSize != 0).as_bool() { strmCtx.dict.cast::<Ptr<u8>>() + strmCtx.dictSize } else { NULL!() };
    let mut cSize: i32;
    if (dictEnd == src).as_bool() {
        rapidlzEncFunc = RapidlzCompWithPrefixDict;
    } else {
        if (strmCtx.strmCtxSpecific != NULL!()).as_bool() {
            RAPIDLZ_RETURN_IF_NOT_EOK!(c_memcpy_s!(strmCtx, c_sizeofval!(strmCtx), strmCtx.strmCtxSpecific, c_sizeofval!(strmCtx)), RAPIDLZ_ENC_NOT_OK!());
        }
        rapidlzEncFunc = RapidlzCompWithExternalDict;
    }
    RapidlzStrmCtxNorm(strmCtx.cast(), src.cast(), srcSize.cast(), dictEnd.cast());
    cSize = rapidlzEncFunc(strmCtx.cast(), src.cast(), dst.cast(), srcSize.cast(), dstSize.cast());
    strmCtx.dictSize = srcSize.cast();
    strmCtx.dict = src.cast::<Ptr<u8>>();
    return cSize.cast();
}
