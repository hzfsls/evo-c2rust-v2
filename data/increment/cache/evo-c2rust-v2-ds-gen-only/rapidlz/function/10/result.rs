pub fn RapidlzLoadDict(mut strmCtx: Ptr<RapidlzStreamCtx>, mut dictionary: Ptr<u8>, mut dictSize: i32) -> i32 {
    let mut dictStart: Ptr<u8> = dictionary.cast::<Ptr<u8>>();
    let mut dictEnd: Ptr<u8> = (dictionary + dictSize).cast::<Ptr<u8>>();
    RAPIDLZ_RETURN_IF_NOT_EOK!(c_memset_s!(strmCtx, c_sizeof!(RapidlzStreamCtx), 0, c_sizeof!(RapidlzStreamCtx)), RAPIDLZ_ENC_NOT_OK!());
    RAPIDLZ_RETURN_IF_NOT_TRUE!(!(dictSize < RAPIDLZ_STREAM_HASH_BYTES!()).as_bool(), RAPIDLZ_ENC_NOT_OK!());

    if (dictSize > RAPIDLZ_MAX_DICT_SIZE!()).as_bool() {
        dictStart = (dictEnd - RAPIDLZ_MAX_DICT_SIZE!()).cast();
    }

    strmCtx.dict = dictStart.cast();
    strmCtx.dictSize = (dictEnd - dictStart).cast();
    strmCtx.currentOffset = RAPIDLZ_MAX_DICT_SIZE!();

    let mut index32: u32 = (strmCtx.currentOffset - strmCtx.dictSize).cast();

    let mut curDict: Ptr<u8> = dictStart.cast();
    while (curDict <= dictEnd - RAPIDLZ_STREAM_HASH_BYTES!()).as_bool() {
        let mut hashValue: u32 = RapidlzHash4CalcValue(curDict.cast()).cast();
        RapidlzHash4PutPos(index32.cast(), hashValue.cast(), strmCtx.hashTable.cast());
        curDict += RAPIDLZ_DICT_HASH_MOVE_BYTES!();
        index32 += RAPIDLZ_DICT_HASH_MOVE_BYTES!();
    }

    return strmCtx.dictSize.cast::<i32>();
}
