# 1 ".tmp/tmp_files/src/v_avl3_inner.h"







#ifndef V_AVL3_INNER_H
#define V_AVL3_INNER_H 


#ifdef __cplusplus
extern "C" {
#endif

#define TREE_OR_TREEINFO_IS_NULL(pstTree,pstTreeInfo) (((pstTree) == AVL_NULL_PTR) || ((pstTreeInfo) == AVL_NULL_PTR))

#define GET_NODE_START_ADDRESS(pstNode,usOffset) (((pstNode) != AVL_NULL_PTR) ? (void *)((unsigned char *)(pstNode) - (usOffset)) : AVL_NULL_PTR)

#define GET_KEYOFFSET(pstTreeInfo) ((int)((pstTreeInfo)->usKeyOffset - (pstTreeInfo)->usNodeOffset))

#ifdef __cplusplus
}
#endif

#endif
