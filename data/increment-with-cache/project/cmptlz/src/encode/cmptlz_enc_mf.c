/**
 * @file cmptlz_enc_mf.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 压缩Match Finder最长匹配文件
 * @author Anonym
 * @date 2024-01-09
 */

#include "cmptlz_enc_inner.h"

#define CMPT_EMPTY_HASH_VALUE 0
#define CMPTLZ_HASH_2_SIZE (1 << 10)
#define CMPTLZ_HASH_3_SIZE (1 << 16)
#define CMPTLZ_HASH_4_SIZE (1 << 20)
#define CMPTLZ_HASH_2_MASK (CMPTLZ_HASH_2_SIZE - 1)
#define CMPTLZ_HASH_3_MASK (CMPTLZ_HASH_3_SIZE - 1)
#define CMPTLZ_FIX_3_HASH (CMPTLZ_HASH_2_SIZE)
#define CMPTLZ_FIX_4_HASH (CMPTLZ_HASH_2_SIZE + CMPTLZ_HASH_3_SIZE)

#define CMPT_HASH_MASK_CALC(hashMask) \
    do { \
        hashMask |= hashMask >> 1; \
        hashMask |= hashMask >> 2; \
        hashMask |= hashMask >> 4; \
        hashMask |= hashMask >> 8; \
        hashMask >>= 1; \
        hashMask |= 0xFFFF; \
        if (hashMask > (1 << 24)) { \
            hashMask >>= 1; \
        } \
    } while (0)

#define CMPT_HASH_4_CALC(mf, cur, temp, hash2Value, hash3Value, hashValue) \
    do { \
        temp = mf->hashRootTable[cur[0]] ^ cur[1]; \
        hash2Value = temp & CMPTLZ_HASH_2_MASK; \
        hash3Value = (temp ^ ((uint32_t)(cur[2]) << 8)) & CMPTLZ_HASH_3_MASK; \
        hashValue = (temp ^ ((uint32_t)(cur[2]) << 8) ^ \
            (mf->hashRootTable[cur[3]] << 5)) & mf->hashMask; \
    } while (0)

#define CMPT_HASH_UPDATE(mf, hash2Value, hash3Value, hashValue, pos) \
    do { \
        mf->hash[hash2Value] = pos; \
        mf->hash[CMPTLZ_FIX_3_HASH + hash3Value] = pos; \
        mf->hash[CMPTLZ_FIX_4_HASH + hashValue] = pos; \
    } while (0)

// 两字节匹配，记录于matches[0]
#define CMPT_HASH_FIND_2_BYTES(mf, delta2, longestLen, matchesCount, cur, matches) \
    do { \
        if (delta2 < mf->cycleSize && *(cur - delta2) == *cur) { \
            longestLen = CMPT_MF_MATCH_2_BYTES; \
            matches[0].len = CMPT_MF_MATCH_2_BYTES; \
            matches[0].dist = delta2 - 1; \
            matchesCount = 1; \
        } \
    } while (0)

 // 三字节匹配，记录于matches[1]
#define CMPT_HASH_FIND_3_BYTES(mf, delta2, delta3, longestLen, matchesCount, cur, matches) \
    do { \
        if (delta2 != delta3 && delta3 < mf->cycleSize && *(cur - delta3) == *cur) { \
            longestLen = CMPT_MF_MATCH_3_BYTES; \
            matches[matchesCount++].dist = delta3 - 1; \
            delta2 = delta3; \
        } \
    } while (0)

#define CMPT_MF_MOVE_POS(mf) \
    do { \
        mf->readPos++; \
        mf->cyclePos++; \
        mf->cyclePos = (mf->cyclePos == mf->cycleSize) ? 0 : mf->cyclePos; \
        if (CMPTLZ_UNLIKELY(mf->readPos + mf->offset == CMPTLZ_UINT32_MAX)) { \
            CmptMfMovePos(mf); \
        } \
    } while (0)

#define CMPT_MF_LEFT_SON_UPDATE(ptr1, pair, curMatch, len1, len) \
    do { \
        *ptr1 = curMatch; \
        ptr1 = pair + 1; \
        curMatch = *ptr1; \
        len1 = len; \
    } while (0)

#define CMPT_MF_RIGHT_SON_UPDATE(ptr0, pair, curMatch, len0, len) \
    do { \
        *ptr0 = curMatch; \
        ptr0 = pair; \
        curMatch = *ptr0; \
        len0 = len; \
    } while (0)

