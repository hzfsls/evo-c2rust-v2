/*
 * @file rapidlz_dictionary.c
 * Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * Description: stream compress & decompress function description
 * Author: Anonym
 * Create: 2024-08-22
 */
#include <stddef.h>
#include <stdbool.h>
#include "rapidlz.h"
#include "rapidlz_inner.h"

#ifdef __cplusplus
extern "C" {
#endif

/* public */
#define RAPIDLZ_MAX_DICT_SIZE (64 * 1024)
#define RAPIDLZ_LAST_LITERALS 5
#define RAPIDLZ_RETURN_IF_NOT_TRUE(condition, errCode) \
    do { \
        if (!condition) { \
            RAPIDLZ_LOG(errCode, " "); \
            return errCode; \
        } \
    } while (0)

#define RAPIDLZ_RETURN_IF_NOT_EOK(condition, errCode) \
    do { \
        if (condition != EOK) { \
            RAPIDLZ_LOG(errCode, " "); \
            return errCode; \
        } \
    } while (0)

/* decompress */
#define RAPIDLZ_DEC_NOT_OK 0
#define RAPIDLZ_ERROR_PARAM_UNSUPPORTED (-1)
#define RAPIDLZ_ERROR_OUTPUT (int)(-((void *)curSrc - (void *)src) - 1)
#define RAPIDLZ_DICT_HASH_MOVE_BYTES 3
#define RAPIDLZ_POSITION_UPDATE(curSrc, curDest, len) \
    do { \
        curDest += len; \
        curSrc += len; \
    } while (0)

#define RAPIDLZ_SAFE_LIT_COPY(curSrc, leftSrcSize, curDest, destEnd, litLen) \
    do { \
        if (RAPIDLZ_UNLIKELY(litLen > leftSrcSize || memmove_s(curDest, destEnd - curDest, curSrc, litLen) != EOK)) { \
            RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "litLen:%u dstEnd - dst:%zu\n", litLen, leftSrcSize); \
            return RAPIDLZ_ERROR_OUTPUT; \
        } \
    } while (0)

#define RAPIDLZ_SAFE_COPY_TILL_END(curSrc, leftSrcSize, curDest, destEnd, len) \
    do { \
        RAPIDLZ_SAFE_LIT_COPY(curSrc, leftSrcSize, curDest, destEnd, len); \
        RAPIDLZ_POSITION_UPDATE(curSrc, curDest, len); \
        if (leftSrcSize == len) { \
            return curDest - (uint8_t *)(dest); \
        } \
    } while (0)

#define RAPIDLZ_FAST_SAFE_COPY_BY_BYTES(curDest, matchSrc, len) do { \
    while ((len) > 2) { \
        *(curDest)++ = *(matchSrc)++; \
        *(curDest)++ = *(matchSrc)++; \
        *(curDest)++ = *(matchSrc)++; \
        (len) -= 3; \
    } \
    if ((len) > 0) { \
        *(curDest)++ = *(matchSrc)++; \
        if ((len) > 1) { \
            *(curDest)++ = *(matchSrc)++; \
        } \
    } \
} while (0)

/* compress */
#define RAPIDLZ_STREAM_HASH_BYTES 4
#define RAPIDLZ_STREAM_ACCELERAT_MIN 1
#define RAPIDLZ_STREAM_ACCELERAT_MAX 10
#define RAPIDLZ_PTR_DIFF_MAX_32 0x80000000
#define RAPIDLZ_MAX_INPUT_SIZE 0x7E000000
#define RAPIDLZ_LAST_LITERAL_LENGTH 13
#define RAPIDLZ_ENC_NOT_OK 0
#define RAPIDLZ_FAST_DIST_MIN 64
#define RAPIDLZ_MIN_COMPRESSED_SIZE 12
#define RAPIDLZ_CURRENT_OFFSET (64 * 1024)
#define RAPIDLZ_GOLDEN_SECTION_PRIME 2654435761U
#define RAPIDLZ_STREAM_HASH_BITS 20
#define RAPIDLZ_LITERAL_LEN_COPY_END(curDest, len) \
    ((curDest) + (len) + 1 + (((len) + RAPIDLZ_MAX_BYTE_VALUE - RAPIDLZ_MAX_4BIT_VALUE) / RAPIDLZ_MAX_BYTE_VALUE))

#define RAPIDLZ_LIT_AND_MATCH_COPY_END(curDest, matchLen) \
    (curDest) + (1 + RAPIDLZ_LAST_LITERALS) + ((matchLen + 240) / RAPIDLZ_MAX_BYTE_VALUE)

typedef int (*RapidlzCompressFunc)(RapidlzStreamCtx* strmCtx,
    const char* src, char* dest, int srcSize, int destSize);
typedef int (*RapidlzDecompressFunc)(const char *const src, char *const dest, int srcSize, int outBufferSize,
    const char *dictStart, int dictSize);

static RAPIDLZ_ALWAYS_INLINE int RapidlzZeroBytesDecode(const char *const src, int srcSize)
{
    if ((*src == 0) && (srcSize == 1)) {
        return RAPIDLZ_DEC_NOT_OK;
    }
    return RAPIDLZ_ERROR_PARAM_UNSUPPORTED;
}

static RAPIDLZ_ALWAYS_INLINE void RapidlzDictSizeNorm(RapidlzStreamCtx* strmCtx, char *src, char *dictEnd)
{
    if (strmCtx->dictSize < RAPIDLZ_STREAM_HASH_BYTES) { // dictSize 不足一个哈希值,按照无字典模式压缩
        strmCtx->dictSize = 0;
        strmCtx->dict = (uint8_t *)src;
        dictEnd = src;
    }
    if (strmCtx->dictSize > RAPIDLZ_MAX_DICT_SIZE) { // 只取后64KB
        strmCtx->dict = (uint8_t *)(dictEnd - RAPIDLZ_MAX_DICT_SIZE);
        strmCtx->dictSize = RAPIDLZ_MAX_DICT_SIZE;
    }
}

