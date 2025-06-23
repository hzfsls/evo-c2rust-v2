/*
 * @file rapidlz_decompress.c
 * Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
 * Description: Decompress function description
 * Author: Anonym
 * Create: 2022-06-24
 */

#include "rapidlz_inner.h"
#include "securec.h"
#include "rapidlz.h"

#ifdef __cplusplus
extern "C" {
#endif

#define RAPIDLZ_MAX_4BIT_MATCH 19     /* 单个token的matchLength阈值 15 + 4 */

// 总的解压缩函数
size_t RapidlzDecompress(const void *src, void *dst, size_t srcSize, size_t dstSize)
{
    if (src == NULL || dst == NULL || srcSize == 0 || dstSize == 0) {
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

    while (srcCurr < srcEnd) {
        token = *srcCurr++;
        litLen = (token >> 4); /* token右移4 */

        if (RAPIDLZ_LIKELY(litLen < RAPIDLZ_MAX_4BIT_VALUE)) { // 此处litLen小于16,fast模式一次拷贝就可以
            if (RAPIDLZ_LIKELY(srcCurr + litLen <= srcEndFast && dstCurr + litLen <= dstEndFast)) {
                RapidlzCopy16Byte(dstCurr, srcCurr);
                dstCurr += litLen;
                srcCurr += litLen;
                goto READ_MATCH;
            }
        } else {
            RAPIDLZ_READ_OPTIONAL_LENGTH(litLen, srcCurr, srcEnd, temp);
            if (RAPIDLZ_LIKELY(srcCurr + litLen <= srcEndFast && dstCurr + litLen <= dstEndFast)) {
                RapidlzWildCopy16(srcCurr, dstCurr, dstCurr + litLen);
                dstCurr += litLen;
                srcCurr += litLen;
                goto READ_MATCH;
            }
        }

        // 此处为不能快速拷贝的分支
        size_t leftSrcSize = srcEnd - srcCurr;
        if (RAPIDLZ_UNLIKELY(litLen > leftSrcSize || memmove_s(dstCurr, dstEnd - dstCurr, srcCurr, litLen) != EOK)) {
            RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "litLen:%u dstEnd - dst:%zu\n", litLen, leftSrcSize);
            return 0;
        }

        dstCurr += litLen;
        srcCurr += litLen;

        if (leftSrcSize == litLen) {
            return dstCurr - (uint8_t *)(dst);
        }

READ_MATCH:
        if (RAPIDLZ_UNLIKELY(srcCurr > srcEnd - 2)) { // RapidlzReadLE16Bit中需要读取2字节
            RAPIDLZ_LOG(RAPIDLZ_FORMAT_INVALID, "rapidlz format invalid\n");
            return 0;
        }
        offset = RapidlzReadLE16Bit(srcCurr);
        srcCurr += 2;                  /* 指针移动2字节offset大小 */
        matchSrc = dstCurr - offset;
        if (RAPIDLZ_UNLIKELY((void *)matchSrc < dst)) { // 匹配到的指针在dst前说明出错
            RAPIDLZ_LOG(RAPIDLZ_FORMAT_INVALID, "rapidlz format invalid\n");
            return 0;
        }

        matchLen = (token & RAPIDLZ_MAX_4BIT_VALUE) + RAPIDLZ_MIN_MATCH; // 读的时候直接加上4字节
        if (matchLen == RAPIDLZ_MAX_4BIT_MATCH) { // 这里是15 + 4 = 19
            RAPIDLZ_READ_OPTIONAL_LENGTH(matchLen, srcCurr, srcEnd, temp);
        }

        if (RAPIDLZ_LIKELY(dstCurr + matchLen <= dstEndFast)) { // 这里根据offset 8字节或16字节拷贝
            RapidlzCopyMatchFast(dstCurr, matchSrc, offset, matchLen);
            dstCurr += matchLen;
        } else {
            if (dstCurr + matchLen > dstEnd) {
                RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "dstEnd - dstCurr:%zu matchLen:%u\n", dstEnd - dstCurr, matchLen);
                return 0;
            }

            SAFE_COPY_MATCH(dstCurr, matchSrc, matchLen);
        }
    }

    return dstCurr - (uint8_t *)dst;
}

#ifdef __cplusplus
}
#endif