pub fn CmptlzSetParam(mut encCtx: Ptr<CmptLzEncCtx>, mut props: Ptr<CmptlzEncParam>) {
    let mut param: CmptlzEncParam = *props;
    CmptlzParamNormalize(c_ref!(param));
    encCtx.dicSize = param.dictSize;
    encCtx.numFastBytes = param.fastBytes.cast();
    encCtx.litCtx = param.litCtx;
    encCtx.litPos = param.litPos;
    encCtx.posBits = param.posBits;
    let mut i: u32 = 7;
    c_for!(; i < 32; i.suffix_plus_plus(); {
        if (encCtx.dicSize <= (1 << i)) {
            break;
        }
    });
    encCtx.distTableSize = i * 2;
}