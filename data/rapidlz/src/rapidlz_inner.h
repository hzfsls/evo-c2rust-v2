/*
 * @file rapidlz_inner.h
 * Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
 * Description: compress function description
 * Author: Anonym
 * Create: 2022-06-24
 */

#ifndef RAPIDLZ_INNER_H
#define RAPIDLZ_INNER_H

#include <stdint.h>
#include <stdlib.h>
#include <assert.h>
#include "securec.h"
#include "rapidlz_log.h"

#if defined(__SSE2__) || defined(_M_AMD64)
#define X86_SSE2
#endif

#if defined(X86_SSE2)
#include <emmintrin.h>
#endif

#if defined(__ARM_NEON) || defined(_M_ARM64)
#define ARM_NEON
#endif

#if defined(ARM_NEON)
#include <arm_neon.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

#define RAPIDLZ_MAX_BYTE_VALUE 255   /* 一个byte可以表示的最大数目 */
#define RAPIDLZ_MAX_4BIT_VALUE 15    /* 4个bit可以表示的最大数目 */
#define RAPIDLZ_MIN_MATCH 4          /* 最小math长度，默认为4字节 */
#define RAPIDLZ_HASH_TYPE_4 4                  /* 取4字节hash */
#define RAPIDLZ_HASH_TYPE_5 5                  /* 取5字节hash */
#define RAPIDLZ_STEP_FORWARD_BASE 6            /* 当总是找不到匹配时，加大查找的步长 */
#define RAPIDLZ_MAX_OFFSET 65535               /* 最大offset距离 */

/****************************************错误码仅记录在日志中******************************************/

/**
* @ingroup rapidlz
* 输入非法
*/
#define RAPIDLZ_INPUT_INVALID (size_t)(-100)

/**
* @ingroup rapidlz
* 申请内存失败
*/
#define RAPIDLZ_MALLOC_FAILED (size_t)(-99)

/**
* @ingroup rapidlz
* dst内存大小不够
*/
#define RAPIDLZ_DST_SIZE_SMALL (size_t)(-98)

/**
* @ingroup rapidlz
* 安全函数库出错
*/
#define RAPIDLZ_SECUREC_ERROR (size_t)(-97)

/**
* @ingroup rapidlz
* lz格式出错
*/
#define RAPIDLZ_FORMAT_INVALID (size_t)(-96)

/**/

// #if defined(__GNUC__) && (__GNUC__ >= 4)
// #define RAPIDLZ_ALWAYS_INLINE inline __attribute__((always_inline))
// #else
// #define RAPIDLZ_ALWAYS_INLINE inline
// #endif

/**
 * @ingroup rapidlz
 * 预测分支可能走向，帮助编译器优化编译，减少代码的分支预测失败时的可能开销。
 */
#define RAPIDLZ_LIKELY(x) (__builtin_expect(!!(x), 1))

/**
 * @ingroup rapidlz
 * 预测分支不可能走向，帮助编译器优化编译，减少代码的分支预测失败时的可能开销。
 */
#define RAPIDLZ_UNLIKELY(x) (__builtin_expect(!!(x), 0))

/*
* @ingroup rapidlz
 * @brief struct to read or write unaligned addr
*/
// #if defined(_MSC_VER) || (defined(__INTEL_COMPILER) && defined(WIN32))
// typedef struct { uint16_t v; } RapidlzUnalignU16;
//     typedef struct { uint32_t v; } RapidlzUnalignU32;
//     typedef struct { uint64_t v; } RapidlzUnalignU64;
//     typedef struct { int64_t v; } RapidlzUnalign64;
// #else
typedef struct { uint16_t v; } __attribute__((packed)) RapidlzUnalignU16;
typedef struct { uint32_t v; } __attribute__((packed)) RapidlzUnalignU32;
typedef struct { uint64_t v; } __attribute__((packed)) RapidlzUnalignU64;
// #endif

#define RAPIDLZ_READ16BIT(ptr) (((const RapidlzUnalignU16 *)(ptr))->v)
#define RAPIDLZ_READ32BIT(ptr) (((const RapidlzUnalignU32 *)(ptr))->v)
#define RAPIDLZ_READ64BIT(ptr) (((const RapidlzUnalignU64 *)(ptr))->v)
#define RAPIDLZ_WRITE64BIT(ptr, val) (((RapidlzUnalignU64 *)(ptr))->v = (val))

