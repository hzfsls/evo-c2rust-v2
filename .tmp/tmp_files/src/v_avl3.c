# 1 ".tmp/tmp_files/src/v_avl3.c"
# 9 ".tmp/tmp_files/src/v_avl3.c"
#include "v_avlbase.h"
#include "v_avl3_inner.h"
#include "v_avl3.h"


unsigned int VOS_V_AVL3Init(const char *pscKey)
{
    (void)pscKey;
    return 0;
}


unsigned int VOS_V_AVL3Fini(void)
{
    return 0;
}

void *AVL3_Find_Or_Find_Next(AVL3_TREE *pstTree, const void *pKey,
                             unsigned int bFlag, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNode;
    void *pFoundNode = AVL_NULL_PTR;
    int iResult;
    int iKeyOffset;

    if (TREE_OR_TREEINFO_IS_NULL(pstTree, pstTreeInfo)) {
        return AVL_NULL_PTR;
    }
    pstNode = pstTree->pstRoot;
    if (pstNode == AVL_NULL_PTR) {
        return AVL_NULL_PTR;
    }

    iKeyOffset = GET_KEYOFFSET(pstTreeInfo);

    for (;;) {

        iResult = pstTreeInfo->pfCompare(pKey,
            (void *)((unsigned char *)pstNode + iKeyOffset));
        if (iResult > 0) {


            if (pstNode->pstRight == AVL_NULL_PTR) {


                pFoundNode = VOS_AVL3_Next(pstNode, pstTreeInfo);
                break;
            }

            pstNode = pstNode->pstRight;
        } else if (iResult < 0) {


            if (pstNode->pstLeft == AVL_NULL_PTR) {

                pFoundNode = (void *)((unsigned char *)pstNode - pstTreeInfo->usNodeOffset);
                break;
            }

            pstNode = pstNode->pstLeft;
        } else {

            if (bFlag != 0) {

                pFoundNode = VOS_AVL3_Next(pstNode, pstTreeInfo);
            } else {
                pFoundNode = (void *)((unsigned char *)pstNode - pstTreeInfo->usNodeOffset);
            }
            break;
        }
    }

    return pFoundNode;
}
# 102 ".tmp/tmp_files/src/v_avl3.c"
void *VOS_AVL3_Insert_Or_Find(AVL3_TREE *pstTree, AVL3_NODE *pstNode, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstParentNode;
    int iResult;
    int iKeyOffset;

    if (TREE_OR_TREEINFO_IS_NULL(pstTree, pstTreeInfo) || (pstNode == AVL_NULL_PTR)) {
        return AVL_NULL_PTR;
    }

    pstNode->sRHeight = 0;
    pstNode->sLHeight = 0;

    if (pstTree->pstRoot == AVL_NULL_PTR) {

        pstTree->pstRoot = pstNode;
        pstTree->pstFirst = pstNode;
        pstTree->pstLast = pstNode;
        return AVL_NULL_PTR;
    }


    pstParentNode = pstTree->pstRoot;

    iKeyOffset = GET_KEYOFFSET(pstTreeInfo);
    while (pstParentNode != AVL_NULL_PTR) {

        iResult = pstTreeInfo->pfCompare(
            (void *)((unsigned char *)pstNode + iKeyOffset),
            (void *)((unsigned char *)pstParentNode + iKeyOffset));
        if (iResult > 0) {


            if (pstParentNode->pstRight != AVL_NULL_PTR) {

                pstParentNode = pstParentNode->pstRight;
                continue;
            }


            VosAvlNodeRightInsert((AVLBASE_TREE_S *)pstTree, (AVLBASE_NODE_S *)pstParentNode,
                (AVLBASE_NODE_S *)pstNode);
        } else if (iResult < 0) {


            if (pstParentNode->pstLeft != AVL_NULL_PTR) {

                pstParentNode = pstParentNode->pstLeft;
                continue;
            }


            VosAvlNodeLeftInsert((AVLBASE_TREE_S *)pstTree, (AVLBASE_NODE_S *)pstParentNode, (AVLBASE_NODE_S *)pstNode);
        } else {

            pstNode->sRHeight = -1;
            pstNode->sLHeight = -1;
            return (void *)((unsigned char *)pstParentNode - pstTreeInfo->usNodeOffset);
        }

        break;
    }


    VosAvlBalanceTree((AVLBASE_TREE_S *)pstTree, (AVLBASE_NODE_S *)pstParentNode);

    return AVL_NULL_PTR;
}

