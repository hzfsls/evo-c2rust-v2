pub fn CmptLzDecAllocate(mut decCtx: Ptr<CmptLzDecCtx>, mut protData: Ptr<u8>, mut protSize: u32, mut memHook: Ptr<CmptLzMemHook>) -> i32 {
    let mut res: i32;
    let mut dictMask: u32;
    let mut dictBufSize: usize;
    let mut decProt: CmptLzDecProt = Default::default();

    if (decCtx == NULL!()) || (protData == NULL!()) || (memHook == NULL!()) {
        return CMPT_ERROR_UNSUPPORTED!();
    }

    res = CmptLzPropsDecode(protData, protSize.cast(), c_ref!(decProt));
    if (res != CMPT_OK!()) {
        return res;
    }
    res = CmptLzDecAllocateProbs(decCtx, c_ref!(decProt), memHook);
    if (res != CMPT_OK!()) {
        return res;
    }

    let mut dictSize: u32 = decProt.dicSize;
    if (dictSize >= (1 << CMPTLZ_BIG_DICT_LG_SIZE!()).cast::<u32>()) {
        dictMask = ((1 << CMPTLZ_MID_DICT_LG_SIZE!()) - 1).cast::<u32>();
    } else if (dictSize >= (1 << CMPTLZ_MID_DICT_LG_SIZE!()).cast::<u32>()) {
        dictMask = ((1 << CMPTLZ_SMALL_DICT_LG_SIZE!()) - 1).cast::<u32>();
    } else {
        dictMask = (CMPTLZ_DICT_MIN_LEN!() - 1).cast::<u32>();
    }

    dictBufSize = ((dictSize + dictMask) & !dictMask).cast::<usize>();
    if (dictBufSize < dictSize.cast::<usize>()) {
        dictBufSize = dictSize.cast();
    }

    if (decCtx.dict == NULL!()) {
        decCtx.dict = CmptLzDecMemAlloc(memHook, CMPTLZ_DICT_HANDLE!(), dictBufSize).cast::<Ptr<u8>>();
    } else {
        if (dictBufSize != decCtx.dictBufSize) {
            CmptLzFreeDict(decCtx, memHook);
            decCtx.dict = CmptLzDecMemAlloc(memHook, CMPTLZ_DICT_HANDLE!(), dictBufSize).cast::<Ptr<u8>>();
        }
    }

    if (decCtx.dict == NULL!()) {
        CmptLzDecFreeProbs(decCtx, memHook);
        return CMPT_ERROR_MEM!();
    }

    decCtx.dictBufSize = dictBufSize;
    decCtx.prop = decProt;

    return CMPT_OK!();
}