/**
 * @ingroup rapidlz
 * 内部断言
 */
#define RAPIDLZ_ASSERT(x) assert(x)

/**
 * @ingroup rapidlz
 * @brief 判断当前机器是否是小端
 * @retval != 0 是小端
 * @retval == 0 是大端
 */
static RAPIDLZ_ALWAYS_INLINE int RapidlzIsLE(void)
{
#if (defined(__GNUC__) || defined(__clang__))
    return __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__;
#endif
    int n = 1;
    return *(char *)(&n);
}

/**
 * @ingroup rapidlz
 * @brief 以小端方式读取当前地址2字节的值
 * @param addr   [IN] 读取的地址
 * @retval 小端2字节值
 */
static RAPIDLZ_ALWAYS_INLINE uint16_t RapidlzReadLE16Bit(const void *addr)
{
    if (RapidlzIsLE() != 0) {
        return *(const uint16_t *)addr;
    }

    uint8_t tmp1 = ((const uint8_t *)addr)[0];
    uint8_t tmp2 = ((const uint8_t *)addr)[1];
    return (uint16_t)(tmp1 + (tmp2 << 8)); // 左移8位和8位地址组成16位
}

/**
 * @ingroup rapidlz
 * @brief 获取当前8字节值的末尾0的个数,注意：输入不能为0否则输出不定
 * @param x    [IN] 64位值，不能为0
 * @retval 末尾0的个数
 */
static RAPIDLZ_ALWAYS_INLINE uint8_t RapidlzCountTailZero64(uint64_t x)
{
#if (defined(__GNUC__) && (__GNUC__ >= 3))
    return (uint8_t)__builtin_ctzll(x);
#endif
    if (x == 0) {
        return 0;
    }
    uint64_t val = x;
    uint8_t num = 0;
    while ((val & 1) == 0) {
        val >>= 1;
        num++;
    }
    return num;
}

/**
 * @ingroup rapidlz
 * @brief 获取当前8字节值的前导0的个数,注意：输入不能为0否则输出不定
 * @param x    [IN] 64位值，不能为0
 * @retval 前导0的个数
 */
static RAPIDLZ_ALWAYS_INLINE uint8_t RapidlzCountLeadZero64(uint64_t x)
{
#if (defined(__GNUC__) && (__GNUC__ >= 3))
    return (uint8_t)__builtin_clzll(x);
#endif
    if (x == 0) {
        return 0;
    }
    uint8_t num = 0;
    uint64_t val = x;
    while ((val & 0x8000000000000000ULL) == 0) {
        val <<= 1;
        num++;
    }
    return num;
}

/**
 * @ingroup rapidlz
 * @brief 获取当前8字节值的前导1的位数,注意：输入不能为0否则输出不定
 * @param x    [IN] 64位值，不能为0
 * @retval 前导1的位数
 */
static RAPIDLZ_ALWAYS_INLINE uint8_t RapidlzHighBit64(uint64_t x)
{
    RAPIDLZ_ASSERT(x != 0);

#if (defined(__GNUC__) && (__GNUC__ >= 3))
    return ((uint8_t)__builtin_clzll(x) ^ 63); // 获得前导0的个数后异或63就是前导1的位数
#endif
    uint8_t pos = 64;
    uint64_t value = x;

    if (value == 0) {
        return 0;
    }
    if ((value & 0xFFFFFFFF00000000) == 0) {
        value <<= 32; // 左移32位取低位
        pos -= 32;
    }
    if ((value & 0xFFFF000000000000) == 0) {
        value <<= 16; // 左移16位取低位
        pos -= 16;
    }
    if ((value & 0xFF00000000000000) == 0) {
        value <<=  8; // 左移8位取低位
        pos -= 8;
    }
    if ((value & 0xF000000000000000) == 0) {
        value <<=  4; // 左移4位取低位
        pos -= 4;
    }
    if ((value & 0xC000000000000000) == 0) {
        value <<=  2; // 左移2位取低位
        pos -= 2;
    }
    if ((value & 0x8000000000000000) == 0) {
        value <<=  1;
        pos -= 1;
    }

    return pos - 1;
}

