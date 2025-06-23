pub fn RapidlzDictSizeNorm(mut strmCtx: Ptr<RapidlzStreamCtx>, mut src: Ptr<u8>, mut dictEnd: Ptr<u8>) {
    if (strmCtx.dictSize < RAPIDLZ_STREAM_HASH_BYTES!()).as_bool() {
        strmCtx.dictSize = 0;
        strmCtx.dict = src.cast::<Ptr<u8>>();
        dictEnd = src.cast();
    }
    if (strmCtx.dictSize > RAPIDLZ_MAX_DICT_SIZE!()).as_bool() {
        strmCtx.dict = (dictEnd - RAPIDLZ_MAX_DICT_SIZE!()).cast::<Ptr<u8>>();
        strmCtx.dictSize = RAPIDLZ_MAX_DICT_SIZE!();
    }
}
