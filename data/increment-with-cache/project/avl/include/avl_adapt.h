/*
 * @file avl_adapt.h
 * Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * Description: It is the header file of the AVL adapt.
 * Author: Anonym
 * Create: 2024-04-08
 * Notes: none
 */

/**
 * @defgroup v_avl AVLBASE
 * @ingroup util
 */
#ifndef AVL_ADAPT_H
#define AVL_ADAPT_H

/* 兼容SSP基础数据类型依赖。DOPRA SSP使用，不定义宏，依赖VOS头文件；AVL库单独使用需要定义该宏，不依赖VOS头文件 */
#ifndef AVL_VOS_NO_DEPEND
#include "vos_base.h"
#endif

#ifdef __cplusplus
extern "C" {
#endif /* --cplusplus */

#define AVL_NULL_PTR 0L

#define AVL_TRUE 1
#define AVL_FALSE 0

/* 兼容SSP VXWORKS */
#if defined(VOS_OS_VER) && defined(VOS_VXWORKS) && (VOS_OS_VER == VOS_VXWORKS)
#define AVL_INT32_TO_LONG 1
#endif

/* 兼容SSP VXWORKS */
#ifndef VOS_PACKEND_ZERO
#define VOS_PACKEND_ZERO 1
#endif
#if defined(VOS_PACKEND_ZERO) && defined(VOS_PACK_END) && (VOS_PACKEND_ZERO == VOS_PACK_END)
#define AVL_PACKEND_ZERO 1
#endif


#ifdef __cplusplus
}
#endif /* --cplusplus */

#endif