/**
 * @ingroup rapidlz
 * @brief 以小端2字节值写入
 * @param addr  [IN] 需要写入的地址
 * @param val   [IN] 写入的无符号16位值
 * @retval
 */
static RAPIDLZ_ALWAYS_INLINE void RapidlzWriteLE16(void *addr, uint16_t val)
{
    if (RapidlzIsLE() != 0) {
        *(uint16_t *)addr = val;
    } else {
        uint8_t *tmp = (uint8_t *)addr;
        tmp[0] = (uint8_t)val;
        tmp[1] = (uint8_t)(val >> 8); // 右移8位获得高8位值
    }
}

/**
 * @ingroup rapidlz
 * @brief 指令加速拷贝32byte内容，不区分大小端
 * @attention dst和src不能有重叠！否则结果出错
 * @param dst  [IN] 写入地址
 * @param src  [IN] 拷贝源数据地址
 * @retval
 */
static RAPIDLZ_ALWAYS_INLINE void RapidlzCopy32Byte(void *dst, const void *src)
{
#if defined(ARM_NEON)
    // 此处使用vst1q要比vst2q要好
    vst1q_u8((uint8_t *)dst, vld1q_u8((const uint8_t *)src));
    vst1q_u8((uint8_t *)dst + 16, vld1q_u8((const uint8_t *)src + 16));
#elif defined(X86_SSE2)
    _mm_storeu_si128((__m128i *)dst, _mm_loadu_si128((const __m128i *)src));
    _mm_storeu_si128((__m128i *)dst + 1, _mm_loadu_si128((const __m128i *)src + 1));
#else
    RAPIDLZ_WRITE64BIT(dst, RAPIDLZ_READ64BIT(src));
    RAPIDLZ_WRITE64BIT((uint8_t *)dst + 8, RAPIDLZ_READ64BIT((uint8_t *)src + 8));
    RAPIDLZ_WRITE64BIT((uint8_t *)dst + 16, RAPIDLZ_READ64BIT((uint8_t *)src + 16));
    RAPIDLZ_WRITE64BIT((uint8_t *)dst + 24, RAPIDLZ_READ64BIT((uint8_t *)src + 24));
#endif
}

/**
 * @ingroup rapidlz
 * @brief 指令加速拷贝16byte内容，不区分大小端
 * @attention dst和src不能有重叠！否则结果出错
 * @param dst  [IN] 写入地址
 * @param src  [IN] 拷贝源数据地址
 * @retval
 */
static RAPIDLZ_ALWAYS_INLINE void RapidlzCopy16Byte(void *dst, const void *src)
{
#if defined(ARM_NEON)
    vst1q_u8((uint8_t *)dst, vld1q_u8((const uint8_t *)src));
#elif defined(X86_SSE2)
    _mm_storeu_si128((__m128i *)dst, _mm_loadu_si128((const __m128i *)src));
#else
    RAPIDLZ_WRITE64BIT(dst, RAPIDLZ_READ64BIT(src));
    RAPIDLZ_WRITE64BIT((uint8_t *)dst + 8, RAPIDLZ_READ64BIT((uint8_t *)src + 8));
#endif
}

/**
 * @ingroup rapidlz
 * @brief 指令加速拷贝8byte内容，不区分大小端
 * @attention dst和src不能有重叠！否则结果出错
 * @param dst  [IN] 写入地址
 * @param src  [IN] 拷贝源数据地址
 * @retval
 */
static RAPIDLZ_ALWAYS_INLINE void RapidlzCopy8Byte(void *dst, const void *src)
{
#if defined(ARM_NEON)
    vst1_u8((uint8_t*)dst, vld1_u8((const uint8_t*)src));
#else
    RAPIDLZ_WRITE64BIT(dst, RAPIDLZ_READ64BIT(src));
#endif
}

