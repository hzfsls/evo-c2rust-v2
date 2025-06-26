pub fn CmptLzFreeDict(mut decCtx: Ptr<CmptLzDecCtx>, mut memHook: Ptr<CmptLzMemHook>) {
    if (decCtx.dict != NULL!()) {
        CmptLzDecMemFree(memHook, CMPTLZ_DICT_HANDLE!(), decCtx.dict);
        decCtx.dict = NULL!();
    }
}