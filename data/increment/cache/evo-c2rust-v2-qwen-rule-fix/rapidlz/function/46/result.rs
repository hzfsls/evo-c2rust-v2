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
    let mut hashTable: Ptr<u8> = cCtx.hashTable;
    let mut srcBegin: Ptr<u8> = src.cast::<Ptr<u8>>();
    let mut srcEnd: Ptr<u8> = (src.cast::<Ptr<u8>>() + srcSize);
    let mut srcCurr: Ptr<u8> = (srcBegin + 1);
    let mut srcCurrMatchEnd: Ptr<u8> = Default::default();
    let mut srcAnchor: Ptr<u8> = srcBegin;
    let mut matchBegin: Ptr<u8> = Default::default();
    let mut matchLimit: Ptr<u8> = (srcEnd - RAPIDLZ_LAST_LITERALS!());
    let mut srcLimit: Ptr<u8> = (srcEnd - RAPIDLZ_MIN_COMPRESS_SIZE!());
    let mut dstBegin: Ptr<u8> = dst.cast::<Ptr<u8>>();
    let mut dstEnd: Ptr<u8> = (dst.cast::<Ptr<u8>>() + dstSize);
    let mut dstCurr: Ptr<u8> = dstBegin;
    let mut hashType: u8 = cCtx.hashType;
    let mut hashBits: u8 = cCtx.hashBits;
    let mut searchMatchNb: u32 = (cCtx.step << RAPIDLZ_STEP_FORWARD_BASE!()).cast();
    let mut searchMatchNbTmp: u32 = searchMatchNb;
    let mut bufferLimit: u8 = cCtx.bufferLimit;

    while (srcCurr <= srcLimit) {
        loop {
            hashValue = RapidlzCalcHashValue(srcCurr, hashType, hashBits);
            matchBegin = (srcBegin + RapidlzGetPosOnTable(hashValue, hashTable, hashType));
            RapidlzPutPosOnTable((srcCurr - srcBegin).cast(), hashValue, hashTable, hashType);

            if (RAPIDLZ_READ32BIT!(srcCurr).cast::<u32>() == RAPIDLZ_READ32BIT!(matchBegin).cast::<u32>()) && (srcCurr - matchBegin <= RAPIDLZ_MAX_OFFSET!()) {
                break;
            }

            srcCurr += step;
            step = (searchMatchNbTmp >> RAPIDLZ_STEP_FORWARD_BASE!());

            if (srcCurr > srcLimit) {
                dstCurr = RapidlzStoreLastLiterals(dstCurr, dstEnd, srcAnchor, (srcEnd - srcAnchor).cast(), bufferLimit);
                if (dstCurr == NULL!()) {
                    return 0;
                }
                return (dstCurr - dstBegin);
            }
        }
        step = 1;
        searchMatchNbTmp = searchMatchNb;

        srcCurrMatchEnd = RapidlzCompressExpandBackward(matchLimit, (matchBegin + RAPIDLZ_MIN_MATCH!()), (srcCurr + RAPIDLZ_MIN_MATCH!()));
        RAPIDLZ_EXPAND_FORWARD!(srcBegin, matchBegin, srcCurr, srcAnchor);
        matchLength = (srcCurrMatchEnd - srcCurr - RAPIDLZ_MIN_MATCH!()).cast();
        offset = (srcCurr - matchBegin).cast::<u16>();
        literalLength = (srcCurr - srcAnchor).cast();
        if (bufferLimit != 0) {
            let mut writeSize: u32 = (literalLength + 8 + (literalLength + matchLength / RAPIDLZ_MAX_BYTE_VALUE!()));
            if (RAPIDLZ_UNLIKELY!(dstCurr + writeSize > dstEnd)) {
                RAPIDLZ_LOG!(RAPIDLZ_DST_SIZE_SMALL!(), cstr!("dstEnd - dstCur:{} writeSize:{}\n"), (dstEnd - dstCurr), writeSize);
                return 0;
            }
        }
        dstCurr = RapidlzStoreSequence(dstCurr, srcAnchor, literalLength, matchLength, offset);
        srcCurr = srcCurrMatchEnd;
        srcAnchor = srcCurr;
        hashValue = RapidlzCalcHashValue((srcCurr - 2), hashType, hashBits);
        RapidlzPutPosOnTable((srcCurr - 2 - srcBegin).cast(), hashValue, hashTable, hashType);

        if (RAPIDLZ_UNLIKELY!(srcCurr > srcLimit)) {
            break;
        }

        hashValue = RapidlzCalcHashValue(srcCurr, hashType, hashBits);
        matchBegin = (srcBegin + RapidlzGetPosOnTable(hashValue, hashTable, hashType));
        RapidlzPutPosOnTable((srcCurr - srcBegin).cast(), hashValue, hashTable, hashType);

        if (RAPIDLZ_READ32BIT!(srcCurr).cast::<u32>() != RAPIDLZ_READ32BIT!(matchBegin).cast::<u32>()) || RAPIDLZ_UNLIKELY!((srcCurr - matchBegin) > RAPIDLZ_MAX_OFFSET!()) {
            srcCurr += 1;
            continue;
        }

        srcCurrMatchEnd = RapidlzCompressExpandBackward(matchLimit, (matchBegin + RAPIDLZ_MIN_MATCH!()), (srcCurr + RAPIDLZ_MIN_MATCH!()));
        matchLength = (srcCurrMatchEnd - srcCurr - RAPIDLZ_MIN_MATCH!()).cast();
        offset = (srcCurr - matchBegin).cast::<u16>();
        if (bufferLimit != 0) {
            let mut writeSize: u32 = (8 + matchLength / RAPIDLZ_MAX_BYTE_VALUE!());
            if (RAPIDLZ_UNLIKELY!(dstCurr + writeSize > dstEnd)) {
                RAPIDLZ_LOG!(RAPIDLZ_DST_SIZE_SMALL!(), cstr!("dstEnd - dstCur:{} writeSize:{}\n"), (dstEnd - dstCurr), writeSize);
                return 0;
            }
        }
        *dstCurr = 0;
        dstCurr = RapidlzStoreOffMatch((dstCurr + 1), dstCurr, matchLength, offset);

        srcCurr = srcCurrMatchEnd;
        srcAnchor = srcCurr;

        hashValue = RapidlzCalcHashValue((srcCurr - 2), hashType, hashBits);
        RapidlzPutPosOnTable((srcCurr - 2 - srcBegin).cast(), hashValue, hashTable, hashType);
    }

    if (srcAnchor < srcEnd) {
        dstCurr = RapidlzStoreLastLiterals(dstCurr, dstEnd, srcAnchor, (srcEnd - srcAnchor).cast(), bufferLimit);
        if (dstCurr == NULL!()) {
            return 0;
        }
    }

    return (dstCurr - dstBegin);
}