static RAPIDLZ_ALWAYS_INLINE void RapidlzWildCopy8(const uint8_t *srcPtr, uint8_t *dstPtr, uint8_t *dstEnd)
{
    uint8_t *tmpDstPtr = dstPtr;
    const uint8_t *tmpSrcPtr = srcPtr;
    do {
        RapidlzCopy8Byte(tmpDstPtr, tmpSrcPtr);
        tmpDstPtr += 8; // 偏移8字节
        tmpSrcPtr += 8; // 偏移8字节
    } while (tmpDstPtr < dstEnd);
}

static RAPIDLZ_ALWAYS_INLINE void RapidlzWildCopy16(const uint8_t *srcPtr, uint8_t *dstPtr, uint8_t *dstEnd)
{
    uint8_t *tmpDstPtr = dstPtr;
    const uint8_t *tmpSrcPtr = srcPtr;
    do {
        RapidlzCopy16Byte(tmpDstPtr, tmpSrcPtr);
        tmpDstPtr += 16; // 偏移16字节
        tmpSrcPtr += 16; // 偏移16字节
    } while (tmpDstPtr < dstEnd);
}

static RAPIDLZ_ALWAYS_INLINE void RapidlzWildCopy32(const uint8_t *srcPtr, uint8_t *dstPtr, uint8_t *dstEnd)
{
    uint8_t *tmpDstPtr = dstPtr;
    const uint8_t *tmpSrcPtr = srcPtr;
    do {
        RapidlzCopy32Byte(tmpDstPtr, tmpSrcPtr);
        tmpDstPtr += 32; // 偏移32字节
        tmpSrcPtr += 32; // 偏移32字节
    } while (tmpDstPtr < dstEnd);
}

// 匹配向前扩展宏函数定义
#define RAPIDLZ_EXPAND_FORWARD(srcBegin, matchBegin, srcCurr, srcAnchor) do {                                      \
    while ((srcBegin) < (matchBegin) && (srcCurr) > (srcAnchor) &&                                                 \
        RAPIDLZ_UNLIKELY((matchBegin)[-1] == (srcCurr)[-1])) {                                                     \
        (matchBegin)--;                                                                                            \
        (srcCurr)--;                                                                                               \
    }                                                                                                              \
} while (0)

#define RAPIDLZ_FOUR_BYTE 4
#define RAPIDLZ_EIGHT_BYTE 8
#define RAPIDLZ_SIXTEEN_BYTE 16
#define RAPIDLZ_COPY_PROTECT_SIZE 16  /* 确保拷贝不越界的大小 */

#define RAPIDLZ_READ_OPTIONAL_LENGTH(len, srcCurr, srcEnd, temp) do {    \
    if (RAPIDLZ_LIKELY((srcCurr) < (srcEnd))) {                          \
        (temp) = *(srcCurr)++;                                   \
        (len) += (temp);                                         \
    }                                                            \
    while (((temp) == RAPIDLZ_MAX_BYTE_VALUE) && (srcCurr) < (srcEnd)) { \
        (temp) = *(srcCurr)++;                                   \
        (len) += (temp);                                         \
    }                                                            \
} while (0)

#define SAFE_COPY_MATCH(dstCurr, matchSrc, matchLength) do { \
    while ((matchLength)-- > 0) {                            \
        *(dstCurr)++ = *(matchSrc)++;                        \
    }                                                        \
} while (0)

static RAPIDLZ_ALWAYS_INLINE void RapidlzCopyLiteralsFast(const uint8_t *src, uint8_t *dst, uint32_t length)
{
    if (RAPIDLZ_LIKELY(length <= RAPIDLZ_SIXTEEN_BYTE)) { // 小于16拷贝一次
        RapidlzCopy16Byte(dst, src);
        return;
    }

    RapidlzWildCopy16(src, dst, dst + length);
}

#define RAPIDLZ_FALLTHROUGH __attribute__((fallthrough))