static RAPIDLZ_ALWAYS_INLINE uint32_t RapidlzHash4GetPos(uint32_t hashValue, uint32_t *hashTable)
{
    return hashTable[hashValue];
}

static RAPIDLZ_ALWAYS_INLINE void RapidlzHash4PutPos(uint32_t pos, uint32_t hashValue, uint32_t *hashTable)
{
    hashTable[hashValue] = pos;
}

static RAPIDLZ_ALWAYS_INLINE uint32_t RapidlzHash4CalcValue(const uint8_t *curSrc)
{
    return (RAPIDLZ_READ32BIT(curSrc) * RAPIDLZ_GOLDEN_SECTION_PRIME) >> RAPIDLZ_STREAM_HASH_BITS;
}

static RAPIDLZ_ALWAYS_INLINE size_t RapidlzStoreLiteralLen(uint32_t litLen, uint8_t *curDest)
{
    uint8_t *curDestAnchor = curDest;
    if (litLen >= RAPIDLZ_MAX_4BIT_VALUE) {
        uint32_t tmp = litLen - RAPIDLZ_MAX_4BIT_VALUE;
        *(curDest)++ = (RAPIDLZ_MAX_4BIT_VALUE << 4); // 高4位
        for (; tmp >= RAPIDLZ_MAX_BYTE_VALUE; tmp -= RAPIDLZ_MAX_BYTE_VALUE) {
            *(curDest)++ = RAPIDLZ_MAX_BYTE_VALUE;
        }
        *(curDest)++ = (uint8_t)tmp;
    }
    else {
        *(curDest)++ = (uint8_t)(litLen << 4); // 高4位
    }
    return curDest - curDestAnchor;
}

static RAPIDLZ_ALWAYS_INLINE size_t RapidlzStoreMatchLen(uint32_t matchLen, uint8_t *token, uint8_t *curDest)
{
    uint8_t *curDestAnchor = curDest;
    if (matchLen >= RAPIDLZ_MAX_4BIT_VALUE) {
        *token += RAPIDLZ_MAX_4BIT_VALUE;
        matchLen -= RAPIDLZ_MAX_4BIT_VALUE;
        *curDest = RAPIDLZ_MAX_BYTE_VALUE;
        while (matchLen >= RAPIDLZ_MAX_BYTE_VALUE) {
            *(++curDest) = RAPIDLZ_MAX_BYTE_VALUE;
            matchLen -= RAPIDLZ_MAX_BYTE_VALUE;
        }
        curDest += matchLen / RAPIDLZ_MAX_BYTE_VALUE;
        *curDest++ = (uint8_t)(matchLen % RAPIDLZ_MAX_BYTE_VALUE);
    } else {
        *token += (uint8_t)(matchLen);
    }
    return curDest - curDestAnchor;
}

static RAPIDLZ_ALWAYS_INLINE int RapidlzZeroBytesEncode(char* dest, int destSize)
{
    RAPIDLZ_RETURN_IF_NOT_TRUE(!(destSize <= 0), RAPIDLZ_ENC_NOT_OK);
    dest[0] = 0;
    return 1; // 0字节压缩后长度为1
}

static int RapidlzEncLastLiterals(uint8_t *curSrcAnchor, uint8_t *srcEnd,
    uint8_t *curDest, uint8_t *destEnd, char* destStart) // no inline
{
    size_t lastLiteralsLen = (size_t)(srcEnd - curSrcAnchor);
    RAPIDLZ_RETURN_IF_NOT_TRUE(!(RAPIDLZ_LITERAL_LEN_COPY_END(curDest, lastLiteralsLen) > destEnd),
        RAPIDLZ_ENC_NOT_OK);
    curDest += RapidlzStoreLiteralLen(lastLiteralsLen, curDest);
    RAPIDLZ_RETURN_IF_NOT_EOK(memcpy_s(curDest, destEnd - curDest, curSrcAnchor, lastLiteralsLen),
        RAPIDLZ_ENC_NOT_OK);
    curDest += lastLiteralsLen;
    return (int)((void *)curDest - (void *)destStart);
}

static RAPIDLZ_ALWAYS_INLINE bool RapidlzStreamEncLiterals(uint8_t *curSrc,
    uint8_t *curSrcAnchor, uint8_t **curDest, uint8_t *destEnd)
{
    uint32_t litLen = curSrc - curSrcAnchor;
    RAPIDLZ_RETURN_IF_NOT_TRUE(!(RAPIDLZ_UNLIKELY(RAPIDLZ_LITERAL_LEN_COPY_END(*curDest, litLen) > destEnd)), false);
    *curDest += RapidlzStoreLiteralLen(litLen, *curDest);
    RapidlzWildCopy8(curSrcAnchor, *curDest, *curDest + litLen);
    *curDest += litLen;
    return true;
}

