/**
 * @file cmptlz_utils.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 公共基础工具库
 * @author Anonym
 * @date 2024-01-09
 */
#ifndef CMPTLZ_UTILS_H
#define CMPTLZ_UTILS_H

#include <stdint.h>
#include "cmptlz_def.h"

#ifdef __cplusplus
extern "C" {
#endif

#if defined(_MSC_VER) || (defined(__INTEL_COMPILER) && defined(WIN32))
typedef struct { uint32_t v; } CmptlzUnalignU32;
#else
typedef struct { uint32_t v; } __attribute__((packed)) CmptlzUnalignU32;
#endif

#define CMPTLZ_WRITE32BIT(ptr, val) (((CmptlzUnalignU32 *)(ptr))->v = (val))

#define CMPTLZ_READ32BIT(ptr) (((const CmptlzUnalignU32 *)(ptr))->v)

/**
 * @ingroup cmptlz
 * @brief 判断当前机器是否是小端
 * @retval !=0 是小端
 * @retval ==0 是大端
 */
static ALWAYS_INLINE int CmptlzIsLE(void)
{
#if (defined(__GNUC__) || defined(__clang__))
    return __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__;
#endif
    int n = 1;
    return *(char *)(&n);
}

/**
 * @ingroup cmptlz
 * @brief 4字节值大小端转换
 * @param val   [IN] 需要转换的值
 * @retval 小端4字节值
 */
static ALWAYS_INLINE uint32_t CmptlzSwap32(uint32_t val)
{
#if (defined(__GNUC__) && (__GNUC__ > 4 || (__GNUC__ == 4 && __GNUC_MINOR__ >= 2)))
    return (uint32_t)__builtin_bswap32(val);
#endif
    return ((0xff000000 & (val << 24)) | // 左移24位和右移24位值互换
            (0x000000ff & (val >> 24)) |
            (0x00ff0000 & (val <<  8)) | // 高8位和低8位值互换
            (0x0000ff00 & (val >>  8)));
}

/**
 * @ingroup cmptlz
 * @brief 以小端4字节值写入
 * @param addr  [IN] 需要写入的地址
 * @param val   [IN] 写入的无符号32位值
 * @retval 无
 */
static ALWAYS_INLINE void CmptlzWriteLE32Bit(void *addr, uint32_t val)
{
    if (CmptlzIsLE() != 0) {
        CMPTLZ_WRITE32BIT(addr, val);
    } else {
        CMPTLZ_WRITE32BIT(addr, CmptlzSwap32(val));
    }
}

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* CMPTLZ_UTILS_H */