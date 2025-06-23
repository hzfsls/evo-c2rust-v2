pub fn CmptlzCompress(mut src: Ptr<Void>, mut srcSize: usize, mut dst: Ptr<Void>, mut dstSize: Ptr<usize>, mut param: Ptr<CmptlzCompParam>) -> i32 {
    if (src == NULL!()) && (srcSize != 0) {
        return CMPT_ENC_ERROR_PARAM!();
    }
    let mut endMarker: i32 = 0;
    let mut props: CmptlzEncParam = Default::default();
    props.level = param.level;
    props.dictSize = param.dictSize;
    props.litCtx = param.litCtx;
    props.litPos = param.litPos;
    props.posBits = param.posBits;
    props.fastBytes = param.fastBytes;
    props.numThreads = param.numThreads;
    let mut alloc: Ptr<CmptLzMemHook> = param.memHook;
    return CmptlzEncode(dst.cast::<Ptr<u8>>(), dstSize, src.cast::<Ptr<u8>>(), srcSize, c_ref!(props), param.protData, c_ref!(param.protSize), endMarker, alloc);
}