int RapidlzLoadDict(RapidlzStreamCtx *strmCtx, const char *dictionary, int dictSize)
{
    const uint8_t *dictStart = (const uint8_t *)dictionary;
    const uint8_t *dictEnd = (const uint8_t *)dictionary + dictSize;
    RAPIDLZ_RETURN_IF_NOT_EOK(memset_s(strmCtx,
        sizeof(RapidlzStreamCtx), 0, sizeof(RapidlzStreamCtx)), RAPIDLZ_ENC_NOT_OK);
    RAPIDLZ_RETURN_IF_NOT_TRUE(!(dictSize < RAPIDLZ_STREAM_HASH_BYTES), RAPIDLZ_ENC_NOT_OK);

    if (dictSize > RAPIDLZ_MAX_DICT_SIZE) {
        dictStart = dictEnd - RAPIDLZ_MAX_DICT_SIZE; // 只取后64k作为字典
    }

    strmCtx->dict = (uint8_t *)dictStart;
    strmCtx->dictSize = dictEnd - dictStart;
    strmCtx->currentOffset = RAPIDLZ_MAX_DICT_SIZE; // 每轮load都认为是流式压缩的第一次压缩

    uint32_t index32 = strmCtx->currentOffset - strmCtx->dictSize;

    const uint8_t *curDict = dictStart;
    while (curDict <= dictEnd - RAPIDLZ_STREAM_HASH_BYTES) {
        uint32_t hashValue = RapidlzHash4CalcValue(curDict);
        RapidlzHash4PutPos(index32, hashValue, strmCtx->hashTable);
        curDict += RAPIDLZ_DICT_HASH_MOVE_BYTES;
        index32 += RAPIDLZ_DICT_HASH_MOVE_BYTES;
    }

    return (int)strmCtx->dictSize;
}

#define RAPIDLZ_CONTINUE_IF_NOT_A_MATCH(matchOffset, prefixDictLimit, current) \
    if (((matchOffset) < (prefixDictLimit)) || ((matchOffset) + RAPIDLZ_MAX_OFFSET < (current))) { continue; }

