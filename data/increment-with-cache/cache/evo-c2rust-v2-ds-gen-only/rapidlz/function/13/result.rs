pub fn RapidlzStrmCtxNorm(mut strmCtx: Ptr<RapidlzStreamCtx>, mut src: Ptr<u8>, mut srcSize: i32, mut dictEnd: Ptr<u8>) {
    RapidlzDictSizeNorm(strmCtx.cast(), src.cast(), dictEnd.cast());
    strmCtx.acceleration = if strmCtx.acceleration < RAPIDLZ_STREAM_ACCELERAT_MIN!() {
        RAPIDLZ_STREAM_ACCELERAT_MIN!()
    } else if strmCtx.acceleration > RAPIDLZ_STREAM_ACCELERAT_MAX!() {
        RAPIDLZ_STREAM_ACCELERAT_MAX!()
    } else {
        strmCtx.acceleration
    };
    if (strmCtx.currentOffset + srcSize.cast::<u32>() > RAPIDLZ_PTR_DIFF_MAX_32!()).as_bool() {
        let mut delta: u32 = strmCtx.currentOffset - RAPIDLZ_MAX_DICT_SIZE!();
        let mut i: i32 = 0;
        c_for!(; i < RAPIDLZ_STREAM_HASH_SIZE!(); i.suffix_plus_plus(); {
            if (strmCtx.hashTable[i] < delta).as_bool() {
                strmCtx.hashTable[i] = 0;
            } else {
                strmCtx.hashTable[i] -= delta;
            }
        });
        strmCtx.currentOffset = RAPIDLZ_MAX_DICT_SIZE!();
    }
    let mut srcEnd: Ptr<u8> = (src + srcSize).cast();
    if (srcEnd > strmCtx.dict.cast::<Ptr<u8>>()).as_bool() && (srcEnd < dictEnd).as_bool() {
        strmCtx.dictSize = (dictEnd - srcEnd).cast::<u32>();
        strmCtx.dictSize = if strmCtx.dictSize > RAPIDLZ_MAX_DICT_SIZE!() {
            RAPIDLZ_MAX_DICT_SIZE!()
        } else if strmCtx.dictSize < RAPIDLZ_STREAM_HASH_BYTES!() {
            0
        } else {
            strmCtx.dictSize
        };
        strmCtx.dict = (dictEnd - strmCtx.dictSize).cast::<Ptr<u8>>();
    }
}
