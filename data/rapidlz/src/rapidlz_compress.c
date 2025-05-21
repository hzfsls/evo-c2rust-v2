/*
 * @file rapidlz_compress.c
 * Copyright (c) Huawei Technologies Co., Ltd. 2022-2022. All rights reserved.
 * Description: compress function description
 * Author: Anonym
 * Create: 2022-06-24
 */

#include <stddef.h>
#include "rapidlz_inner.h"
#include "securec.h"
#include "rapidlz.h"

#ifdef __cplusplus
extern "C" {
#endif

#define RAPIDLZ_MAX_INPUT_SIZE 0x7E000000U /* 最大输入流大小：2 113 929 216 bytes */
#define RAPIDLZ_COMPRESSBOUND(size) ((uint32_t)(size) > RAPIDLZ_MAX_INPUT_SIZE ? 0 : (size) + ((size) / 255) + 16)
#define RAPIDLZ_ACCELERATION_MAX 10    /* 每次最大压缩步长 */
#define RAPIDLZ_SRC_SIZE_THRESHOLD 65536       /* src大小来控制lz参数 */

#define RAPIDLZ_LAST_LITERALS 6                /* 开源是5，这里为了匹配后前2字节取8字节hash所以要预留8-2=6字节 */
#define RAPIDLZ_MIN_COMPRESS_SIZE 16           /* 开源是12，这里为了copy读src不越界扩大至16 */
#define RAPIDLZ_MIN_HASH_BIT 6                 /* 最小hash表的长度 = 2 ^ 6 */
#define RAPIDLZ_MAX_HASH_BIT 12                /* 最大hash表的长度 = 2 ^ 12 */


typedef struct {
    uint8_t *hashTable;   /* hash表数组 */
    uint8_t hashType;    /* 每次计算的字节长度，对于小输入流为4字节，对于大输入流为5字节 */
    uint8_t hashBits;    /* hash表的长度，根据输入流大小来确定 */
    uint8_t step;        /* 步长 */
    uint8_t bufferLimit; /* 判断在压缩时是否需要判断dst大小 */
} RapidlzCCtx;

static const char *g_rapidlzVersion = "rapidlz 3.24.10.B201";

const char *RapidlzVersionGet(void)
{
    return g_rapidlzVersion;
}

size_t RapidlzCompressBound(size_t srcSize)
{
    // 这里兼容lz4的bound，主要是为了某些文件dst不够用的情况
    return RAPIDLZ_COMPRESSBOUND(srcSize);
}

static RAPIDLZ_ALWAYS_INLINE void RapidlzPutPosOnTable(uint32_t pos, uint32_t hashValue, uint8_t *hashTable, uint8_t hashType)
{
    if (hashType == 4) { /* 表示对每4个字节做hash计算 */
        *(((uint16_t *)hashTable) + hashValue) = (uint16_t)pos;
    } else if (hashType == 5) {
        *(((uint32_t *)hashTable) + hashValue) = (uint32_t)pos;
    }
}

static RAPIDLZ_ALWAYS_INLINE uint32_t RapidlzGetPosOnTable(uint32_t hashValue, uint8_t *hashTable, uint8_t hashType)
{
    if (hashType == 4) { // 表示对每4个字节做hash计算
        return (uint32_t)(*(((uint16_t *)hashTable) + hashValue));
    } else if (hashType == 5) {
        return (*(((uint32_t *)hashTable) + hashValue));
    }

    return 0;
}

static RAPIDLZ_ALWAYS_INLINE uint32_t RapidlzCalcHashValue(const uint8_t *srcCurr, uint8_t hashType, uint8_t hashBits)
{
    if (hashType == 5) {  // 将获得的64bit的内容先左移24bit
        return (uint32_t)((((RAPIDLZ_READ64BIT(srcCurr)) << 24) * 11400714819323198485ULL) >> (64 - hashBits));
    } else { // 将获得的32bit的字段进行hash计算后取HASHBIT到32bit的内容
        return (RAPIDLZ_READ32BIT(srcCurr) * 2654435769U) >> (32 - hashBits);
    }
}

// 储存额外的长度（>15字节）
static RAPIDLZ_ALWAYS_INLINE uint8_t *RapidlzCompressStoreOptionalLength(uint8_t *dst, uint32_t litLength)
{
    uint8_t *dstCurr = dst;
    uint32_t length = litLength;

    if (length < RAPIDLZ_MAX_BYTE_VALUE) {
        *dstCurr = (uint8_t)length;
        dstCurr++;
        return dstCurr;
    }

    do {
        *dstCurr = RAPIDLZ_MAX_BYTE_VALUE;
        dstCurr++;
        length -= RAPIDLZ_MAX_BYTE_VALUE;
    } while (length >= RAPIDLZ_MAX_BYTE_VALUE);

    *dstCurr = (uint8_t)length;
    dstCurr++;
    return dstCurr;
}

static RAPIDLZ_ALWAYS_INLINE uint8_t *RapidlzStoreLastLiterals(uint8_t *dst, uint8_t *dstEnd, const uint8_t *srcCurr,
                                                               uint32_t litLength, uint8_t bufferLimit)
{
    uint8_t *dstCurr = dst;

    if (bufferLimit != 0) {
        const uint32_t litTokSize = 1 + litLength + (litLength / RAPIDLZ_MAX_BYTE_VALUE);
        if (dstCurr + litTokSize > dstEnd) {
            RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "dstEnd - dstCur:%zu litTokSize:%u\n", dstEnd - dstCurr, litTokSize);
            return NULL;
        }
    }

    // 此处只需要ll 高位4bit即可 ml为0
    uint8_t token = (uint8_t)(((litLength < RAPIDLZ_MAX_4BIT_VALUE) ? (litLength) : (RAPIDLZ_MAX_4BIT_VALUE)) << 4);
    *dstCurr = token;
    dstCurr++;

    if (litLength >= RAPIDLZ_MAX_4BIT_VALUE) {
        dstCurr = RapidlzCompressStoreOptionalLength(dstCurr, litLength - RAPIDLZ_MAX_4BIT_VALUE);
    }

    if (memcpy_s(dstCurr, dstEnd - dstCurr, srcCurr, litLength) != EOK) {
        RAPIDLZ_LOG(RAPIDLZ_SECUREC_ERROR, "dstEnd - dstCurr:%zu litLength%u\n", dstEnd - dstCurr, litLength);
        return NULL;
    }

    return dstCurr + litLength;
}