int RapidlzCompWithExternalDict(RapidlzStreamCtx* strmCtx, const char* src, char* dest, int srcSize, int destSize)
{
    /* load IO */
    uint8_t *curSrc = (uint8_t *)src; // 告警
    uint8_t *curSrcAnchor = (uint8_t *)curSrc; // anchor of curSrc
    uint8_t *srcEnd = curSrc + srcSize;
    uint8_t *curDest = (uint8_t *)dest;
    uint8_t *destEnd = curDest + destSize;
    if (srcSize < RAPIDLZ_LAST_LITERAL_LENGTH) {
        return RapidlzEncLastLiterals(curSrcAnchor, srcEnd, curDest, destEnd, dest);
    }
    /* load boundary */
    uint8_t *matchStartLimit = srcEnd - RAPIDLZ_MIN_COMPRESSED_SIZE + 1;
    uint8_t *matchEndLimit = srcEnd - RAPIDLZ_LAST_LITERALS;
    uint32_t startIndex = strmCtx->currentOffset;
    uint8_t *base = (uint8_t *)src - startIndex;
    /* load dict */
    const uint8_t *dict, *dictBase, *dictEnd, *prefixDictStart;
    uint32_t dictSize;
    uint32_t offset = 0;

    dict = strmCtx->dict;
    dictSize = strmCtx->dictSize;
    dictBase = dict + dictSize - strmCtx->currentOffset;
    prefixDictStart = 0;
    dictBase = dict + dictSize - strmCtx->currentOffset;
    dictEnd = dict + dictSize;
    strmCtx->dictSize += srcSize;

    uint32_t prefixDictLimit = startIndex - dictSize;
    strmCtx->currentOffset += (uint32_t)srcSize; // update currentOffset to base

    uint32_t hashValue = RapidlzHash4CalcValue(curSrc);
    RapidlzHash4PutPos(startIndex, hashValue, strmCtx->hashTable);
    curSrc++;
    uint32_t forwardHashValue = RapidlzHash4CalcValue(curSrc);

    const uint8_t *match;
    uint8_t *token; // match相对base的偏移为matchOffset
    int acceleration = strmCtx->acceleration;
    while (true) {
        uint8_t *forwardPos = curSrc;
        int jumpStep = 1;
        int searchMatchNb = acceleration << RAPIDLZ_STEP_FORWARD_BASE;
        do {
            hashValue = forwardHashValue;
            uint32_t current = (forwardPos - base);
            uint32_t matchOffset = RapidlzHash4GetPos(hashValue, strmCtx->hashTable);
            curSrc = forwardPos;
            forwardPos += jumpStep;
            jumpStep = (searchMatchNb++ >> RAPIDLZ_STEP_FORWARD_BASE);

            if (RAPIDLZ_UNLIKELY(forwardPos > matchStartLimit)) {
                return RapidlzEncLastLiterals(curSrcAnchor, srcEnd, curDest, destEnd, dest);
            }

            if (matchOffset < startIndex) {
                /* src无匹配 , try external mem */
                match = dictBase + matchOffset;
                prefixDictStart = dict;
            } else { // 匹配位置在src内（因为匹配是从base开始算的，base + offset == src）
                match = base + matchOffset;
                prefixDictStart = (uint8_t *)src;
            }

            forwardHashValue = RapidlzHash4CalcValue(forwardPos);
            RapidlzHash4PutPos(current, hashValue, strmCtx->hashTable);
            RAPIDLZ_CONTINUE_IF_NOT_A_MATCH(matchOffset, prefixDictLimit, current);
            if (RAPIDLZ_READ32BIT(curSrc) == RAPIDLZ_READ32BIT(match)) { // 找到四字节匹配
                offset = current - matchOffset;
                break;
            }
        } while (1);

        RAPIDLZ_EXPAND_FORWARD(prefixDictStart, match, curSrc, curSrcAnchor); // 向前扩展，因为跳了step，不一定代表匹配起点

        /* Encode Literals */
        token = curDest;
        RAPIDLZ_RETURN_IF_NOT_TRUE(RapidlzStreamEncLiterals(curSrc, curSrcAnchor, &curDest, destEnd),
            RAPIDLZ_ENC_NOT_OK);

_OFFSET_AND_MATCH: // 前面是一定会压literal，但是可能出现不需要压缩literal的情况，比如： asdfjasf(6,4)(token && 高四位 == 0)(9,4),
        /* Encode Offset */
        RapidlzWriteLE16(curDest, offset);
        curDest += 2; /* 指针移动2字节 */
        /* Encode MatchLength */
        uint32_t matchLen;
        uint8_t *curSrcMatchEnd;

        if (prefixDictStart == dict) { // 在字典里的匹配,要判断是否有跨匹配, 这里的判断也可以写成 dictEnd > match
            uint8_t *srcLimitOnlyWithDict = curSrc + (dictEnd - match); // 仅在字典里的匹配
            if (srcLimitOnlyWithDict > matchEndLimit) {
                srcLimitOnlyWithDict = matchEndLimit;
            }
            curSrcMatchEnd = (uint8_t *)RapidlzCompressExpandBackward(srcLimitOnlyWithDict,
                match + RAPIDLZ_MIN_MATCH, curSrc + RAPIDLZ_MIN_MATCH);
            matchLen = curSrcMatchEnd - curSrc - RAPIDLZ_MIN_MATCH;
            curSrc = curSrcMatchEnd;
        
            if (curSrc == srcLimitOnlyWithDict) { // 字典模式才会出现, more
                curSrcMatchEnd = (uint8_t *)RapidlzCompressExpandBackward(matchEndLimit,
                    (uint8_t *)src, srcLimitOnlyWithDict);
                matchLen += (curSrcMatchEnd - curSrc); // more
                curSrc = curSrcMatchEnd;
            }
        } else { // 仅在src的匹配
            curSrcMatchEnd = (uint8_t *)RapidlzCompressExpandBackward(matchEndLimit,
                match + RAPIDLZ_MIN_MATCH, curSrc + RAPIDLZ_MIN_MATCH);
            matchLen = curSrcMatchEnd - curSrc - RAPIDLZ_MIN_MATCH;
            curSrc = curSrcMatchEnd;
        }
#ifdef RAPIDLZ_DEBUG
        if (RAPIDLZ_UNLIKELY(RAPIDLZ_LIT_AND_MATCH_COPY_END(curDest, matchLen) > destEnd)) {
            return RAPIDLZ_ENC_NOT_OK;
        }
#endif /* RAPIDLZ_DEBUG */
        curDest += RapidlzStoreMatchLen(matchLen, token, curDest);
        curSrcAnchor = curSrc;
        if (curSrc >= matchStartLimit) {
            break;
        }
        uint32_t hv2 = RapidlzHash4CalcValue(curSrc - 2);
        uint32_t index = curSrc - 2 - base;
        RapidlzHash4PutPos(index, hv2, strmCtx->hashTable);

        hashValue = RapidlzHash4CalcValue(curSrc);
        uint32_t current = (curSrc - base); // current 对应 currentOffset
        uint32_t matchOffset = RapidlzHash4GetPos(hashValue, strmCtx->hashTable);
        if (matchOffset < startIndex) {
            /* src 无匹配 , try external mem */
            match = dictBase + matchOffset;
            prefixDictStart = dict;
        } else { // 匹配位置在src内（因为匹配是从base开始算的，base + offset == src）
            match = base + matchOffset;
            prefixDictStart = (uint8_t *)src;
        }

        RapidlzHash4PutPos(current, hashValue, strmCtx->hashTable);

        if ((matchOffset >= prefixDictLimit) && (matchOffset + RAPIDLZ_MAX_OFFSET >= current)) {
            if (RAPIDLZ_READ32BIT(curSrc) == RAPIDLZ_READ32BIT(match)) {
                token = curDest++;
                *token = 0;
                offset = current - matchOffset;
                goto _OFFSET_AND_MATCH;
            }
        }
        forwardHashValue = RapidlzHash4CalcValue(++curSrc); // 下一循环准备
    }

    return RapidlzEncLastLiterals(curSrcAnchor, srcEnd, curDest, destEnd, dest);
}

