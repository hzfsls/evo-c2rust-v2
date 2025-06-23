/*
 * @file v_avl3.inc
 * Copyright (c) Huawei Technologies Co., Ltd. 2020-2020. All rights reserved.
 * Description: It contains the macro & data which will be used for file
 * Author: Anonym
 * Create: 2020-11-03
 */
#ifndef V_AVL3_INNER_H
#define V_AVL3_INNER_H


#ifdef __cplusplus
extern "C" {
#endif /* --cplusplus */

#define TREE_OR_TREEINFO_IS_NULL(pstTree, pstTreeInfo) (((pstTree) == AVL_NULL_PTR) || ((pstTreeInfo) == AVL_NULL_PTR))

#define GET_NODE_START_ADDRESS(pstNode, usOffset) \
        (((pstNode) != AVL_NULL_PTR) ?  (void *)((unsigned char *)(pstNode) - (usOffset)) : AVL_NULL_PTR)

#define GET_KEYOFFSET(pstTreeInfo) ((int)((pstTreeInfo)->usKeyOffset - (pstTreeInfo)->usNodeOffset))

#ifdef __cplusplus
}
#endif /* --cplusplus */

#endif /* V_AVL3_INNER_H */
