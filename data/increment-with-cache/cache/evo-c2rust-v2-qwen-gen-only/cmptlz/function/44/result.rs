pub fn CmptlzDecompress(mut src: Ptr<Void>, mut srcSize: usize, mut dst: Ptr<Void>, mut dstSize: Ptr<usize>, mut param: Ptr<CmptlzDecParam>) -> i32 {
    if (src == NULL!()).as_bool() || (dst == NULL!()).as_bool() || (dstSize == NULL!()).as_bool() {
        CMPTLZ_LOG!(CMPT_ERROR_UNSUPPORTED!(), cstr!("The input parameter NULL is incorrect."));
        return CMPT_ERROR_UNSUPPORTED!();
    }
    if (srcSize > 0x7fffffff).as_bool() || (*dstSize.cast() > 0x7fffffff).as_bool() {
        CMPTLZ_LOG!(CMPT_ERROR_UNSUPPORTED!(), cstr!("dstSize:0x%zx srcSize:0x%zx"), *dstSize.cast(), srcSize.cast());
        return CMPT_ERROR_UNSUPPORTED!();
    }
    if (param == NULL!()).as_bool() || (param.memHook == NULL!()).as_bool() || (param.protData == NULL!()).as_bool() || (param.protSize != CMPTLZ_PROPS_SIZE!()).as_bool() {
        CMPTLZ_LOG!(CMPT_ERROR_UNSUPPORTED!(), cstr!("The compress param NULL is incorrect."));
        return CMPT_ERROR_UNSUPPORTED!();
    }
    let mut decIn: CmptLzDecIn = Default::default();
    decIn.pSrcIn = src.cast();
    decIn.strInLen = srcSize.cast();
    decIn.strInCostLen = 0;
    let mut decOut: CmptLzDecOut = Default::default();
    decOut.pDestOut = dst.cast();
    decOut.destOutLen = *dstSize.cast();
    decOut.destOutFillLen = 0;
    let mut enFinStat: EnCmptLzStatus = CMPTLZ_STATUS_BUT!();
    let mut ret: i32 = CmptLzDecode(c_ref!(decIn).cast(), c_ref!(decOut).cast(), param.protData.cast(), CMPTLZ_FINISH_ANY!(), c_ref!(enFinStat).cast(), param.memHook.cast()).cast();
    *dstSize = decOut.destOutFillLen.cast();
    return ret.cast();
}