int RapidlzCompWithPrefixDict(RapidlzStreamCtx* strmCtx, const char* src, char* dest, int srcSize, int destSize)
{
    uint8_t *curSrc = (uint8_t *)src; // 告警
    uint8_t *curSrcAnchor = (uint8_t *)curSrc; // anchor of curSrc
    uint8_t *srcEnd = curSrc + srcSize;
    uint8_t *curDest = (uint8_t *)dest;
    uint8_t *destEnd = curDest + destSize;
    if (srcSize < RAPIDLZ_LAST_LITERAL_LENGTH) {
        return RapidlzEncLastLiterals(curSrcAnchor, srcEnd, curDest, destEnd, dest);
    }
    uint8_t *matchStartLimit = srcEnd - RAPIDLZ_MIN_COMPRESSED_SIZE + 1;
    uint8_t *matchEndLimit = srcEnd - RAPIDLZ_LAST_LITERALS;
    uint32_t startIndex = strmCtx->currentOffset; // src相对base的index
    uint8_t *base = (uint8_t *)src - startIndex;

    /* load dict */
    uint8_t *prefixDictStart;
    uint32_t dictSize;
    dictSize = strmCtx->dictSize;
    prefixDictStart = (uint8_t *)src - dictSize;
    strmCtx->dictSize += srcSize;
    uint32_t prefixDictLimit = startIndex - dictSize;
    strmCtx->currentOffset += (uint32_t)srcSize; // update currentOffset to base

    uint32_t hashValue = RapidlzHash4CalcValue(curSrc);
    RapidlzHash4PutPos(startIndex, hashValue, strmCtx->hashTable);
    curSrc++;
    uint32_t forwardHashValue = RapidlzHash4CalcValue(curSrc);

    uint8_t *match, *token;
    int acceleration = strmCtx->acceleration;
    while (true) {
        uint8_t *forwardPos = curSrc;
        int step = 1;
        int searchMatchNb = acceleration << RAPIDLZ_STEP_FORWARD_BASE;
        do {
            hashValue = forwardHashValue;
            uint32_t current = (forwardPos - base); // current和currentOffset都是相对base而言
            uint32_t matchOffset = RapidlzHash4GetPos(hashValue, strmCtx->hashTable);
            curSrc = forwardPos;
            forwardPos += step;
            step = (searchMatchNb++ >> RAPIDLZ_STEP_FORWARD_BASE); // 跳步查找，加速，降压缩率

            if (RAPIDLZ_UNLIKELY(forwardPos > matchStartLimit)) {
                return RapidlzEncLastLiterals(curSrcAnchor, srcEnd, curDest, destEnd, dest);
            }
            // 连续内存块
            match = base + matchOffset;
            forwardHashValue = RapidlzHash4CalcValue(forwardPos);
            RapidlzHash4PutPos(current, hashValue, strmCtx->hashTable);

            if (matchOffset < prefixDictLimit) {
                continue;
            }
            if ((matchOffset + RAPIDLZ_MAX_OFFSET) < current) { // 其他模式匹配位置太远。
                continue;
            }
            if (RAPIDLZ_READ32BIT(curSrc) == RAPIDLZ_READ32BIT(match)) { // 找到四字节匹配
                break;
            }
        } while (1);

        RAPIDLZ_EXPAND_FORWARD(prefixDictStart, match, curSrc, curSrcAnchor); // 向前扩展，因为跳了step，不一定代表匹配起点

        /* Encode Literals */
        token = curDest;
        RAPIDLZ_RETURN_IF_NOT_TRUE(RapidlzStreamEncLiterals(curSrc, curSrcAnchor, &curDest, destEnd),
            RAPIDLZ_ENC_NOT_OK);

_OFFSET_AND_MATCH: // 前面是一定会压literal，但是可能出现不需要压缩literal的情况，比如： asdfjasf(6,4)(token && 高四位 == 0)(9,4),
        /* Encode Offset */
        RapidlzWriteLE16(curDest, curSrc - match);
        curDest += 2; /* 指针移动2字节 */
        /* Encode MatchLength */
        uint32_t matchLen;
        uint8_t *curSrcMatchEnd;
        // constant mem
        curSrcMatchEnd = (uint8_t *)RapidlzCompressExpandBackward(matchEndLimit,
            match + RAPIDLZ_MIN_MATCH, curSrc + RAPIDLZ_MIN_MATCH);
        matchLen = curSrcMatchEnd - curSrc - RAPIDLZ_MIN_MATCH;
        curSrc = curSrcMatchEnd;
#ifdef RAPIDLZ_DEBUG
        if (RAPIDLZ_UNLIKELY(RAPIDLZ_LIT_AND_MATCH_COPY_END(curDest, matchLen) > destEnd)) {
            return RAPIDLZ_ENC_NOT_OK;
        }
#endif /* RAPIDLZ_DEBUG */
        curDest += RapidlzStoreMatchLen(matchLen, token, curDest);

        /* anchor update, next lz77 */
        curSrcAnchor = curSrc;
        if (curSrc >= matchStartLimit) {
            break;
        }
        uint32_t hv = RapidlzHash4CalcValue(curSrc - 2);
        uint32_t index = curSrc - 2 - base;
        RapidlzHash4PutPos(index, hv, strmCtx->hashTable);

        hashValue = RapidlzHash4CalcValue(curSrc);
        uint32_t current = (curSrc - base); // current 对应 currentOffset
        uint32_t matchOffset = RapidlzHash4GetPos(hashValue, strmCtx->hashTable);

        match = base + matchOffset;
        
        RapidlzHash4PutPos(current, hashValue, strmCtx->hashTable);
        if ((matchOffset >= prefixDictLimit) && ((matchOffset + RAPIDLZ_MAX_OFFSET) >= current)) {
            if (RAPIDLZ_READ32BIT(curSrc) == RAPIDLZ_READ32BIT(match)) {
                token = curDest++;
                *token = 0;
                goto _OFFSET_AND_MATCH;
            }
        }
        forwardHashValue = RapidlzHash4CalcValue(++curSrc); // 下一循环准备
    }

    return RapidlzEncLastLiterals(curSrcAnchor, srcEnd, curDest, destEnd, dest);
}

