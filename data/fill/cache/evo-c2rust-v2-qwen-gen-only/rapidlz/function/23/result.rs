pub fn RapidlzCompressProcess(
    mut dst: Ptr<Void>,
    mut dstSize: usize,
    mut src: Ptr<Void>,
    mut srcSize: usize,
    mut cCtx: Ptr<RapidlzCCtx>,
) -> usize {
    let mut hashValue: u32 = Default::default();
    let mut matchLength: u32 = Default::default();
    let mut literalLength: u32 = Default::default();
    let mut step: u32 = 1;
    let mut offset: u16 = Default::default();
    let mut hashTable: Ptr<u8> = cCtx.hashTable.cast();
    let mut srcBegin: Ptr<u8> = src.cast::<Ptr<u8>>();
    let mut srcEnd: Ptr<u8> = (src.cast::<Ptr<u8>>() + srcSize).cast();
    let mut srcCurr: Ptr<u8> = srcBegin + 1;
    let mut srcCurrMatchEnd: Ptr<u8> = Default::default();
    let mut srcAnchor: Ptr<u8> = srcBegin.cast();
    let mut matchBegin: Ptr<u8> = Default::default();
    let mut matchLimit: Ptr<u8> = srcEnd - RAPIDLZ_LAST_LITERALS;
    let mut srcLimit: Ptr<u8> = srcEnd - RAPIDLZ_MIN_COMPRESS_SIZE;
    let mut dstBegin: Ptr<u8> = dst.cast::<Ptr<u8>>();
    let mut dstEnd: Ptr<u8> = (dst.cast::<Ptr<u8>>() + dstSize).cast();
    let mut dstCurr: Ptr<u8> = dstBegin.cast();
    let mut hashType: u8 = cCtx.hashType.cast();
    let mut hashBits: u8 = cCtx.hashBits.cast();
    let mut searchMatchNb: u32 = cCtx.step << RAPIDLZ_STEP_FORWARD_BASE;
    let mut searchMatchNbTmp: u32 = searchMatchNb;
    let mut bufferLimit: u8 = cCtx.bufferLimit.cast();

    while (RAPIDLZ_LIKELY!(srcCurr <= srcLimit).as_bool()) {
        loop {
            hashValue = RapidlzCalcHashValue(srcCurr.cast(), hashType.cast(), hashBits.cast());
            matchBegin = srcBegin + RapidlzGetPosOnTable(hashValue.cast(), hashTable.cast(), hashType.cast());
            RapidlzPutPosOnTable((srcCurr - srcBegin).cast(), hashValue.cast(), hashTable.cast(), hashType.cast());
            if (RAPIDLZ_READ32BIT!(srcCurr).cast::<u32>() == RAPIDLZ_READ32BIT!(matchBegin).cast::<u32>()) &&
                (RAPIDLZ_LIKELY!((srcCurr - matchBegin) <= RAPIDLZ_MAX_OFFSET).as_bool()) {
                break;
            }
            srcCurr += step;
            step = (searchMatchNbTmp >> RAPIDLZ_STEP_FORWARD_BASE).cast();
            if (srcCurr > srcLimit).as_bool() {
                dstCurr = RapidlzStoreLastLiterals(dstCurr.cast(), dstEnd.cast(), srcAnchor.cast(), (srcEnd - srcAnchor).cast(), bufferLimit.cast());
                if (dstCurr == NULL!()).as_bool() {
                    return 0;
                }
                return (dstCurr - dstBegin).cast();
            }
        }
        step = 1;
        searchMatchNbTmp = searchMatchNb;
        srcCurrMatchEnd = RapidlzCompressExpandBackward(matchLimit.cast(), (matchBegin + RAPIDLZ_MIN_MATCH).cast(), (srcCurr + RAPIDLZ_MIN_MATCH).cast());
        RAPIDLZ_EXPAND_FORWARD!(srcBegin.cast(), matchBegin.cast(), srcCurr.cast(), srcAnchor.cast());
        matchLength = (srcCurrMatchEnd - srcCurr - RAPIDLZ_MIN_MATCH).cast();
        offset = (srcCurr - matchBegin).cast::<u16>();
        literalLength = (srcCurr - srcAnchor).cast();
        if (bufferLimit != 0).as_bool() {
            let mut writeSize: u32 = literalLength + 8 + (literalLength + matchLength / RAPIDLZ_MAX_BYTE_VALUE).cast();
            if (RAPIDLZ_UNLIKELY!(dstCurr + writeSize > dstEnd).as_bool()) {
                RAPIDLZ_LOG!(RAPIDLZ_DST_SIZE_SMALL, cstr!("dstEnd - dstCur:{} writeSize:{}\n"), (dstEnd - dstCurr).cast(), writeSize.cast());
                return 0;
            }
        }
        dstCurr = RapidlzStoreSequence(dstCurr.cast(), srcAnchor.cast(), literalLength.cast(), matchLength.cast(), offset.cast());
        srcCurr = srcCurrMatchEnd.cast();
        srcAnchor = srcCurr.cast();
        hashValue = RapidlzCalcHashValue((srcCurr - 2).cast(), hashType.cast(), hashBits.cast());
        RapidlzPutPosOnTable((srcCurr - 2 - srcBegin).cast(), hashValue.cast(), hashTable.cast(), hashType.cast());
        if (RAPIDLZ_UNLIKELY!(srcCurr > srcLimit).as_bool()) {
            break;
        }
        hashValue = RapidlzCalcHashValue(srcCurr.cast(), hashType.cast(), hashBits.cast());
        matchBegin = srcBegin + RapidlzGetPosOnTable(hashValue.cast(), hashTable.cast(), hashType.cast());
        RapidlzPutPosOnTable((srcCurr - srcBegin).cast(), hashValue.cast(), hashTable.cast(), hashType.cast());
        if (RAPIDLZ_READ32BIT!(srcCurr).cast::<u32>() != RAPIDLZ_READ32BIT!(matchBegin).cast::<u32>()) ||
            (RAPIDLZ_UNLIKELY!((srcCurr - matchBegin) > RAPIDLZ_MAX_OFFSET).as_bool()) {
            srcCurr += 1;
            continue;
        }
        srcCurrMatchEnd = RapidlzCompressExpandBackward(matchLimit.cast(), (matchBegin + RAPIDLZ_MIN_MATCH).cast(), (srcCurr + RAPIDLZ_MIN_MATCH).cast());
        matchLength = (srcCurrMatchEnd - srcCurr - RAPIDLZ_MIN_MATCH).cast();
        offset = (srcCurr - matchBegin).cast::<u16>();
        if (bufferLimit != 0).as_bool() {
            let mut writeSize: u32 = 8 + matchLength / RAPIDLZ_MAX_BYTE_VALUE;
            if (RAPIDLZ_UNLIKELY!(dstCurr + writeSize > dstEnd).as_bool()) {
                RAPIDLZ_LOG!(RAPIDLZ_DST_SIZE_SMALL, cstr!("dstEnd - dstCur:{} writeSize:{}\n"), (dstEnd - dstCurr).cast(), writeSize.cast());
                return 0;
            }
        }
        *dstCurr = 0;
        dstCurr = RapidlzStoreOffMatch((dstCurr + 1).cast(), dstCurr.cast(), matchLength.cast(), offset.cast());
        srcCurr = srcCurrMatchEnd.cast();
        srcAnchor = srcCurr.cast();
        hashValue = RapidlzCalcHashValue((srcCurr - 2).cast(), hashType.cast(), hashBits.cast());
        RapidlzPutPosOnTable((srcCurr - 2 - srcBegin).cast(), hashValue.cast(), hashTable.cast(), hashType.cast());
    }
    if (srcAnchor < srcEnd).as_bool() {
        dstCurr = RapidlzStoreLastLiterals(dstCurr.cast(), dstEnd.cast(), srcAnchor.cast(), (srcEnd - srcAnchor).cast(), bufferLimit.cast());
        if (dstCurr == NULL!()).as_bool() {
            return 0;
        }
    }
    return (dstCurr - dstBegin).cast();
}