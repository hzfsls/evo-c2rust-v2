pub fn RapidlzLoadDict(mut strmCtx: Ptr<RapidlzStreamCtx>, mut dictionary: Ptr<u8>, mut dictSize: i32) -> i32 {
    let mut dictStart: Ptr<u8> = dictionary.cast::<Ptr<u8>>();
    let mut dictEnd: Ptr<u8> = (dictionary + dictSize).cast::<Ptr<u8>>();
    RAPIDLZ_RETURN_IF_NOT_EOK!(c_memset_s!(strmCtx, c_sizeof!(RapidlzStreamCtx), 0, c_sizeof!(RapidlzStreamCtx)), RAPIDLZ_ENC_NOT_OK!());
    RAPIDLZ_RETURN_IF_NOT_TRUE!(!(dictSize < RAPIDLZ_STREAM_HASH_BYTES!()), RAPIDLZ_ENC_NOT_OK!());

    if (dictSize > RAPIDLZ_MAX_DICT_SIZE!()) {
        dictStart = (dictEnd - RAPIDLZ_MAX_DICT_SIZE!());
    }

    strmCtx.dict = dictStart;
    strmCtx.dictSize = (dictEnd - dictStart).cast();
    strmCtx.currentOffset = RAPIDLZ_MAX_DICT_SIZE!();

    let mut index32: u32 = (strmCtx.currentOffset - strmCtx.dictSize);

    let mut curDict: Ptr<u8> = dictStart;
    while (curDict <= dictEnd - RAPIDLZ_STREAM_HASH_BYTES!()) {
        let mut hashValue: u32 = RapidlzHash4CalcValue(curDict);
        RapidlzHash4PutPos(index32, hashValue, strmCtx.hashTable.cast());
        curDict += RAPIDLZ_DICT_HASH_MOVE_BYTES!();
        index32 += RAPIDLZ_DICT_HASH_MOVE_BYTES!();
    }

    return strmCtx.dictSize.cast::<i32>();
}
