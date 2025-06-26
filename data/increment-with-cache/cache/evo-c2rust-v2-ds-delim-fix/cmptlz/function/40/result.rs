pub fn CmptlzCompress(mut src: Ptr<Void>, mut srcSize: usize, mut dst: Ptr<Void>, mut dstSize: Ptr<usize>, mut param: Ptr<CmptlzCompParam>) -> i32 {
    if (src == NULL!()).as_bool() && (srcSize != 0).as_bool() {
        return CMPT_ENC_ERROR_PARAM!();
    }

    let mut endMarker: i32 = 0;

    let mut props: CmptlzEncParam = Default::default();
    props.level = param.level.cast();
    props.dictSize = param.dictSize.cast();
    props.litCtx = param.litCtx.cast();
    props.litPos = param.litPos.cast();
    props.posBits = param.posBits.cast();
    props.fastBytes = param.fastBytes.cast();
    props.numThreads = param.numThreads.cast();
    let mut alloc: Ptr<CmptLzMemHook> = param.memHook.cast();
    return CmptlzEncode(dst.cast::<Ptr<u8>>(), dstSize.cast(), src.cast::<Ptr<u8>>(), srcSize.cast(), c_ref!(props).cast(), param.protData.cast(), c_ref!(param.protSize).cast(), endMarker.cast(), alloc.cast()).cast();
}
