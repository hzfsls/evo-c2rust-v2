pub fn CmptLzDecAllocate(mut decCtx: Ptr<CmptLzDecCtx>, mut protData: Ptr<u8>, mut protSize: u32, mut memHook: Ptr<CmptLzMemHook>) -> i32 {
    let mut res: i32;
    let mut dictMask: u32;
    let mut dictBufSize: usize;
    let mut decProt: CmptLzDecProt = Default::default();

    if (decCtx == NULL!()).as_bool() || (protData == NULL!()).as_bool() || (memHook == NULL!()).as_bool() {
        return CMPT_ERROR_UNSUPPORTED!();
    }

    res = CmptLzPropsDecode(protData.cast(), protSize.cast(), c_ref!(decProt).cast()).cast();
    if (res != CMPT_OK!()).as_bool() {
        return res;
    }
    res = CmptLzDecAllocateProbs(decCtx.cast(), c_ref!(decProt).cast(), memHook.cast()).cast();
    if (res != CMPT_OK!()).as_bool() {
        return res;
    }

    let mut dictSize: u32 = decProt.dicSize.cast();
    if (dictSize >= (1 << CMPTLZ_BIG_DICT_LG_SIZE!()).cast::<u32>()).as_bool() {
        dictMask = ((1 << CMPTLZ_MID_DICT_LG_SIZE!()) - 1).cast::<u32>();
    } else if (dictSize >= (1 << CMPTLZ_MID_DICT_LG_SIZE!()).cast::<u32>()).as_bool() {
        dictMask = ((1 << CMPTLZ_SMALL_DICT_LG_SIZE!()) - 1).cast::<u32>();
    } else {
        dictMask = (CMPTLZ_DICT_MIN_LEN!() - 1).cast::<u32>();
    }

    dictBufSize = ((dictSize + dictMask) & !dictMask).cast::<usize>();
    if (dictBufSize < dictSize.cast::<usize>()).as_bool() {
        dictBufSize = dictSize.cast();
    }

    if (decCtx.dict == NULL!()).as_bool() {
        decCtx.dict = CmptLzDecMemAlloc(memHook.cast(), CMPTLZ_DICT_HANDLE!(), dictBufSize).cast::<Ptr<u8>>();
    } else {
        if (dictBufSize != decCtx.dictBufSize).as_bool() {
            CmptLzFreeDict(decCtx.cast(), memHook.cast());
            decCtx.dict = CmptLzDecMemAlloc(memHook.cast(), CMPTLZ_DICT_HANDLE!(), dictBufSize).cast::<Ptr<u8>>();
        }
    }

    if (decCtx.dict == NULL!()).as_bool() {
        CmptLzDecFreeProbs(decCtx.cast(), memHook.cast());
        return CMPT_ERROR_MEM!();
    }

    decCtx.dictBufSize = dictBufSize.cast();
    decCtx.prop = decProt.cast();

    return CMPT_OK!();
}