void RapidlzStrmCtxNorm(RapidlzStreamCtx *strmCtx, char *src, int srcSize, char *dictEnd)
{
    RapidlzDictSizeNorm(strmCtx, src, dictEnd); // 流式压缩过程中的字典大小规范化
    strmCtx->acceleration = ((strmCtx->acceleration < RAPIDLZ_STREAM_ACCELERAT_MIN) ? RAPIDLZ_STREAM_ACCELERAT_MIN :
                            ((strmCtx->acceleration > RAPIDLZ_STREAM_ACCELERAT_MAX) ? RAPIDLZ_STREAM_ACCELERAT_MAX :
                                                                                      strmCtx->acceleration));
    if (strmCtx->currentOffset + (uint32_t)srcSize > RAPIDLZ_PTR_DIFF_MAX_32) { // ptr32数据类型越界，reset
        uint32_t delta = strmCtx->currentOffset - RAPIDLZ_MAX_DICT_SIZE;
        int i = 0;
        for (; i < RAPIDLZ_STREAM_HASH_SIZE; i++) {
            if (strmCtx->hashTable[i] < delta) {
                strmCtx->hashTable[i] = 0;
            } else {
                strmCtx->hashTable[i] -= delta;
            }
        }
        strmCtx->currentOffset = RAPIDLZ_MAX_DICT_SIZE;
    }

    char* srcEnd = src + srcSize;
    if ((srcEnd > (char *)strmCtx->dict) && (srcEnd < dictEnd)) { // 当src与字典重合时，只取不交叉的部分。
        strmCtx->dictSize = (uint32_t)(dictEnd - srcEnd);
        strmCtx->dictSize = ((strmCtx->dictSize > RAPIDLZ_MAX_DICT_SIZE) ? RAPIDLZ_MAX_DICT_SIZE :
                           ((strmCtx->dictSize < RAPIDLZ_STREAM_HASH_BYTES) ? 0 : strmCtx->dictSize));
        strmCtx->dict = (uint8_t *)(dictEnd - strmCtx->dictSize);
    }
}

static int g_enc32table[8] = {0, 1, 2,  1,  0,  4, 4, 4};
static int g_dec64table[8] = {0, 0, 0, -1, -4,  1, 2, 3};
// 当copy即将到达结尾:destEndFast - RAPIDLZ_COPY_PROTECT_SIZE + RAPIDLZ_LAST_LITERALS时
static RAPIDLZ_ALWAYS_INLINE void RapidlzSafeCopyMatchFast(uint8_t *curDest, uint8_t *matchSrc,
    uint8_t *destEnd, uint16_t offset, uint32_t len)
{
    errno_t err;
    uint8_t *curDestCopyEnd = curDest + len;
    if (offset < RAPIDLZ_EIGHT_BYTE) { // offset < 8 说明重叠了
        curDest[0] = matchSrc[0];
        curDest[1] = matchSrc[1];
        curDest[2] = matchSrc[2]; // 下标2copy
        curDest[3] = matchSrc[3]; // 下标3copy
        matchSrc += g_enc32table[offset];
        err = memcpy_s(curDest + RAPIDLZ_FOUR_BYTE, RAPIDLZ_FOUR_BYTE, matchSrc, RAPIDLZ_FOUR_BYTE);
        matchSrc -= g_dec64table[offset];
    } else {
        err = memcpy_s(curDest, RAPIDLZ_EIGHT_BYTE, matchSrc, RAPIDLZ_EIGHT_BYTE);
        matchSrc += RAPIDLZ_EIGHT_BYTE;
    }
#ifdef RAPIDLZ_DEBUG
    RAPIDLZ_RETURN_IF_NOT_EOK(err, RAPIDLZ_DEC_NOT_OK);
#else /* RAPIDLZ_DEBUG */
    (void)err;
#endif /* RAPIDLZ_DEBUG */
    curDest += RAPIDLZ_EIGHT_BYTE;
    uint8_t *curDestLimit = destEnd - (RAPIDLZ_EIGHT_BYTE - 1);
    if (curDest < curDestLimit) {
        RapidlzWildCopy8(matchSrc, curDest, curDestLimit);
        matchSrc += (curDestLimit - curDest);
        curDest = curDestLimit;
    }
    while (curDest < curDestCopyEnd) {
        *curDest++ = *matchSrc++;
    }
}

static int RapidlzDecWithPrefixDict(const char *const src, char *const dest, int srcSize, int outBufferSize,
    const char *dictStart, int dictSize)
{
    (void)dictStart;
    (void)dictSize;
#ifdef RAPIDLZ_DEBUG
    dictSize = (dictSize > RAPIDLZ_MAX_DICT_SIZE) ? RAPIDLZ_MAX_DICT_SIZE : dictSize;
    uint8_t *prefixDictStart = (uint8_t *)dest - dictSize;
#endif /* RAPIDLZ_DEBUG */
    const uint8_t *curSrc = (const uint8_t *)src;
    const uint8_t *srcEnd = curSrc + srcSize;
    uint8_t *curDest = (uint8_t *)dest;
    uint8_t *destEnd = curDest + outBufferSize;

    const uint8_t *srcEndFast = srcEnd - RAPIDLZ_COPY_PROTECT_SIZE;
    const uint8_t *destEndFast = destEnd - RAPIDLZ_COPY_PROTECT_SIZE;

    uint32_t token, len;
    uint16_t offset;
    uint8_t *matchSrc;
    uint32_t tmp = 0;
    size_t leftSrcSize;
    while (1) {
        token = *curSrc++;
        /* decode literal len */
        len = token >> 4; // token高4位
        if (RAPIDLZ_LIKELY(len < RAPIDLZ_MAX_4BIT_VALUE)) {
            if (RAPIDLZ_LIKELY((curSrc + len <= srcEndFast) && (curDest + len <= destEndFast))) {
                RapidlzCopy16Byte(curDest, curSrc);
                RAPIDLZ_POSITION_UPDATE(curSrc, curDest, len);
            } else {
                leftSrcSize = srcEnd - curSrc;
                RAPIDLZ_SAFE_COPY_TILL_END(curSrc, leftSrcSize, curDest, destEnd, len);
            }
        } else {
            RAPIDLZ_READ_OPTIONAL_LENGTH(len, curSrc, srcEnd, tmp);
            if (RAPIDLZ_LIKELY((curSrc + len <= srcEndFast) && (curDest + len <= destEndFast))) {
                RapidlzWildCopy16(curSrc, curDest, curDest + len);
                RAPIDLZ_POSITION_UPDATE(curSrc, curDest, len);
            } else {
                leftSrcSize = srcEnd - curSrc;
                RAPIDLZ_SAFE_COPY_TILL_END(curSrc, leftSrcSize, curDest, destEnd, len);
            }
        }
        /* decode offset */
        offset = RapidlzReadLE16Bit(curSrc);
        curSrc += 2; // 指针移动2字节offset大小
        matchSrc = curDest - offset;
#ifdef RAPIDLZ_DEBUG
        RAPIDLZ_RETURN_IF_NOT_TRUE(!(RAPIDLZ_UNLIKELY(matchSrc < prefixDictStart)), RAPIDLZ_DEC_NOT_OK);
#endif /* RAPIDLZ_DEBUG */
        /* decode match len */
        len = token & RAPIDLZ_MAX_4BIT_VALUE;

        if (len < RAPIDLZ_MAX_4BIT_VALUE) {
            len += 4; // 最小匹配距离4
        } else {
            RAPIDLZ_READ_OPTIONAL_LENGTH(len, curSrc, srcEnd, tmp);
            len += 4; // 最小匹配距离4
        }
#ifdef RAPIDLZ_DEBUG
        RAPIDLZ_RETURN_IF_NOT_TRUE(!(curDest + len > destEnd - RAPIDLZ_LAST_LITERALS), RAPIDLZ_DEC_NOT_OK);
#endif /* RAPIDLZ_DEBUG */

        if (RAPIDLZ_LIKELY((curDest + len) <=
            (destEndFast - RAPIDLZ_COPY_PROTECT_SIZE + RAPIDLZ_LAST_LITERALS))) { // 2 * 16 - 5
            RapidlzCopyMatchFast(curDest, matchSrc, offset, len);
            curDest += len;
        } else {
            if (RAPIDLZ_LIKELY(len < 1024)) { // 1024长度的匹配内,都认为是单字节拷贝最快
                RAPIDLZ_FAST_SAFE_COPY_BY_BYTES(curDest, matchSrc, len);
            } else {
                RapidlzSafeCopyMatchFast(curDest, matchSrc, destEnd, offset, len);
                curDest += len;
            }
        }
    }

    return (int)((void *)curDest - (void *)(dest));
}

