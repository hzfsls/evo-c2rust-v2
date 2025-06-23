pub fn RapidlzCompress(mut src: Ptr<Void>, mut dst: Ptr<Void>, mut srcSize: usize, mut dstSize: usize, mut acceleration: i32) -> usize {
    if (src == NULL!()) || (dst == NULL!()) || (srcSize == 0) || (dstSize == 0) {
        RAPIDLZ_LOG!(RAPIDLZ_INPUT_INVALID!(), cstr!("input invalid\n"));
        return 0;
    }

    if (acceleration < 1) || (acceleration > RAPIDLZ_ACCELERATION_MAX!()) {
        RAPIDLZ_LOG!(RAPIDLZ_INPUT_INVALID!(), cstr!("acceleration:{}\n"), acceleration);
        return 0;
    }

    let mut cCtx: Ptr<RapidlzCCtx> = c_malloc!(c_sizeof!(RapidlzCCtx));
    if (cCtx == NULL!()) {
        RAPIDLZ_LOG!(RAPIDLZ_MALLOC_FAILED!(), cstr!("cCtx malloc failed\n"));
        return 0;
    }

    cCtx.hashBits = RAPIDLZ_MIN_HASH_BIT!();
    let mut totalHashSize: usize;
    if (srcSize <= RAPIDLZ_SRC_SIZE_THRESHOLD!()) {
        cCtx.hashType = RAPIDLZ_HASH_TYPE_4!();
        if (srcSize >= 64) {
            cCtx.hashBits = if (RapidlzHighBit64(srcSize) > RAPIDLZ_MAX_HASH_BIT!()) {
                RAPIDLZ_MAX_HASH_BIT!() + 1
            } else {
                RapidlzHighBit64(srcSize)
            };
        }
        totalHashSize = c_sizeof!(u16) * (1 << cCtx.hashBits).cast::<usize>();
    } else {
        cCtx.hashType = RAPIDLZ_HASH_TYPE_5!();
        cCtx.hashBits = RAPIDLZ_MAX_HASH_BIT!();
        totalHashSize = c_sizeof!(u32) * (1 << cCtx.hashBits).cast::<usize>();
    }

    let mut table: Ptr<u8> = c_malloc!(totalHashSize);
    if (table == NULL!()) {
        RAPIDLZ_LOG!(RAPIDLZ_MALLOC_FAILED!(), cstr!("hash table malloc failed\n"));
        c_free!(cCtx);
        return 0;
    }
    c_memset_s!(table, totalHashSize, 0, totalHashSize).cast::<Void>();
    cCtx.hashTable = table;
    cCtx.step = acceleration.cast();
    cCtx.bufferLimit = (dstSize < RapidlzCompressBound(srcSize));

    let mut cSize: usize = RapidlzCompressProcess(dst, dstSize, src, srcSize, cCtx);
    RapidlzCCtxFree(cCtx);
    return cSize;
}