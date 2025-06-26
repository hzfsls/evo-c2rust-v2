pub fn CmptlzSetParam(mut encCtx: Ptr<CmptLzEncCtx>, mut props: Ptr<CmptlzEncParam>) {
    let mut param: CmptlzEncParam = *props;

    CmptlzParamNormalize(c_ref!(param).cast());

    encCtx.dicSize = param.dictSize.cast();
    encCtx.numFastBytes = param.fastBytes.cast();
    encCtx.litCtx = param.litCtx.cast();
    encCtx.litPos = param.litPos.cast();
    encCtx.posBits = param.posBits.cast();
    let mut i: u32;
    c_for!(i = 7; i < 32; i.suffix_plus_plus(); {
        if encCtx.dicSize <= (1 << i).cast::<u32>() {
            break;
        }
    });
    encCtx.distTableSize = (i * 2).cast();
}
