typedef void (*RapidlzLogFunc)(const char *message, size_t size);

typedef struct TagRapidlzStreamCtx RapidlzStreamCtx;

struct TagRapidlzStreamCtx
{
    uint32_t hashTable[RAPIDLZ_STREAM_HASH_SIZE];
    const uint8_t *dict;
    uint32_t dictSize;
    uint32_t currentOffset;
    int acceleration;
    RapidlzStreamCtx *strmCtxSpecific;
};

typedef struct
{
    uint16_t v;
} __attribute__((packed)) RapidlzUnalignU16;

typedef struct
{
    uint32_t v;
} __attribute__((packed)) RapidlzUnalignU32;

typedef struct
{
    uint64_t v;
} __attribute__((packed)) RapidlzUnalignU64;

static uint8_t g_overlapOffAddVal[] = {0, 1, 2, 2, 4, 3, 2, 1};

static RapidlzLogFunc g_rapidlzLogFunc = NULL;

typedef struct
{
    uint8_t *hashTable;
    uint8_t hashType;
    uint8_t hashBits;
    uint8_t step;
    uint8_t bufferLimit;
} RapidlzCCtx;

static const char *g_rapidlzVersion = "rapidlz 3.24.10.B201";

#define RAPIDLZ_STREAM_HASH_SIZE (1 << 12)

#define RAPIDLZ_MAX_BYTE_VALUE 255

#define RAPIDLZ_MAX_4BIT_VALUE 15

#define RAPIDLZ_MIN_MATCH 4

#define RAPIDLZ_HASH_TYPE_4 4

#define RAPIDLZ_HASH_TYPE_5 5

#define RAPIDLZ_STEP_FORWARD_BASE 6

#define RAPIDLZ_MAX_OFFSET 65535

#define RAPIDLZ_EIGHT_BYTE 8

#define RAPIDLZ_SIXTEEN_BYTE 16

#define RAPIDLZ_COPY_PROTECT_SIZE 16

#define RAPIDLZ_INPUT_INVALID (size_t)(-100)

#define RAPIDLZ_MALLOC_FAILED (size_t)(-99)

#define RAPIDLZ_DST_SIZE_SMALL (size_t)(-98)

#define RAPIDLZ_SECUREC_ERROR (size_t)(-97)

#define RAPIDLZ_FORMAT_INVALID (size_t)(-96)

#define LOG_BUF_SIZE 1024

#define RAPIDLZFILENAME (strrchr(__FILE__, '/') ? strrchr(__FILE__, '/') + 1 : __FILE__)

#define RAPIDLZ_MAX_INPUT_SIZE 0x7E000000U

#define RAPIDLZ_MAX_4BIT_MATCH 19

#define RAPIDLZ_ACCELERATION_MAX 10

#define RAPIDLZ_SRC_SIZE_THRESHOLD 65536

#define RAPIDLZ_LAST_LITERALS 6

#define RAPIDLZ_MIN_COMPRESS_SIZE 16

#define RAPIDLZ_MIN_HASH_BIT 6

#define RAPIDLZ_MAX_HASH_BIT 12