void VOS_AVL3_Delete(AVL3_TREE *pstTree, AVL3_NODE *pstNode)
{

    AVLBASE_NODE_S *pstBaseNode;
    AVLBASE_TREE_S *pstBaseTree;
    if ((pstTree == AVL_NULL_PTR) || (pstNode == AVL_NULL_PTR)) {
        return;
    }

    pstBaseNode = (AVLBASE_NODE_S *)pstNode;
    pstBaseTree = (AVLBASE_TREE_S *)pstTree;
    VosAvlDelete(pstBaseNode, pstBaseTree);
}
# 199 ".tmp/tmp_files/src/v_avl3.c"
void *VOS_AVL3_Find(AVL3_TREE *pstTree, const void *pstKey, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNode;
    int iResult;
    int iKeyOffset;

    if (TREE_OR_TREEINFO_IS_NULL(pstTree, pstTreeInfo)) {
        return AVL_NULL_PTR;
    }

    pstNode = pstTree->pstRoot;
    iKeyOffset = GET_KEYOFFSET(pstTreeInfo);

    while (pstNode != AVL_NULL_PTR) {
        iResult = pstTreeInfo->pfCompare(pstKey,
            (void *)((unsigned char *)pstNode + iKeyOffset));
        if (iResult > 0) {


            pstNode = pstNode->pstRight;
        } else if (iResult < 0) {


            pstNode = pstNode->pstLeft;
        } else {

            break;
        }
    }

    return GET_NODE_START_ADDRESS(pstNode, pstTreeInfo->usNodeOffset);
}

void *VOS_AVL3_First(AVL3_TREE *pstTree, AVL3_TREE_INFO *pstTreeInfo)
{

    AVL3_NODE *pstNode;

    if (TREE_OR_TREEINFO_IS_NULL(pstTree, pstTreeInfo)) {
        return AVL_NULL_PTR;
    }

    pstNode = pstTree->pstFirst;

    return GET_NODE_START_ADDRESS(pstNode, pstTreeInfo->usNodeOffset);
}

void *VOS_AVL3_Last(AVL3_TREE *pstTree, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNode;

    if (TREE_OR_TREEINFO_IS_NULL(pstTree, pstTreeInfo)) {
        return AVL_NULL_PTR;
    }

    pstNode = pstTree->pstLast;

    return GET_NODE_START_ADDRESS(pstNode, pstTreeInfo->usNodeOffset);
}
# 269 ".tmp/tmp_files/src/v_avl3.c"
void *VOS_AVL3_Next(AVL3_NODE *pstNode, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNodeTmp = pstNode;
    if ((pstNodeTmp == AVL_NULL_PTR) || (pstTreeInfo == AVL_NULL_PTR)) {
        return AVL_NULL_PTR;
    }

    if (pstNodeTmp->pstRight != AVL_NULL_PTR) {

        pstNodeTmp = pstNodeTmp->pstRight;
        FIND_LEFTMOST_NODE(pstNodeTmp);
    } else {

        while (pstNodeTmp != AVL_NULL_PTR) {
            if ((pstNodeTmp->pstParent == AVL_NULL_PTR) || (pstNodeTmp->pstParent->pstLeft == pstNodeTmp)) {
                pstNodeTmp = pstNodeTmp->pstParent;
                break;
            }

            pstNodeTmp = pstNodeTmp->pstParent;
        }
    }

    return GET_NODE_START_ADDRESS(pstNodeTmp, pstTreeInfo->usNodeOffset);
}
# 305 ".tmp/tmp_files/src/v_avl3.c"
void *VOS_AVL3_Prev(AVL3_NODE *pstNode, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNodeTmp = pstNode;
    if ((pstNodeTmp == AVL_NULL_PTR) || (pstTreeInfo == AVL_NULL_PTR)) {
        return AVL_NULL_PTR;
    }

    if (pstNodeTmp->pstLeft != AVL_NULL_PTR) {

        pstNodeTmp = pstNodeTmp->pstLeft;
        FIND_RIGHTMOST_NODE(pstNodeTmp);
    } else {

        while (pstNodeTmp != AVL_NULL_PTR) {
            if ((pstNodeTmp->pstParent == AVL_NULL_PTR) || (pstNodeTmp->pstParent->pstRight == pstNodeTmp)) {

                pstNodeTmp = pstNodeTmp->pstParent;
                break;
            }

            pstNodeTmp = pstNodeTmp->pstParent;
        }
    }

    return GET_NODE_START_ADDRESS(pstNodeTmp, pstTreeInfo->usNodeOffset);
}