static void CmptlzMfGenHashTable(CmptMfCtx *mf)
{
    uint32_t *hashRootTable = mf->hashRootTable;
    const uint32_t poly32 = 0xEDB88320;
    uint32_t i, j;
    for (i = 0; i < CMPT_MF_HASH_TABLE_SIZE; i++) {
        uint32_t value = i;
        for (j = 0; j < 8; j++) { // 8位
            if (value & 1) {
                value = (value >> 1) ^ poly32;
            } else {
                value >>= 1;
            }
        }
        hashRootTable[i] = value;
    }
    return;
}

int CmptMfPrepare(CmptLzEncCtx *encCtx, const uint8_t *src, size_t srcLen, CmptLzMemHook *alloc)
{
    /* alloc */
    CmptMfCtx *mf = alloc->CmptLzAlloc(CMPTLZ_MF_CCTX_HANDLE, sizeof(CmptMfCtx));
    if (mf == NULL) {
        return CMPT_ENC_MF_INIT_FAIL;
    }
    memset_s(mf, sizeof(CmptMfCtx), 0, sizeof(CmptMfCtx));
    /* passing */
    encCtx->mfCtx = mf;
    mf->cycleSize = encCtx->dicSize + 1;
    uint32_t hashMask = encCtx->dicSize - 1;
    CMPT_HASH_MASK_CALC(hashMask);
    mf->hashMask = hashMask;
    ++hashMask;
    hashMask += CMPTLZ_HASH_2_SIZE;
    hashMask += CMPTLZ_HASH_3_SIZE;
    mf->hashCount = hashMask;
    mf->sonsCount = mf->cycleSize * 2; // 2倍cycleSize
    mf->hash = NULL;
    mf->son = NULL;
    mf->hash = alloc->CmptLzAlloc(CMPTLZ_MF_HASH_HANDLE, mf->hashCount * sizeof(uint32_t));
    memset_s(mf->hash, mf->hashCount * sizeof(uint32_t), 0, mf->hashCount * sizeof(uint32_t));
    if (mf->hash == NULL) {
        return CMPT_ENC_MF_INIT_FAIL;
    }
    mf->son = alloc->CmptLzAlloc(CMPTLZ_MF_SON_HANDLE, mf->sonsCount * sizeof(uint32_t));
    memset_s(mf->son, mf->sonsCount * sizeof(uint32_t), 0, mf->sonsCount * sizeof(uint32_t));
    if (mf->son == NULL) {
        return CMPT_ENC_MF_INIT_FAIL;
    }
    /* init */
    CmptlzMfGenHashTable(mf);
    mf->srcStart = src;
    mf->srcLen = srcLen;
    mf->offset = mf->cycleSize;
    mf->niceLen = encCtx->numFastBytes; // readAhead cyclePos mfStart readPos 已初始化为0;
    mf->depth = CMPT_MF_BASE_DEPTH + mf->niceLen / 2; // 除2
    return 0;
}

static void CmptMfMovePos(CmptMfCtx *mf)
{
    const uint32_t subvalue = (CMPTLZ_UINT32_MAX - mf->cycleSize);
    uint32_t i;
    for (i = 0; i < mf->hashCount; i++) {
        if (mf->hash[i] <= subvalue) {
            mf->hash[i] = CMPT_EMPTY_HASH_VALUE;
        } else {
            mf->hash[i] -= subvalue;
        }
    }
    for (i = 0; i < mf->sonsCount; ++i) {
        if (mf->son[i] <= subvalue) {
            mf->son[i] = CMPT_EMPTY_HASH_VALUE;
        } else {
            mf->son[i] -= subvalue;
        }
    }
    mf->offset -= subvalue;
}