#define RAPIDLZ_LOG(error_code, fmt, args...)                                                                          \
    do                                                                                                                 \
    {                                                                                                                  \
        RapidlzLogWrite((size_t)(error_code), RAPIDLZFILENAME, __LINE__, fmt, ##args);                                 \
    } while (0)

#define RAPIDLZ_LIKELY(x) (__builtin_expect(!!(x), 1))

#define RAPIDLZ_UNLIKELY(x) (__builtin_expect(!!(x), 0))

#define RAPIDLZ_READ16BIT(ptr) (((const RapidlzUnalignU16 *)(ptr))->v)

#define RAPIDLZ_READ32BIT(ptr) (((const RapidlzUnalignU32 *)(ptr))->v)

#define RAPIDLZ_READ64BIT(ptr) (((const RapidlzUnalignU64 *)(ptr))->v)

#define RAPIDLZ_WRITE64BIT(ptr, val) (((RapidlzUnalignU64 *)(ptr))->v = (val))

#define RAPIDLZ_ASSERT(x) assert(x)

#define RAPIDLZ_EXPAND_FORWARD(srcBegin, matchBegin, srcCurr, srcAnchor)                                               \
    do                                                                                                                 \
    {                                                                                                                  \
        while ((srcBegin) < (matchBegin) && (srcCurr) > (srcAnchor) &&                                                 \
               RAPIDLZ_UNLIKELY((matchBegin)[-1] == (srcCurr)[-1]))                                                    \
        {                                                                                                              \
            (matchBegin)--;                                                                                            \
            (srcCurr)--;                                                                                               \
        }                                                                                                              \
    } while (0)

#define RAPIDLZ_READ_OPTIONAL_LENGTH(len, srcCurr, srcEnd, temp)                                                       \
    do                                                                                                                 \
    {                                                                                                                  \
        if (RAPIDLZ_LIKELY((srcCurr) < (srcEnd)))                                                                      \
        {                                                                                                              \
            (temp) = *(srcCurr)++;                                                                                     \
            (len) += (temp);                                                                                           \
        }                                                                                                              \
        while (((temp) == RAPIDLZ_MAX_BYTE_VALUE) && (srcCurr) < (srcEnd))                                             \
        {                                                                                                              \
            (temp) = *(srcCurr)++;                                                                                     \
            (len) += (temp);                                                                                           \
        }                                                                                                              \
    } while (0)

#define SAFE_COPY_MATCH(dstCurr, matchSrc, matchLength)                                                                \
    do                                                                                                                 \
    {                                                                                                                  \
        while ((matchLength)-- > 0)                                                                                    \
        {                                                                                                              \
            *(dstCurr)++ = *(matchSrc)++;                                                                              \
        }                                                                                                              \
    } while (0)

#define RAPIDLZ_COMPRESSBOUND(size) ((uint32_t)(size) > RAPIDLZ_MAX_INPUT_SIZE ? 0 : (size) + ((size) / 255) + 16)

static int RapidlzIsLE(void)
{
    int n = 1;
    return *(char *)(&n);
}

static uint16_t RapidlzReadLE16Bit(const void *addr)
{
    if (RapidlzIsLE() != 0)
    {
        return *(const uint16_t *)addr;
    }
    uint8_t tmp1 = ((const uint8_t *)addr)[0];
    uint8_t tmp2 = ((const uint8_t *)addr)[1];
    return (uint16_t)(tmp1 + (tmp2 << 8));
}

static uint8_t RapidlzCountTailZero64(uint64_t x)
{
    if (x == 0)
    {
        return 0;
    }
    uint64_t val = x;
    uint8_t num = 0;
    while ((val & 1) == 0)
    {
        val >>= 1;
        num++;
    }
    return num;
}

static uint8_t RapidlzCountLeadZero64(uint64_t x)
{
    if (x == 0)
    {
        return 0;
    }
    uint8_t num = 0;
    uint64_t val = x;
    while ((val & 0x8000000000000000ULL) == 0)
    {
        val <<= 1;
        num++;
    }
    return num;
}

static uint8_t RapidlzHighBit64(uint64_t x)
{
    RAPIDLZ_ASSERT(x != 0);
    uint8_t pos = 64;
    uint64_t value = x;
    if (value == 0)
    {
        return 0;
    }
    if ((value & 0xFFFFFFFF00000000) == 0)
    {
        value <<= 32;
        pos -= 32;
    }
    if ((value & 0xFFFF000000000000) == 0)
    {
        value <<= 16;
        pos -= 16;
    }
    if ((value & 0xFF00000000000000) == 0)
    {
        value <<= 8;
        pos -= 8;
    }
    if ((value & 0xF000000000000000) == 0)
    {
        value <<= 4;
        pos -= 4;
    }
    if ((value & 0xC000000000000000) == 0)
    {
        value <<= 2;
        pos -= 2;
    }
    if ((value & 0x8000000000000000) == 0)
    {
        value <<= 1;
        pos -= 1;
    }
    return pos - 1;
}

static void RapidlzWriteLE16(void *addr, uint16_t val)
{
    if (RapidlzIsLE() != 0)
    {
        *(uint16_t *)addr = val;
    }
    else
    {
        uint8_t *tmp = (uint8_t *)addr;
        tmp[0] = (uint8_t)val;
        tmp[1] = (uint8_t)(val >> 8);
    }
}

static void RapidlzCopy16Byte(void *dst, const void *src)
{
    RAPIDLZ_WRITE64BIT(dst, RAPIDLZ_READ64BIT(src));
    RAPIDLZ_WRITE64BIT((uint8_t *)dst + 8, RAPIDLZ_READ64BIT((uint8_t *)src + 8));
}

static void RapidlzCopy8Byte(void *dst, const void *src)
{
    RAPIDLZ_WRITE64BIT(dst, RAPIDLZ_READ64BIT(src));
}

static void RapidlzWildCopy16(const uint8_t *srcPtr, uint8_t *dstPtr, uint8_t *dstEnd)
{
    uint8_t *tmpDstPtr = dstPtr;
    const uint8_t *tmpSrcPtr = srcPtr;
    do
    {
        RapidlzCopy16Byte(tmpDstPtr, tmpSrcPtr);
        tmpDstPtr += 16;
        tmpSrcPtr += 16;
    } while (tmpDstPtr < dstEnd);
}

static void RapidlzCopyLiteralsFast(const uint8_t *src, uint8_t *dst, uint32_t length)
{
    if (RAPIDLZ_LIKELY(length <= RAPIDLZ_SIXTEEN_BYTE))
    {
        RapidlzCopy16Byte(dst, src);
        return;
    }
    RapidlzWildCopy16(src, dst, dst + length);
}

static const uint8_t *RapidlzCompressExpandBackward(const uint8_t *matchLimit, const uint8_t *matchPtr,
                                                    const uint8_t *srcCurr)
{
    uint64_t xorVal;
    const uint8_t *loopEnd = matchLimit - 7;
    const uint8_t *srcCurrMatchEnd = srcCurr;
    const uint8_t *matchBegin = matchPtr;
    while (srcCurrMatchEnd < loopEnd)
    {
        xorVal = RAPIDLZ_READ64BIT(matchBegin) ^ RAPIDLZ_READ64BIT(srcCurrMatchEnd);
        if (RAPIDLZ_UNLIKELY(xorVal == 0))
        {
            srcCurrMatchEnd += sizeof(uint64_t);
            matchBegin += sizeof(uint64_t);
            continue;
        }
        srcCurrMatchEnd +=
            RapidlzIsLE() ? (RapidlzCountTailZero64(xorVal) >> 3) : (RapidlzCountLeadZero64(xorVal) >> 3);
        return srcCurrMatchEnd;
    }
    if (((srcCurrMatchEnd + 3) < matchLimit) && (RAPIDLZ_READ32BIT(srcCurrMatchEnd) == RAPIDLZ_READ32BIT(matchBegin)))
    {
        srcCurrMatchEnd += sizeof(uint32_t);
        matchBegin += sizeof(uint32_t);
    }
    if (((srcCurrMatchEnd + 1) < matchLimit) && (RAPIDLZ_READ16BIT(srcCurrMatchEnd) == RAPIDLZ_READ16BIT(matchBegin)))
    {
        srcCurrMatchEnd += sizeof(uint16_t);
        matchBegin += sizeof(uint16_t);
    }
    if ((srcCurrMatchEnd < matchLimit) && (srcCurrMatchEnd[0] == matchBegin[0]))
    {
        srcCurrMatchEnd++;
    }
    return srcCurrMatchEnd;
}

static void RapidlzCopyMatchFast(uint8_t *dst, uint8_t *match, uint16_t offset, uint32_t length)
{
    uint8_t *dstCurr = dst;
    uint8_t *matchPtr = match;
    if (offset >= RAPIDLZ_SIXTEEN_BYTE)
    {
        RapidlzCopyLiteralsFast(matchPtr, dstCurr, length);
        return;
    }
    for (int i = 0; i < RAPIDLZ_EIGHT_BYTE; ++i)
    {
        dstCurr[i] = matchPtr[i];
    }
    if (length <= RAPIDLZ_EIGHT_BYTE)
    {
        return;
    }
    uint8_t *dstEnd = dstCurr + length;
    if (offset < RAPIDLZ_EIGHT_BYTE)
    {
        matchPtr += g_overlapOffAddVal[offset];
        dstCurr += RAPIDLZ_EIGHT_BYTE;
    }
    do
    {
        RapidlzCopy8Byte(dstCurr, matchPtr);
        dstCurr += RAPIDLZ_EIGHT_BYTE;
        matchPtr += RAPIDLZ_EIGHT_BYTE;
    } while (dstCurr < dstEnd);
}

void RapidlzLogWrite(size_t error_code, const char *file_name, unsigned short line, const char *fmt, ...)
{
    va_list alist;
    char output[LOG_BUF_SIZE];
    int retVal, len;
    RapidlzLogFunc func = g_rapidlzLogFunc;
    char *filename;
    if (func == NULL)
    {
        return;
    }
    filename = strdup(file_name);
    if (filename == NULL)
    {
        return;
    }
    retVal = snprintf_s(output, LOG_BUF_SIZE, LOG_BUF_SIZE - 1, "\n[Rapidlz-Log] File=%s, Line=%u, Error=%zu\n",
                        basename(filename), line, error_code);
    if (retVal < 0)
    {
        free(filename);
        return;
    }
    len = retVal;
    free(filename);
    va_start(alist, fmt);
    retVal = vsnprintf_s(output + len, LOG_BUF_SIZE - len, LOG_BUF_SIZE - len - 1, fmt, alist);
    va_end(alist);
    if (retVal < 0)
    {
        return;
    }
    func(output, strlen(output) + 1);
}

void RapidlzLogRegister(RapidlzLogFunc func)
{
    g_rapidlzLogFunc = func;
}

const char *RapidlzVersionGet(void)
{
    return g_rapidlzVersion;
}

size_t RapidlzCompressBound(size_t srcSize)
{
    return RAPIDLZ_COMPRESSBOUND(srcSize);
}

static void RapidlzPutPosOnTable(uint32_t pos, uint32_t hashValue, uint8_t *hashTable, uint8_t hashType)
{
    if (hashType == 4)
    {
        *(((uint16_t *)hashTable) + hashValue) = (uint16_t)pos;
    }
    else if (hashType == 5)
    {
        *(((uint32_t *)hashTable) + hashValue) = (uint32_t)pos;
    }
}

static uint32_t RapidlzGetPosOnTable(uint32_t hashValue, uint8_t *hashTable, uint8_t hashType)
{
    if (hashType == 4)
    {
        return (uint32_t)(*(((uint16_t *)hashTable) + hashValue));
    }
    else if (hashType == 5)
    {
        return (*(((uint32_t *)hashTable) + hashValue));
    }
    return 0;
}

static uint32_t RapidlzCalcHashValue(const uint8_t *srcCurr, uint8_t hashType, uint8_t hashBits)
{
    if (hashType == 5)
    {
        return (uint32_t)((((RAPIDLZ_READ64BIT(srcCurr)) << 24) * 11400714819323198485ULL) >> (64 - hashBits));
    }
    else
    {
        return (RAPIDLZ_READ32BIT(srcCurr) * 2654435769U) >> (32 - hashBits);
    }
}

static uint8_t *RapidlzCompressStoreOptionalLength(uint8_t *dst, uint32_t litLength)
{
    uint8_t *dstCurr = dst;
    uint32_t length = litLength;
    if (length < RAPIDLZ_MAX_BYTE_VALUE)
    {
        *dstCurr = (uint8_t)length;
        dstCurr++;
        return dstCurr;
    }
    do
    {
        *dstCurr = RAPIDLZ_MAX_BYTE_VALUE;
        dstCurr++;
        length -= RAPIDLZ_MAX_BYTE_VALUE;
    } while (length >= RAPIDLZ_MAX_BYTE_VALUE);
    *dstCurr = (uint8_t)length;
    dstCurr++;
    return dstCurr;
}

static uint8_t *RapidlzStoreLastLiterals(uint8_t *dst, uint8_t *dstEnd, const uint8_t *srcCurr, uint32_t litLength,
                                         uint8_t bufferLimit)
{
    uint8_t *dstCurr = dst;
    if (bufferLimit != 0)
    {
        const uint32_t litTokSize = 1 + litLength + (litLength / RAPIDLZ_MAX_BYTE_VALUE);
        if (dstCurr + litTokSize > dstEnd)
        {
            RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "dstEnd - dstCur:%zu litTokSize:%u\n", dstEnd - dstCurr, litTokSize);
            return NULL;
        }
    }
    uint8_t token = (uint8_t)(((litLength < RAPIDLZ_MAX_4BIT_VALUE) ? (litLength) : (RAPIDLZ_MAX_4BIT_VALUE)) << 4);
    *dstCurr = token;
    dstCurr++;
    if (litLength >= RAPIDLZ_MAX_4BIT_VALUE)
    {
        dstCurr = RapidlzCompressStoreOptionalLength(dstCurr, litLength - RAPIDLZ_MAX_4BIT_VALUE);
    }
    if (memcpy_s(dstCurr, dstEnd - dstCurr, srcCurr, litLength) != EOK)
    {
        RAPIDLZ_LOG(RAPIDLZ_SECUREC_ERROR, "dstEnd - dstCurr:%zu litLength%u\n", dstEnd - dstCurr, litLength);
        return NULL;
    }
    return dstCurr + litLength;
}

