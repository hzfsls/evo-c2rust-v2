/*
 * @file v_avlbase.h
 * Copyright (c) Huawei Technologies Co., Ltd. 2020-2020. All rights reserved.
 * Description: It is the include file of AVL Tree Algorithm
 * Author: Anonym
 * Create: 2020-03-26
 */

#ifndef V_AVLBASE_H
#define V_AVLBASE_H

#ifdef __cplusplus
extern "C" {
#endif /* --cplusplus */

// #define AVL_NULL_PTR 0L

typedef struct AVLBaseNode {
    struct AVLBaseNode *pstParent;
    struct AVLBaseNode *pstLeft;
    struct AVLBaseNode *pstRight;
    short int sLHeight;
    short int sRHeight;
} AVLBASE_NODE_S;

typedef struct AVLBaseTree {
    AVLBASE_NODE_S *pstRoot;
    AVLBASE_NODE_S *pstFirst;
    AVLBASE_NODE_S *pstLast;
} AVLBASE_TREE_S;

#define FIND_LEFTMOST_NODE(pstNode) \
    do { \
        while ((pstNode)->pstLeft != AVL_NULL_PTR) { \
            (pstNode) = (pstNode)->pstLeft; \
        } \
    } while (0)

#define FIND_RIGHTMOST_NODE(pstNode) \
    do { \
        while ((pstNode)->pstRight != AVL_NULL_PTR) { \
            (pstNode) = (pstNode)->pstRight; \
        } \
    } while (0)

static inline void VosAvlNodeRightInsert(AVLBASE_TREE_S *pstTree,
                                         AVLBASE_NODE_S *pstParentNode,
                                         AVLBASE_NODE_S *pstNode)
{
    pstNode->pstParent = pstParentNode;
    pstParentNode->pstRight = pstNode;
    pstParentNode->sRHeight = 1;
    if (pstParentNode == pstTree->pstLast) {
        /* parent was the right-most pstNode in the pstTree, */
        /* so new pstNode is now right-most               */
        pstTree->pstLast = pstNode;
    }
}

static inline void VosAvlNodeLeftInsert(AVLBASE_TREE_S *pstTree,
                                        AVLBASE_NODE_S *pstParentNode,
                                        AVLBASE_NODE_S *pstNode)
{
    pstNode->pstParent = pstParentNode;
    pstParentNode->pstLeft = pstNode;
    pstParentNode->sLHeight = 1;
    if (pstParentNode == pstTree->pstFirst) {
        /* parent was the left-most pstNode in the pstTree, */
        /* so new pstNode is now left-most               */
        pstTree->pstFirst = pstNode;
    }
}

extern void VosAvlRotateRight(AVLBASE_NODE_S **ppstSubTree);
extern void VosAvlRotateLeft(AVLBASE_NODE_S **ppstSubTree);
extern void VosAvlSwapRightMost(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstSubTree, AVLBASE_NODE_S *pstNode);
extern void VosAvlSwapLeftMost(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstSubTree, AVLBASE_NODE_S *pstNode);
extern void VosAvlRebalance(AVLBASE_NODE_S **ppstSubTree);
extern void VosAvlBalanceTree(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode);
extern AVLBASE_NODE_S* VosAvlDeleteCheck(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode);
extern void VosAvlDelete(AVLBASE_NODE_S *pstBaseNode, AVLBASE_TREE_S *pstBaseTree);

#ifdef __cplusplus
}
#endif /* --cplusplus */

#endif