static CmptlzMatchPair *CmptBtFind(CmptMfCtx *mf, uint32_t curMatch,
    CmptlzMatchPair *matches, uint32_t longestLen)
{
    /* son数组保存当前位置与上一个hash值相同且满足下列关系的位置:
    左节点存放position 1：字符串比较buf + pos1 <  buf + cur_pos
    右节点存放position 2：字符串比较buf + pos2 >= buf + cur_pos */
    uint32_t depth = mf->depth;
    uint32_t *const son = mf->son;
    const uint8_t *cur = (const uint8_t *)(mf->srcStart + mf->readPos);
    const uint32_t niceLen = mf->niceLen;
    const uint32_t cyclePos = mf->cyclePos;
    const uint32_t cycleSize = mf->cycleSize;
    const uint32_t pos = mf->readPos + mf->offset;
    uint32_t *ptr0 = son + (cyclePos << 1) + 1; // 右
    uint32_t *ptr1 = son + (cyclePos << 1); // 左
    uint32_t len0 = 0;
    uint32_t len1 = 0;

    while (true) {
        const uint32_t delta = pos - curMatch; // 这里delta可能大于字典长度，先判断下
        if (depth-- == 0 || delta >= cycleSize) { // 跳出条件：查找深度超过指定的最大查找深度 或 匹配距离大于字典长度
            *ptr0 = CMPT_EMPTY_HASH_VALUE;
            *ptr1 = CMPT_EMPTY_HASH_VALUE;
            return matches;
        }
        uint32_t *const pair = son + ((cyclePos - delta + ((delta > cyclePos) ? cycleSize : 0)) << 1);
        const uint8_t *const pb = cur - delta;
        uint32_t len = CMPTLZ_FIND_MIN(len0, len1);
        if (pb[len] == cur[len]) {
            len = CmptMemCmpLenSafe(pb, cur, len + 1, niceLen);
            if (longestLen < len) { // 出现新的最长匹配,更新
                longestLen = len;
                matches->len = len;
                matches->dist = delta - 1;
                ++matches;
                if (len == niceLen) { // 跳出条件：直到长度达到niceLen
                    *ptr1 = pair[0];
                    *ptr0 = pair[1];
                    return matches;
                }
            }
        }
        if (pb[len] < cur[len]) {
            CMPT_MF_LEFT_SON_UPDATE(ptr1, pair, curMatch, len1, len);
        } else {
            CMPT_MF_RIGHT_SON_UPDATE(ptr0, pair, curMatch, len0, len);
        }
    }
}

static void CmptBtSkip(CmptMfCtx *mf, const uint32_t lenLimit,
    const uint32_t pos, const uint8_t *const cur, uint32_t curMatch)
{
    uint32_t depth = mf->depth;
    uint32_t *const son = mf->son;
    const uint32_t cyclePos = mf->cyclePos;
    const uint32_t cycleSize = mf->cycleSize;

    uint32_t *ptr0 = son + (cyclePos << 1) + 1;
    uint32_t *ptr1 = son + (cyclePos << 1);
    uint32_t len0 = 0;
    uint32_t len1 = 0;

    while (true) {
        const uint32_t delta = pos - curMatch;
        if (depth-- == 0 || delta >= cycleSize) {
            *ptr0 = CMPT_EMPTY_HASH_VALUE;
            *ptr1 = CMPT_EMPTY_HASH_VALUE;
            return;
        }
        uint32_t *pair = son + ((cyclePos - delta + ((delta > cyclePos) ? cycleSize : 0)) << 1);
        const uint8_t *pb = cur - delta;
        uint32_t len = CMPTLZ_FIND_MIN(len0, len1);
        if (pb[len] == cur[len]) {
                len = CmptMemCmpLenSafe(pb, cur, len + 1, lenLimit);
            if (len == lenLimit) {
                *ptr1 = pair[0];
                *ptr0 = pair[1];
                return;
            }
        }
        if (pb[len] < cur[len]) {
            CMPT_MF_LEFT_SON_UPDATE(ptr1, pair, curMatch, len1, len);
        } else {
            CMPT_MF_RIGHT_SON_UPDATE(ptr0, pair, curMatch, len0, len);
        }
    }
}