static uint8_t *RapidlzStoreOffMatch(uint8_t *dst, uint8_t *token, uint32_t matchLength, uint16_t offset)
{
    uint8_t *dstCurr = dst;
    RapidlzWriteLE16(dstCurr, offset);
    dstCurr += 2;
    if (matchLength >= RAPIDLZ_MAX_4BIT_VALUE)
    {
        uint32_t optionalLen = matchLength - RAPIDLZ_MAX_4BIT_VALUE;
        *token += RAPIDLZ_MAX_4BIT_VALUE;
        for (; optionalLen >= RAPIDLZ_MAX_BYTE_VALUE; optionalLen -= RAPIDLZ_MAX_BYTE_VALUE)
        {
            *dstCurr++ = RAPIDLZ_MAX_BYTE_VALUE;
        }
        *dstCurr++ = (uint8_t)optionalLen;
    }
    else
    {
        *token += (uint8_t)matchLength;
    }
    return dstCurr;
}

static uint8_t *RapidlzStoreSequence(uint8_t *dst, const uint8_t *srcAnchor, uint32_t literalLength,
                                     uint32_t matchLength, uint16_t offset)
{
    uint8_t *dstCurr = dst;
    uint8_t *token = dstCurr++;
    if (literalLength >= RAPIDLZ_MAX_4BIT_VALUE)
    {
        *token = (RAPIDLZ_MAX_4BIT_VALUE << 4);
        uint32_t optionalLen = literalLength - RAPIDLZ_MAX_4BIT_VALUE;
        for (; optionalLen >= RAPIDLZ_MAX_BYTE_VALUE; optionalLen -= RAPIDLZ_MAX_BYTE_VALUE)
        {
            *dstCurr++ = (uint8_t)RAPIDLZ_MAX_BYTE_VALUE;
        }
        *dstCurr++ = (uint8_t)optionalLen;
        RapidlzCopy16Byte(dstCurr, srcAnchor);
        if (literalLength > 16)
        {
            RapidlzWildCopy16(srcAnchor + 16, dstCurr + 16, dstCurr + literalLength);
        }
        dstCurr += literalLength;
    }
    else if (literalLength > 0)
    {
        *token = (uint8_t)(literalLength << 4);
        RapidlzCopy16Byte(dstCurr, srcAnchor);
        dstCurr += literalLength;
    }
    else
    {
        *token = 0;
    }
    return RapidlzStoreOffMatch(dstCurr, token, matchLength, offset);
}