#define RAPIDLZ_DICT_FAST_COPY_AVAIL(curSrc, len, srcEndFast, curDest, destEndFast) \
    (((curSrc) + (len) <= (srcEndFast)) && ((curDest) + (len) <= (destEndFast)))

#define RAPIDLZ_GET_MATCH_LEN(len, curSrc, srcEnd, temp) \
    do { \
        if ((len) < RAPIDLZ_MAX_4BIT_VALUE) { \
            (len) += 4; /* 最小匹配距离4 */ \
        } else { \
            RAPIDLZ_READ_OPTIONAL_LENGTH((len), (curSrc), (srcEnd), (temp)); \
            (len) += 4; /* 最小匹配距离4 */ \
        } \
    } while (0)

static int RapidlzDecWithExternalDict(const char *const src, char *const dest, int srcSize, int outBufferSize,
    const char *dictStart, int dictSize)
{
    const uint8_t *curSrc = (const uint8_t *)src;
    const uint8_t *srcEnd = curSrc + srcSize;
    uint8_t *curDest = (uint8_t *)dest;
    uint8_t *destEnd = curDest + outBufferSize;
    const uint8_t *srcEndFast = srcEnd - RAPIDLZ_COPY_PROTECT_SIZE;
    const uint8_t *destEndFast = destEnd - RAPIDLZ_COPY_PROTECT_SIZE;
    const uint8_t *dictEnd = (uint8_t *)dictStart + dictSize;

    uint32_t token, len;
    uint16_t offset;
    uint8_t *matchSrc;
    uint32_t temp = 0;
    size_t leftSrcSize;
    while (1) {
        token = *curSrc++;
        /* decode literal len */
        len = token >> 4; // token高4位
        if (RAPIDLZ_LIKELY(len < RAPIDLZ_MAX_4BIT_VALUE)) {
            if (RAPIDLZ_LIKELY(RAPIDLZ_DICT_FAST_COPY_AVAIL(curSrc, len, srcEndFast, curDest, destEndFast))) {
                RapidlzCopy16Byte(curDest, curSrc);
                RAPIDLZ_POSITION_UPDATE(curSrc, curDest, len);
            } else {
                leftSrcSize = srcEnd - curSrc;
                RAPIDLZ_SAFE_COPY_TILL_END(curSrc, leftSrcSize, curDest, destEnd, len);
            }
        } else {
            RAPIDLZ_READ_OPTIONAL_LENGTH(len, curSrc, srcEnd, temp);
            if (RAPIDLZ_LIKELY(RAPIDLZ_DICT_FAST_COPY_AVAIL(curSrc, len, srcEndFast, curDest, destEndFast))) {
                RapidlzWildCopy16(curSrc, curDest, curDest + len);
                RAPIDLZ_POSITION_UPDATE(curSrc, curDest, len);
            } else {
                leftSrcSize = srcEnd - curSrc;
                RAPIDLZ_SAFE_COPY_TILL_END(curSrc, leftSrcSize, curDest, destEnd, len);
            }
        }

        /* decode offset */
        offset = RapidlzReadLE16Bit(curSrc);
        curSrc += 2; // 指针移动2字节offset大小
        matchSrc = curDest - offset;
        /* decode match len */
        len = token & RAPIDLZ_MAX_4BIT_VALUE;

        RAPIDLZ_GET_MATCH_LEN(len, curSrc, srcEnd, temp);

#ifdef RAPIDLZ_DEBUG
        RAPIDLZ_RETURN_IF_NOT_TRUE(!(curDest + len > destEnd - RAPIDLZ_LAST_LITERALS), RAPIDLZ_DEC_NOT_OK);
#endif /* RAPIDLZ_DEBUG */
        if (matchSrc >= (uint8_t *)dest) { // 全在src内部匹配在这里处理
            if (RAPIDLZ_LIKELY((curDest + len) <=
                (destEndFast - RAPIDLZ_COPY_PROTECT_SIZE + RAPIDLZ_LAST_LITERALS))) { // 2 * 16 - 5
                RapidlzCopyMatchFast(curDest, matchSrc, offset, len);
                curDest += len;
            } else {
                if (RAPIDLZ_LIKELY(len < 1024)) { // 1024长度的匹配内,都认为是单字节拷贝最快
                    RAPIDLZ_FAST_SAFE_COPY_BY_BYTES(curDest, matchSrc, len);
                } else {
                    RapidlzSafeCopyMatchFast(curDest, matchSrc, destEnd, offset, len);
                    curDest += len;
                }
            }
        } else {
            errno_t err;
            if ((int)len <= ((uint8_t *)dest - matchSrc)) { // 所有的匹配对都在exteral字典中，直接memmove即可，内存重叠也可正确拷贝
                err = memmove_s(curDest, destEnd - curDest, dictEnd - ((uint8_t *)dest - matchSrc), len);
                curDest += len;
            } else { // 匹配对一些在exteral字典中，一些在src头部
                size_t externCopySize = (size_t)((uint8_t *)dest - matchSrc); // 外部字典需要复制的长度
                size_t innerCopySize = len - externCopySize; // src需要复制的长度
                err = memcpy_s(curDest, destEnd - curDest, dictEnd - externCopySize, externCopySize);
                curDest += externCopySize;
                if (innerCopySize > (size_t)(curDest - (uint8_t *)dest)) {
                    const uint8_t *copySrc = (uint8_t *)dest; // 出现内存重叠时，比如向前偏移6个字节，有8个字节匹配这种二元组:12345612345612
                    while ((innerCopySize--) != 0) {
                        *curDest++ = *copySrc++;
                    }
                } else { // 无内存重叠
                    err = memcpy_s(curDest, destEnd - curDest, (uint8_t *)dest, innerCopySize);
                    curDest += innerCopySize;
                }
            }
#ifdef RAPIDLZ_DEBUG
            RAPIDLZ_RETURN_IF_NOT_EOK(err, RAPIDLZ_DEC_NOT_OK);
#else /* RAPIDLZ_DEBUG */
            (void)err;
#endif /* RAPIDLZ_DEBUG */
        }
    }

    return (int)((void *)curDest - (void *)(dest));
}

