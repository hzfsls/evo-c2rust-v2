/**
 * @file cmptlz_def.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 公共宏定义文件
 * @author Anonym
 * @date 2024-01-09
 */

#ifndef CMPTLZ_DEF_H
#define CMPTLZ_DEF_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* inline */
// #if defined(__GNUC__) && (__GNUC__ >= 4)
//     #define ALWAYS_INLINE inline __attribute__((always_inline))
//     #define ALWAYS_NO_INLINE __attribute__((noinline))
// #elif defined(_MSC_VER) && (_MSC_VER >= 1300)
//     #define ALWAYS_NO_INLINE __declspec(noinline)
//     #define ALWAYS_INLINE __forceinline
// #else
//     #define ALWAYS_INLINE inline
//     #define ALWAYS_NO_INLINE
// #endif

// #ifdef CMPTLZ_DEBUG
// #define CMPTLZ_HIDDEN
// #define CMPTLZ_STATIC
// #else
// #define CMPTLZ_HIDDEN __attribute__((__visibility__("hidden")))
// #define CMPTLZ_STATIC static
// #endif

#define CMPTLZ_LIKELY(expr) __builtin_expect(expr, true)
#define CMPTLZ_UNLIKELY(expr) __builtin_expect(expr, false)


#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* CMPTLZ_DEF_H */