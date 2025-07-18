pub fn CmptLzFreeDict(mut decCtx: Ptr<CmptLzDecCtx>, mut memHook: Ptr<CmptLzMemHook>) {
    if (decCtx.dict != NULL!()).as_bool() {
        CmptLzDecMemFree(memHook.cast(), CMPTLZ_DICT_HANDLE!(), decCtx.dict.cast());
        decCtx.dict = NULL!();
    }
}