static RAPIDLZ_ALWAYS_INLINE uint8_t *RapidlzStoreOffMatch(uint8_t *dst, uint8_t *token, uint32_t matchLength,
                                                           uint16_t offset)
{
    uint8_t *dstCurr = dst;

    RapidlzWriteLE16(dstCurr, offset);
    dstCurr += 2; /* 指针移动2字节 */

    if (matchLength >= RAPIDLZ_MAX_4BIT_VALUE) {
        uint32_t optionalLen = matchLength - RAPIDLZ_MAX_4BIT_VALUE;
        *token += RAPIDLZ_MAX_4BIT_VALUE;
        for (; optionalLen >= RAPIDLZ_MAX_BYTE_VALUE; optionalLen -= RAPIDLZ_MAX_BYTE_VALUE) {
            *dstCurr++ = RAPIDLZ_MAX_BYTE_VALUE;
        }
        *dstCurr++ = (uint8_t)optionalLen;
    } else {
        *token += (uint8_t)matchLength;
    }

    return dstCurr;
}

// 将一个序列sequence输入到输出流中
static RAPIDLZ_ALWAYS_INLINE uint8_t *RapidlzStoreSequence(uint8_t *dst, const uint8_t *srcAnchor, uint32_t literalLength,
                                                           uint32_t matchLength, uint16_t offset)
{
    uint8_t *dstCurr = dst;
    uint8_t *token = dstCurr++;

    if (literalLength >= RAPIDLZ_MAX_4BIT_VALUE) {
        *token = (RAPIDLZ_MAX_4BIT_VALUE << 4); // 高4bit为literals
        uint32_t optionalLen = literalLength - RAPIDLZ_MAX_4BIT_VALUE;
        for (; optionalLen >= RAPIDLZ_MAX_BYTE_VALUE; optionalLen -= RAPIDLZ_MAX_BYTE_VALUE) {
            *dstCurr++ = (uint8_t)RAPIDLZ_MAX_BYTE_VALUE;
        }
        *dstCurr++ = (uint8_t)optionalLen;
        RapidlzCopy16Byte(dstCurr, srcAnchor);
        if (literalLength > 16) { // 这里一次拷贝16字节
            RapidlzWildCopy16(srcAnchor + 16, dstCurr + 16, dstCurr + literalLength);
        }
        dstCurr += literalLength;
    } else if (literalLength > 0) {
        *token = (uint8_t)(literalLength << 4); // 高4bit为literals
        RapidlzCopy16Byte(dstCurr, srcAnchor);
        dstCurr += literalLength;
    } else {
        *token = 0;
    }

    return RapidlzStoreOffMatch(dstCurr, token, matchLength, offset);
}