static size_t RapidlzCompressProcess(void *dst, size_t dstSize, const void *src, size_t srcSize, RapidlzCCtx *cCtx)
{
    uint32_t hashValue, matchLength, literalLength;
    uint32_t step = 1;
    uint16_t offset;
    uint8_t *hashTable = cCtx->hashTable;
    const uint8_t *srcBegin = (const uint8_t *)src;
    const uint8_t *srcEnd = (const uint8_t *)src + srcSize;
    const uint8_t *srcCurr = srcBegin + 1;
    const uint8_t *srcCurrMatchEnd;
    const uint8_t *srcAnchor = srcBegin;
    const uint8_t *matchBegin;
    const uint8_t *matchLimit = srcEnd - RAPIDLZ_LAST_LITERALS;
    const uint8_t *srcLimit = srcEnd - RAPIDLZ_MIN_COMPRESS_SIZE;
    uint8_t *dstBegin = (uint8_t *)dst;
    uint8_t *dstEnd = (uint8_t *)dst + dstSize;
    uint8_t *dstCurr = dstBegin;
    uint8_t hashType = cCtx->hashType;
    uint8_t hashBits = cCtx->hashBits;
    uint32_t searchMatchNb = cCtx->step << RAPIDLZ_STEP_FORWARD_BASE;
    uint32_t searchMatchNbTmp = searchMatchNb;
    uint8_t bufferLimit = cCtx->bufferLimit;
    while (RAPIDLZ_LIKELY(srcCurr <= srcLimit))
    {
        for (;;)
        {
            hashValue = RapidlzCalcHashValue(srcCurr, hashType, hashBits);
            matchBegin = srcBegin + RapidlzGetPosOnTable(hashValue, hashTable, hashType);
            RapidlzPutPosOnTable(srcCurr - srcBegin, hashValue, hashTable, hashType);
            if ((RAPIDLZ_READ32BIT(srcCurr) == RAPIDLZ_READ32BIT(matchBegin)) &&
                RAPIDLZ_LIKELY((srcCurr - matchBegin) <= RAPIDLZ_MAX_OFFSET))
            {
                break;
            }
            srcCurr += step;
            step = (searchMatchNbTmp++ >> RAPIDLZ_STEP_FORWARD_BASE);
            if (srcCurr > srcLimit)
            {
                dstCurr = RapidlzStoreLastLiterals(dstCurr, dstEnd, srcAnchor, srcEnd - srcAnchor, bufferLimit);
                if (dstCurr == NULL)
                {
                    return 0;
                }
                return dstCurr - dstBegin;
            }
        }
        step = 1;
        searchMatchNbTmp = searchMatchNb;
        srcCurrMatchEnd =
            RapidlzCompressExpandBackward(matchLimit, matchBegin + RAPIDLZ_MIN_MATCH, srcCurr + RAPIDLZ_MIN_MATCH);
        RAPIDLZ_EXPAND_FORWARD(srcBegin, matchBegin, srcCurr, srcAnchor);
        matchLength = srcCurrMatchEnd - srcCurr - RAPIDLZ_MIN_MATCH;
        offset = (uint16_t)(srcCurr - matchBegin);
        literalLength = srcCurr - srcAnchor;
        if (bufferLimit != 0)
        {
            uint32_t writeSize = literalLength + 8 + (literalLength + matchLength / RAPIDLZ_MAX_BYTE_VALUE);
            if (RAPIDLZ_UNLIKELY(dstCurr + writeSize > dstEnd))
            {
                RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "dstEnd - dstCur:%zu writeSize:%u\n", dstEnd - dstCurr, writeSize);
                return 0;
            }
        }
        dstCurr = RapidlzStoreSequence(dstCurr, srcAnchor, literalLength, matchLength, offset);
        srcCurr = srcCurrMatchEnd;
        srcAnchor = srcCurr;
        hashValue = RapidlzCalcHashValue(srcCurr - 2, hashType, hashBits);
        RapidlzPutPosOnTable(srcCurr - 2 - srcBegin, hashValue, hashTable, hashType);
        if (RAPIDLZ_UNLIKELY(srcCurr > srcLimit))
        {
            break;
        }
        hashValue = RapidlzCalcHashValue(srcCurr, hashType, hashBits);
        matchBegin = srcBegin + RapidlzGetPosOnTable(hashValue, hashTable, hashType);
        RapidlzPutPosOnTable(srcCurr - srcBegin, hashValue, hashTable, hashType);
        if ((RAPIDLZ_READ32BIT(srcCurr) != RAPIDLZ_READ32BIT(matchBegin)) ||
            RAPIDLZ_UNLIKELY((srcCurr - matchBegin) > RAPIDLZ_MAX_OFFSET))
        {
            srcCurr++;
            continue;
        }
        srcCurrMatchEnd =
            RapidlzCompressExpandBackward(matchLimit, matchBegin + RAPIDLZ_MIN_MATCH, srcCurr + RAPIDLZ_MIN_MATCH);
        matchLength = srcCurrMatchEnd - srcCurr - RAPIDLZ_MIN_MATCH;
        offset = (uint16_t)(srcCurr - matchBegin);
        if (bufferLimit != 0)
        {
            const uint32_t writeSize = 8 + matchLength / RAPIDLZ_MAX_BYTE_VALUE;
            if (RAPIDLZ_UNLIKELY(dstCurr + writeSize > dstEnd))
            {
                RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "dstEnd - dstCur:%zu writeSize:%u\n", dstEnd - dstCurr, writeSize);
                return 0;
            }
        }
        *dstCurr = 0;
        dstCurr = RapidlzStoreOffMatch(dstCurr + 1, dstCurr, matchLength, offset);
        srcCurr = srcCurrMatchEnd;
        srcAnchor = srcCurr;
        hashValue = RapidlzCalcHashValue(srcCurr - 2, hashType, hashBits);
        RapidlzPutPosOnTable(srcCurr - 2 - srcBegin, hashValue, hashTable, hashType);
    }
    if (srcAnchor < srcEnd)
    {
        dstCurr = RapidlzStoreLastLiterals(dstCurr, dstEnd, srcAnchor, srcEnd - srcAnchor, bufferLimit);
        if (dstCurr == NULL)
        {
            return 0;
        }
    }
    return dstCurr - dstBegin;
}