int RapidlzCompressStream(RapidlzStreamCtx* strmCtx, const char* src, char* dst, int srcSize, int dstSize)
{
    RapidlzCompressFunc rapidlzEncFunc = NULL;
    RAPIDLZ_RETURN_IF_NOT_TRUE(!(srcSize > RAPIDLZ_MAX_INPUT_SIZE), RAPIDLZ_ENC_NOT_OK);
    RAPIDLZ_RETURN_IF_NOT_TRUE(!((src == NULL && srcSize != 0) || (dstSize <= 0) || (dst == NULL)),
        RAPIDLZ_ENC_NOT_OK);
    if (srcSize == 0) { // 0字节压缩
        return RapidlzZeroBytesEncode(dst, dstSize);
    }
    char* dictEnd = (strmCtx->dictSize != 0) ? (char*)strmCtx->dict + strmCtx->dictSize : NULL;
    int cSize; // compressed size
    if (dictEnd == src) { /* prefix mode 包含 no dict mode */
        rapidlzEncFunc = RapidlzCompWithPrefixDict;
    } else { /* external mode 包含 specific mode */
        if (strmCtx->strmCtxSpecific != NULL) { // 高优先级，流式压缩过程中不会改变的常驻字典
            RAPIDLZ_RETURN_IF_NOT_EOK(memcpy_s(strmCtx,
                sizeof(RapidlzStreamCtx), strmCtx->strmCtxSpecific, sizeof(RapidlzStreamCtx)), RAPIDLZ_ENC_NOT_OK);
        } // 访问一层指针快一点
        rapidlzEncFunc = RapidlzCompWithExternalDict;
    }
    RapidlzStrmCtxNorm(strmCtx, (char *)src, srcSize, dictEnd);
    cSize = rapidlzEncFunc(strmCtx, src, dst, srcSize, dstSize);
    strmCtx->dictSize = srcSize; /* stream store */
    strmCtx->dict = (uint8_t *)src;
    return cSize;
}

int RapidlzDecompressSafeUsingDict(const char *src, char *dst, int compressedSize, int dstSize,
    const char *dictStart, int dictSize)
{
    if ((src == NULL) || (compressedSize == 0) || (dst == NULL) || (dstSize < 0)) {
        return RAPIDLZ_ERROR_PARAM_UNSUPPORTED;
    }
    if (RAPIDLZ_UNLIKELY(dstSize == 0)) {
        return RapidlzZeroBytesDecode(src, compressedSize);
    }
    RapidlzDecompressFunc rapidlzDecFunc;
    if ((dictSize == 0) || (dictStart + dictSize == dst)) {
        rapidlzDecFunc = RapidlzDecWithPrefixDict;
    } else {
        rapidlzDecFunc = RapidlzDecWithExternalDict;
    }
    return rapidlzDecFunc(src, dst, compressedSize, dstSize, dictStart, dictSize);
}

#ifdef __cplusplus
}
#endif