// 性能热点 不要轻易修改此函数
static size_t RapidlzCompressProcess(void *dst, size_t dstSize, const void *src, size_t srcSize, RapidlzCCtx *cCtx)
{
    uint32_t hashValue, matchLength, literalLength;
    uint32_t step = 1; // 步长加速查找
    uint16_t offset;
    uint8_t *hashTable = cCtx->hashTable;
    const uint8_t *srcBegin = (const uint8_t *)src;
    const uint8_t *srcEnd = (const uint8_t *)src + srcSize;
    const uint8_t *srcCurr = srcBegin + 1; // 当前扫描指针在输入流的位置,从第二个位置开始扫描
    const uint8_t *srcCurrMatchEnd;        // 当前扫描指针匹配成功的结束位置
    const uint8_t *srcAnchor = srcBegin;   // 这次扫描开始的初始位置，方便记录literal到dst中
    const uint8_t *matchBegin;             // 与扫描指针匹配成功的位置
    const uint8_t *matchLimit = srcEnd - RAPIDLZ_LAST_LITERALS;
    const uint8_t *srcLimit = srcEnd - RAPIDLZ_MIN_COMPRESS_SIZE; // 留有最少16字节的copy空间
    uint8_t *dstBegin = (uint8_t *)dst;
    uint8_t *dstEnd = (uint8_t *)dst + dstSize;
    uint8_t *dstCurr = dstBegin;
    uint8_t hashType = cCtx->hashType;
    uint8_t hashBits = cCtx->hashBits;
    uint32_t searchMatchNb = cCtx->step << RAPIDLZ_STEP_FORWARD_BASE;
    uint32_t searchMatchNbTmp = searchMatchNb;
    uint8_t bufferLimit = cCtx->bufferLimit;

    while (RAPIDLZ_LIKELY(srcCurr <= srcLimit)) {
        for (;;) {
            hashValue = RapidlzCalcHashValue(srcCurr, hashType, hashBits);
            matchBegin = srcBegin + RapidlzGetPosOnTable(hashValue, hashTable, hashType);
            RapidlzPutPosOnTable(srcCurr - srcBegin, hashValue, hashTable, hashType);

            // 循环加入hans直到找到匹配成功
            if ((RAPIDLZ_READ32BIT(srcCurr) == RAPIDLZ_READ32BIT(matchBegin)) &&
                RAPIDLZ_LIKELY((srcCurr - matchBegin) <= RAPIDLZ_MAX_OFFSET)) {
                break;
            }

            srcCurr += step;
            step = (searchMatchNbTmp++ >> RAPIDLZ_STEP_FORWARD_BASE);

            if (srcCurr > srcLimit) {
                dstCurr = RapidlzStoreLastLiterals(dstCurr, dstEnd, srcAnchor, srcEnd - srcAnchor, bufferLimit);
                if (dstCurr == NULL) {
                    return 0;
                }
                return dstCurr - dstBegin;
            }
        }
        step = 1;
        searchMatchNbTmp = searchMatchNb;

        srcCurrMatchEnd = RapidlzCompressExpandBackward(matchLimit,
            matchBegin + RAPIDLZ_MIN_MATCH, srcCurr + RAPIDLZ_MIN_MATCH); // 向后扩展
        RAPIDLZ_EXPAND_FORWARD(srcBegin, matchBegin, srcCurr, srcAnchor); // 向前扩展
        matchLength = srcCurrMatchEnd - srcCurr - RAPIDLZ_MIN_MATCH; // 匹配长度如果非0则编码时要默认减4，解码时默认加4
        offset = (uint16_t)(srcCurr - matchBegin);
        literalLength = srcCurr - srcAnchor;
        if (bufferLimit != 0) { // 8 = token[1] + offset[2] + MIN_LITERALS_SIZE [5]
            uint32_t writeSize = literalLength + 8 + (literalLength + matchLength / RAPIDLZ_MAX_BYTE_VALUE);
            if (RAPIDLZ_UNLIKELY(dstCurr + writeSize > dstEnd)) {
                RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "dstEnd - dstCur:%zu writeSize:%u\n", dstEnd - dstCurr, writeSize);
                return 0;
            }
        }
        dstCurr = RapidlzStoreSequence(dstCurr, srcAnchor, literalLength, matchLength, offset);
        srcCurr = srcCurrMatchEnd;
        srcAnchor = srcCurr;
        hashValue = RapidlzCalcHashValue(srcCurr - 2, hashType, hashBits); // 匹配成功后不加入所有hash，只加入最后前2字节的hash
        RapidlzPutPosOnTable(srcCurr - 2 - srcBegin, hashValue, hashTable, hashType);

        if (RAPIDLZ_UNLIKELY(srcCurr > srcLimit)) {
            break;
        }

        hashValue = RapidlzCalcHashValue(srcCurr, hashType, hashBits);
        matchBegin = srcBegin + RapidlzGetPosOnTable(hashValue, hashTable, hashType);
        RapidlzPutPosOnTable(srcCurr - srcBegin, hashValue, hashTable, hashType);

        if ((RAPIDLZ_READ32BIT(srcCurr) != RAPIDLZ_READ32BIT(matchBegin)) ||
            RAPIDLZ_UNLIKELY((srcCurr - matchBegin) > RAPIDLZ_MAX_OFFSET)) {
            srcCurr++;
            continue;
        }

        srcCurrMatchEnd = RapidlzCompressExpandBackward(matchLimit,
            matchBegin + RAPIDLZ_MIN_MATCH, srcCurr + RAPIDLZ_MIN_MATCH);
        matchLength = srcCurrMatchEnd - srcCurr - RAPIDLZ_MIN_MATCH;
        offset = (uint16_t)(srcCurr - matchBegin);
        if (bufferLimit != 0) { // 8 = token[1] + offset[2] + MIN_LITERALS_SIZE [5]
            const uint32_t writeSize = 8 + matchLength / RAPIDLZ_MAX_BYTE_VALUE;
            if (RAPIDLZ_UNLIKELY(dstCurr + writeSize > dstEnd)) {
                RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "dstEnd - dstCur:%zu writeSize:%u\n", dstEnd - dstCurr, writeSize);
                return 0;
            }
        }
        *dstCurr = 0;
        dstCurr = RapidlzStoreOffMatch(dstCurr + 1, dstCurr, matchLength, offset);

        srcCurr = srcCurrMatchEnd;
        srcAnchor = srcCurr;

        hashValue = RapidlzCalcHashValue(srcCurr - 2, hashType, hashBits); // 这里加入前2个字节的hash
        RapidlzPutPosOnTable(srcCurr - 2 - srcBegin, hashValue, hashTable, hashType);
    }

    if (srcAnchor < srcEnd) {
        dstCurr = RapidlzStoreLastLiterals(dstCurr, dstEnd, srcAnchor, srcEnd - srcAnchor, bufferLimit);
        if (dstCurr == NULL) {
            return 0;
        }
    }

    return dstCurr - dstBegin;
}