static void RapidlzCCtxFree(RapidlzCCtx *cCtx)
{
    if (cCtx != NULL)
    {
        if (cCtx->hashTable != NULL)
        {
            free(cCtx->hashTable);
            cCtx->hashTable = NULL;
        }
        free(cCtx);
    }
}

size_t RapidlzCompress(const void *src, void *dst, size_t srcSize, size_t dstSize, int acceleration)
{
    if (src == NULL || dst == NULL || srcSize == 0 || dstSize == 0)
    {
        RAPIDLZ_LOG(RAPIDLZ_INPUT_INVALID, "input invalid\n");
        return 0;
    }
    if (acceleration < 1 || acceleration > RAPIDLZ_ACCELERATION_MAX)
    {
        RAPIDLZ_LOG(RAPIDLZ_INPUT_INVALID, "acceleration:%d\n", acceleration);
        return 0;
    }
    RapidlzCCtx *cCtx = (RapidlzCCtx *)malloc(sizeof(RapidlzCCtx));
    if (cCtx == NULL)
    {
        RAPIDLZ_LOG(RAPIDLZ_MALLOC_FAILED, "cCtx malloc failed\n");
        return 0;
    }
    cCtx->hashBits = RAPIDLZ_MIN_HASH_BIT;
    size_t totalHashSize;
    if (srcSize <= RAPIDLZ_SRC_SIZE_THRESHOLD)
    {
        cCtx->hashType = RAPIDLZ_HASH_TYPE_4;
        if (srcSize >= 64)
        {
            cCtx->hashBits = (RapidlzHighBit64(srcSize) > RAPIDLZ_MAX_HASH_BIT) ? (RAPIDLZ_MAX_HASH_BIT + 1)
                                                                                : RapidlzHighBit64(srcSize);
        }
        totalHashSize = sizeof(uint16_t) * (uint32_t)(1 << cCtx->hashBits);
    }
    else
    {
        cCtx->hashType = RAPIDLZ_HASH_TYPE_5;
        cCtx->hashBits = RAPIDLZ_MAX_HASH_BIT;
        totalHashSize = sizeof(uint32_t) * (uint32_t)(1 << cCtx->hashBits);
    }
    uint8_t *table = (uint8_t *)malloc(totalHashSize);
    if (table == NULL)
    {
        RAPIDLZ_LOG(RAPIDLZ_MALLOC_FAILED, "hash table malloc failed\n");
        free(cCtx);
        return 0;
    }
    (void)memset_s(table, totalHashSize, 0, totalHashSize);
    cCtx->hashTable = table;
    cCtx->step = (uint8_t)acceleration;
    cCtx->bufferLimit = dstSize < RapidlzCompressBound(srcSize);
    size_t cSize = RapidlzCompressProcess(dst, dstSize, src, srcSize, cCtx);
    RapidlzCCtxFree(cCtx);
    return cSize;
}