// 向后匹配扩展
static RAPIDLZ_ALWAYS_INLINE const uint8_t *RapidlzCompressExpandBackward(const uint8_t *matchLimit,
    const uint8_t *matchPtr, const uint8_t *srcCurr)
{
    uint64_t xorVal;
    const uint8_t *loopEnd = matchLimit - 7;  /* -7是防止向后看8字节越界 */
    const uint8_t *srcCurrMatchEnd = srcCurr;
    const uint8_t *matchBegin = matchPtr;

    while (srcCurrMatchEnd < loopEnd) { // 如果匹配长度大于8,先8个字节找
        xorVal = RAPIDLZ_READ64BIT(matchBegin) ^ RAPIDLZ_READ64BIT(srcCurrMatchEnd);
        if (RAPIDLZ_UNLIKELY(xorVal == 0)) {
            srcCurrMatchEnd += sizeof(uint64_t);
            matchBegin += sizeof(uint64_t);
            continue;
        }
        srcCurrMatchEnd += RapidlzIsLE() ?
        (RapidlzCountTailZero64(xorVal) >> 3) : (RapidlzCountLeadZero64(xorVal) >> 3); // 右移3 除8获得个数
        return srcCurrMatchEnd;
    }

    if (((srcCurrMatchEnd + 3) < matchLimit) && /* +3确保不越界 */
        (RAPIDLZ_READ32BIT(srcCurrMatchEnd) == RAPIDLZ_READ32BIT(matchBegin))) {
        srcCurrMatchEnd += sizeof(uint32_t);
        matchBegin += sizeof(uint32_t);
    }

    if (((srcCurrMatchEnd + 1) < matchLimit) &&
        (RAPIDLZ_READ16BIT(srcCurrMatchEnd) == RAPIDLZ_READ16BIT(matchBegin))) { // +1确保不越界
        srcCurrMatchEnd += sizeof(uint16_t);
        matchBegin += sizeof(uint16_t);
    }

    if ((srcCurrMatchEnd < matchLimit) && (srcCurrMatchEnd[0] == matchBegin[0])) {
        srcCurrMatchEnd++;
    }
    return srcCurrMatchEnd;
}

static uint8_t g_overlapOffAddVal[] = {0, 1, 2, 2, 4, 3, 2, 1};

static RAPIDLZ_ALWAYS_INLINE void RapidlzCopyMatchFast(uint8_t *dst, uint8_t *match, uint16_t offset, uint32_t length)
{
    uint8_t *dstCurr = dst;
    uint8_t *matchPtr = match;

    if (offset >= RAPIDLZ_SIXTEEN_BYTE) {
        RapidlzCopyLiteralsFast(matchPtr, dstCurr, length);
        return;
    }

    for (int i = 0; i < RAPIDLZ_EIGHT_BYTE; ++i) { // 直接先写8个，此处需要一个一个字节写
        dstCurr[i] = matchPtr[i];
    }

    if (length <= RAPIDLZ_EIGHT_BYTE) {
        return;
    }

    uint8_t *dstEnd = dstCurr + length;
    if (offset < RAPIDLZ_EIGHT_BYTE) { // offset < 8 说明重叠了
        matchPtr += g_overlapOffAddVal[offset];
        dstCurr += RAPIDLZ_EIGHT_BYTE;
    }

    do {
        RapidlzCopy8Byte(dstCurr, matchPtr);
        dstCurr += RAPIDLZ_EIGHT_BYTE;
        matchPtr += RAPIDLZ_EIGHT_BYTE;
    } while (dstCurr < dstEnd);
}

/**
 * @ingroup rapidlz
 * @brief 对外压缩接口
 * @par 描述: 提供压缩功能
 * @attention 无
 * @param dst: 输出buffer, 用来存放压缩好的数据流；
 * @param dstSize: dst中留存的内存空间,单位为Byte;
 * @param src: 待压缩数据流;
 * @param srcSize: 待压缩 数据流大小，单位为Byte;
 * @retval 默认压缩等级为1，返回压缩后的数据大小，值大于0说明压缩成功，否则失败
 */
size_t RapidlzCompressDefault(const void *src, void *dst, size_t srcSize, size_t dstSize);

/******************************************获取版本号接口对外说明******************************************/
/**
 * @ingroup rapidlz
 * @brief 获取rapidlz版本号
 * @par 描述: 获取字符串类型的rapidlz版本号
 * @attention 无
 * @retval rapidlz版本号，格式为“rapidlz x.x.x"
 */
const char *RapidlzVersionGet(void);

#ifdef __cplusplus
}
#endif

#endif