static void RapidlzCCtxFree(RapidlzCCtx *cCtx)
{
    if (cCtx != NULL) {
        if (cCtx->hashTable != NULL) {
            free(cCtx->hashTable);
            cCtx->hashTable = NULL;
        }

        free(cCtx);
    }
}

size_t RapidlzCompress(const void *src, void *dst, size_t srcSize, size_t dstSize, int acceleration)
{
    if (src == NULL || dst == NULL || srcSize == 0 || dstSize == 0) {
        RAPIDLZ_LOG(RAPIDLZ_INPUT_INVALID, "input invalid\n");
        return 0;
    }

    if (acceleration < 1 || acceleration > RAPIDLZ_ACCELERATION_MAX) {
        RAPIDLZ_LOG(RAPIDLZ_INPUT_INVALID, "acceleration:%d\n", acceleration);
        return 0;
    }

    RapidlzCCtx *cCtx = (RapidlzCCtx *)malloc(sizeof(RapidlzCCtx));
    if (cCtx == NULL) {
        RAPIDLZ_LOG(RAPIDLZ_MALLOC_FAILED, "cCtx malloc failed\n");
        return 0;
    }

    cCtx->hashBits = RAPIDLZ_MIN_HASH_BIT;
    size_t totalHashSize;
    if (srcSize <= RAPIDLZ_SRC_SIZE_THRESHOLD) {
        cCtx->hashType = RAPIDLZ_HASH_TYPE_4;
        if (srcSize >= 64) { // 大于64的话适当增大点表大小
            cCtx->hashBits = (RapidlzHighBit64(srcSize) > RAPIDLZ_MAX_HASH_BIT) ? (RAPIDLZ_MAX_HASH_BIT + 1) :
                RapidlzHighBit64(srcSize);
        }
        totalHashSize = sizeof(uint16_t) * (uint32_t)(1 << cCtx->hashBits);
    } else {
        cCtx->hashType = RAPIDLZ_HASH_TYPE_5;
        cCtx->hashBits = RAPIDLZ_MAX_HASH_BIT;
        totalHashSize = sizeof(uint32_t) * (uint32_t)(1 << cCtx->hashBits);
    }

    uint8_t *table = (uint8_t *)malloc(totalHashSize); /* 根据不同hashType分配表内存 */
    if (table == NULL) {
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

#ifdef __cplusplus
}
#endif