size_t RapidlzCompressDefault(const void *src, void *dst, size_t srcSize, size_t dstSize)
{
    return RapidlzCompress(src, dst, srcSize, dstSize, 1);
}

size_t RapidlzDecompress(const void *src, void *dst, size_t srcSize, size_t dstSize)
{
    if (src == NULL || dst == NULL || srcSize == 0 || dstSize == 0)
    {
        RAPIDLZ_LOG(RAPIDLZ_INPUT_INVALID, "input invalid\n");
        return 0;
    }
    uint8_t token, temp = 0;
    register uint16_t offset;
    register uint32_t litLen, matchLen;
    uint8_t *matchSrc;
    const uint8_t *srcEnd = (const uint8_t *)src + srcSize;
    const uint8_t *srcCurr = (const uint8_t *)src;
    const uint8_t *srcEndFast = srcEnd - RAPIDLZ_COPY_PROTECT_SIZE;
    uint8_t *dstEnd = (uint8_t *)dst + dstSize;
    uint8_t *dstCurr = (uint8_t *)dst;
    uint8_t *dstEndFast = dstEnd - RAPIDLZ_COPY_PROTECT_SIZE;
    while (srcCurr < srcEnd)
    {
        token = *srcCurr++;
        litLen = (token >> 4);
        if (RAPIDLZ_LIKELY(litLen < RAPIDLZ_MAX_4BIT_VALUE))
        {
            if (RAPIDLZ_LIKELY(srcCurr + litLen <= srcEndFast && dstCurr + litLen <= dstEndFast))
            {
                RapidlzCopy16Byte(dstCurr, srcCurr);
                dstCurr += litLen;
                srcCurr += litLen;
                goto READ_MATCH;
            }
        }
        else
        {
            RAPIDLZ_READ_OPTIONAL_LENGTH(litLen, srcCurr, srcEnd, temp);
            if (RAPIDLZ_LIKELY(srcCurr + litLen <= srcEndFast && dstCurr + litLen <= dstEndFast))
            {
                RapidlzWildCopy16(srcCurr, dstCurr, dstCurr + litLen);
                dstCurr += litLen;
                srcCurr += litLen;
                goto READ_MATCH;
            }
        }
        size_t leftSrcSize = srcEnd - srcCurr;
        if (RAPIDLZ_UNLIKELY(litLen > leftSrcSize || memmove_s(dstCurr, dstEnd - dstCurr, srcCurr, litLen) != EOK))
        {
            RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "litLen:%u dstEnd - dst:%zu\n", litLen, leftSrcSize);
            return 0;
        }
        dstCurr += litLen;
        srcCurr += litLen;
        if (leftSrcSize == litLen)
        {
            return dstCurr - (uint8_t *)(dst);
        }
    READ_MATCH:
        if (RAPIDLZ_UNLIKELY(srcCurr > srcEnd - 2))
        {
            RAPIDLZ_LOG(RAPIDLZ_FORMAT_INVALID, "rapidlz format invalid\n");
            return 0;
        }
        offset = RapidlzReadLE16Bit(srcCurr);
        srcCurr += 2;
        matchSrc = dstCurr - offset;
        if (RAPIDLZ_UNLIKELY((void *)matchSrc < dst))
        {
            RAPIDLZ_LOG(RAPIDLZ_FORMAT_INVALID, "rapidlz format invalid\n");
            return 0;
        }
        matchLen = (token & RAPIDLZ_MAX_4BIT_VALUE) + RAPIDLZ_MIN_MATCH;
        if (matchLen == RAPIDLZ_MAX_4BIT_MATCH)
        {
            RAPIDLZ_READ_OPTIONAL_LENGTH(matchLen, srcCurr, srcEnd, temp);
        }
        if (RAPIDLZ_LIKELY(dstCurr + matchLen <= dstEndFast))
        {
            RapidlzCopyMatchFast(dstCurr, matchSrc, offset, matchLen);
            dstCurr += matchLen;
        }
        else
        {
            if (dstCurr + matchLen > dstEnd)
            {
                RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "dstEnd - dstCurr:%zu matchLen:%u\n", dstEnd - dstCurr, matchLen);
                return 0;
            }
            SAFE_COPY_MATCH(dstCurr, matchSrc, matchLen);
        }
    }
    return dstCurr - (uint8_t *)dst;
}

