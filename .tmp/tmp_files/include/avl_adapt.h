# 1 ".tmp/tmp_files/include/avl_adapt.h"
# 14 ".tmp/tmp_files/include/avl_adapt.h"
#ifndef AVL_ADAPT_H
#define AVL_ADAPT_H 


#ifndef AVL_VOS_NO_DEPEND
#include "vos_base.h"
#endif

#ifdef __cplusplus
extern "C" {
#endif

#define AVL_NULL_PTR 0L

#define AVL_TRUE 1
#define AVL_FALSE 0


#if defined(VOS_OS_VER) && defined(VOS_VXWORKS) && (VOS_OS_VER == VOS_VXWORKS)
#define AVL_INT32_TO_LONG 1
#endif


#ifndef VOS_PACKEND_ZERO
#define VOS_PACKEND_ZERO 1
#endif
#if defined(VOS_PACKEND_ZERO) && defined(VOS_PACK_END) && (VOS_PACKEND_ZERO == VOS_PACK_END)
#define AVL_PACKEND_ZERO 1
#endif


#ifdef __cplusplus
}
#endif

#endif
