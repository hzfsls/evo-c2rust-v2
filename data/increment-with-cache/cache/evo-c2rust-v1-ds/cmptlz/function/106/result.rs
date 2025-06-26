pub fn CmptlzDecompress(mut src: Ptr<Void>, mut srcSize: usize, mut dst: Ptr<Void>, mut dstSize: Ptr<usize>, mut param: Ptr<CmptlzDecParam>) -> i32 {
    if src == NULL!() || dst == NULL!() || dstSize == NULL!() {
        CMPTLZ_LOG!(CMPT_ERROR_UNSUPPORTED!(), cstr!("The input parameter NULL is incorrect."));
        return CMPT_ERROR_UNSUPPORTED!();
    }

    if srcSize > 0x7fffffff || *dstSize > 0x7fffffff {
        CMPTLZ_LOG!(CMPT_ERROR_UNSUPPORTED!(), cstr!("dstSize:0x{} srcSize:0x{}"), *dstSize, srcSize);
        return CMPT_ERROR_UNSUPPORTED!();
    }

    if param == NULL!() || param.memHook == NULL!() || param.protData == NULL!() || param.protSize != CMPTLZ_PROPS_SIZE!() {
        CMPTLZ_LOG!(CMPT_ERROR_UNSUPPORTED!(), cstr!("The compress param NULL is incorrect."));
        return CMPT_ERROR_UNSUPPORTED!();
    }

    let mut decIn: CmptLzDecIn = CmptLzDecIn { pSrcIn: src, strInLen: srcSize, strInCostLen: 0 };
    let mut decOut: CmptLzDecOut = CmptLzDecOut { pDestOut: dst, destOutLen: *dstSize, destOutFillLen: 0 };
    let mut enFinStat: EnCmptLzStatus = CMPTLZ_STATUS_BUT!();
    let mut ret: i32 = CmptLzDecode(c_ref!(decIn).cast(), c_ref!(decOut).cast(), param.protData.cast(), CMPTLZ_FINISH_ANY!(), c_ref!(enFinStat).cast(), param.memHook.cast()).cast();

    *dstSize = decOut.destOutFillLen.cast();
    return ret.cast();
}