static uint32_t CmptlzBt4Finder(CmptMfCtx *mf, CmptlzMatchPair *matches)
{
    const uint32_t niceLen = mf->niceLen;
    const uint8_t *cur = (const uint8_t *)(mf->srcStart + mf->readPos);
    const uint32_t pos = mf->readPos + mf->offset;
    uint32_t temp, hash2Value, hash3Value, hashValue;
    uint32_t longestLen = 1, matchesCount = 0;
    CMPT_HASH_4_CALC(mf, cur, temp, hash2Value, hash3Value, hashValue);
    // 哈希表键：一个唯一的hash_value 哈希表值:位置  hash[hashValue] == position
    uint32_t delta2 = pos - mf->hash[hash2Value];
    uint32_t delta3 = pos - mf->hash[CMPTLZ_FIX_3_HASH + hash3Value];
    uint32_t curMatch = mf->hash[CMPTLZ_FIX_4_HASH + hashValue];
    CMPT_HASH_UPDATE(mf, hash2Value, hash3Value, hashValue, pos);
    CMPT_HASH_FIND_2_BYTES(mf, delta2, longestLen, matchesCount, cur, matches);
    CMPT_HASH_FIND_3_BYTES(mf, delta2, delta3, longestLen, matchesCount, cur, matches);

    if (matchesCount != 0) { // 说明当前至少2, 3字节匹配
        longestLen = CmptMemCmpLenSafe(cur, cur - delta2, longestLen, niceLen);
        matches[matchesCount - 1].len = longestLen; // 如果仅有2字节匹配就更新matches[0]，如果有3字节匹配，就更新matches[1]
        if (longestLen == niceLen) { // 找到niceLen快速返回
            CmptBtSkip(mf, niceLen, pos, cur, curMatch);
            CMPT_MF_MOVE_POS(mf);
            return matchesCount;
        }
    }
    // 简单匹配没找到niceLen那么长,进行深度查找
    if (longestLen < CMPT_MF_MATCH_3_BYTES) { // 经过前面的操作，longestLen可能是1,2,3或者进入第三个分支得到更大的值。小值统一为3，大值不变
        longestLen = CMPT_MF_MATCH_3_BYTES;
    } // 2，3字节已经查找过了，深度查找有价值的最短longestLen应为3
    matchesCount = (uint32_t)(CmptBtFind(mf, curMatch, matches + matchesCount, longestLen) - matches);
    // 从curMatch == delta4位置进行深度查找
    CMPT_MF_MOVE_POS(mf);
    return matchesCount;
}

void CmptlzMatchSkiper(CmptMfCtx *mf, uint32_t amount)
{
    mf->readAhead += amount;
    uint32_t pos, temp, hash2Value, hash3Value, hashValue, curMatch;
    const uint32_t niceLen = mf->niceLen;
    do {
        uint32_t lenLimit = mf->srcLen - mf->readPos;
        if (CMPTLZ_LIKELY(niceLen <= lenLimit)) {
            lenLimit = niceLen;
        } else {
            mf->readPos++;
            continue;
        }
        const uint8_t *cur = (const uint8_t *)(mf->srcStart + mf->readPos);
        pos = mf->readPos + mf->offset;
        CMPT_HASH_4_CALC(mf, cur, temp, hash2Value, hash3Value, hashValue);
        curMatch = mf->hash[CMPTLZ_FIX_4_HASH + hashValue];
        CMPT_HASH_UPDATE(mf, hash2Value, hash3Value, hashValue, pos);
        CmptBtSkip(mf, lenLimit, pos, cur, curMatch);
        CMPT_MF_MOVE_POS(mf);
    } while (--amount != 0);
}

uint32_t CmptlzMatchFinder(CmptMfCtx *mf, uint32_t *pCount, CmptlzMatchPair *matches)
{
    if (CMPTLZ_UNLIKELY(mf->srcLen - mf->readPos < mf->niceLen)) { // 文件尾
        *pCount = 0;
        mf->readPos++;
        mf->readAhead++;
        return 0; // 小于niceLen不会做mf
    }
    const uint32_t count = CmptlzBt4Finder(mf, matches); // bt4finder能找到最长为niceLen的匹配
    if (count == 0) {
        *pCount = 0;
        mf->readAhead++;
        return 0;
    }
    uint32_t longestLen = matches[count - 1].len;
    if (longestLen == mf->niceLen) { // 找到niceLen的匹配，就可以尝试找更长的,[niceLen,273]的匹配
        uint32_t bytesAvail = CMPTLZ_FIND_MIN(mf->srcLen - mf->readPos + 1, CMPT_MF_LONGEST_MATCH);
        const uint8_t *p1 = (const uint8_t *)(mf->srcStart + mf->readPos - 1);
        const uint8_t *p2 = p1 - matches[count - 1].dist - 1;
        longestLen = CmptMemCmpLenSafe(p1, p2, longestLen, bytesAvail);
    }
    *pCount = count;
    mf->readAhead++;
    return longestLen;
}