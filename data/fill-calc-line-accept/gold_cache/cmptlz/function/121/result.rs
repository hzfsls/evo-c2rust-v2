pub fn CmptlzCompress(mut src: VoidPtr, mut srcSize: usize, mut dst: VoidPtr, mut dstSize: Ptr<usize>, mut param: Ptr<CmptlzCompParam>) -> i32 {
    if (src == NULL!()) && (srcSize != 0) {
        return CMPT_ENC_ERROR_PARAM!();
    }
    let endMarker: i32 = 0;
    let mut props: CmptlzEncParam = Default::default();
    props.level = param.level;
    props.dictSize = param.dictSize;
    props.litCtx = param.litCtx;
    props.litPos = param.litPos;
    props.posBits = param.posBits;
    props.fastBytes = param.fastBytes;
    props.numThreads = param.numThreads;
    let alloc: Ptr<CmptLzMemHook> = param.memHook;
    return CmptlzEncode(dst.cast(), dstSize, src.cast(), srcSize, c_ref!(props), param.protData,
                        c_ref!(param.protSize), endMarker, alloc);
}