# 1 ".tmp/tmp_files/src/v_avlpub.c"
# 9 ".tmp/tmp_files/src/v_avlpub.c"
#include "v_avlbase.h"
#include "v_avl_base.h"
# 28 ".tmp/tmp_files/src/v_avlpub.c"
void *VOS_AVL_Insert_Or_Find(AVL_TREE *pstTree, AVL_NODE *pstNode)
{
    AVL_NODE *pstParentNode;
    int iResult;

    if ((pstTree == AVL_NULL_PTR) || (pstNode == AVL_NULL_PTR) || (VOS_AVL_IN_TREE(*pstNode))) {
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


    for (pstParentNode = pstTree->pstRoot; pstParentNode != AVL_NULL_PTR;) {

        iResult = pstTree->pfnCompare(pstNode->pKey, pstParentNode->pKey);
        if (iResult > 0) {


            if (pstParentNode->pstRight != AVL_NULL_PTR) {

                pstParentNode = pstParentNode->pstRight;
                continue;
            }

            VosAvlNodeRightInsert((AVLBASE_TREE_S *)(void *)(&(pstTree->pstRoot)), (AVLBASE_NODE_S *)pstParentNode,
                (AVLBASE_NODE_S *)pstNode);

            break;
        } else if (iResult < 0) {


            if (pstParentNode->pstLeft != AVL_NULL_PTR) {

                pstParentNode = pstParentNode->pstLeft;
                continue;
            }

            VosAvlNodeLeftInsert((AVLBASE_TREE_S *)(void *)(&(pstTree->pstRoot)), (AVLBASE_NODE_S *)pstParentNode,
                (AVLBASE_NODE_S *)pstNode);

            break;
        }

        pstNode->sRHeight = -1;
        pstNode->sLHeight = -1;
        return pstParentNode->pSelf;
    }


    if (pstParentNode != AVL_NULL_PTR) {
        VosAvlBalanceTree((AVLBASE_TREE_S *)(void *)(&(pstTree->pstRoot)), (AVLBASE_NODE_S *)pstParentNode);
    }

    return AVL_NULL_PTR;
}


void VOS_AVL_Delete(AVL_TREE *pstTree, AVL_NODE *pstNode)
{
    AVLBASE_NODE_S *pstBaseNode;
    AVLBASE_TREE_S *pstBaseTree;

    if ((pstTree == AVL_NULL_PTR) || (pstNode == AVL_NULL_PTR) || (!VOS_AVL_IN_TREE(*pstNode))) {
        return;
    }

    pstBaseNode = (AVLBASE_NODE_S *)pstNode;
    pstBaseTree = (AVLBASE_TREE_S *)(void *)(&(pstTree->pstRoot));
    VosAvlDelete(pstBaseNode, pstBaseTree);
    return;
}
# 121 ".tmp/tmp_files/src/v_avlpub.c"
void *VOS_AVL_Find(AVL_TREE *pstTree, const void *pKey)
{

    AVL_NODE *pstNode;
    int iResult;

    if (pstTree == AVL_NULL_PTR) {
        return AVL_NULL_PTR;
    }
    pstNode = pstTree->pstRoot;

    while (pstNode != AVL_NULL_PTR) {

        iResult = pstTree->pfnCompare(pKey, pstNode->pKey);
        if (iResult > 0) {


            pstNode = pstNode->pstRight;
        } else if (iResult < 0) {


            pstNode = pstNode->pstLeft;
        } else {

            break;
        }
    }

    return ((pstNode != AVL_NULL_PTR) ? pstNode->pSelf : AVL_NULL_PTR);
}
# 161 ".tmp/tmp_files/src/v_avlpub.c"
void *VOS_AVL_Next(AVL_NODE *pstNode)
{
    AVL_NODE *pstNodeTmp = pstNode;
    if ((pstNodeTmp == AVL_NULL_PTR) || (!VOS_AVL_IN_TREE(*pstNodeTmp))) {
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

    return ((pstNodeTmp != AVL_NULL_PTR) ? pstNodeTmp->pSelf : AVL_NULL_PTR);
}
# 196 ".tmp/tmp_files/src/v_avlpub.c"
void *VOS_AVL_Prev(AVL_NODE *pstNode)
{
    AVL_NODE *pstNodeTmp = pstNode;
    if ((pstNodeTmp == AVL_NULL_PTR) || (!VOS_AVL_IN_TREE(*pstNodeTmp))) {
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

    return ((pstNodeTmp != AVL_NULL_PTR) ? pstNodeTmp->pSelf : AVL_NULL_PTR);
}
# 233 ".tmp/tmp_files/src/v_avlpub.c"
void *VOS_AVL_Find_Or_Find_Next(AVL_TREE *pstTree, const void *pKey, unsigned int bValue)
{
    AVL_NODE *pstNode;
    void *pFoundNode = AVL_NULL_PTR;
    int iResult;

    if (pstTree == AVL_NULL_PTR) {
        return AVL_NULL_PTR;
    }
    pstNode = pstTree->pstRoot;

    if (pstNode == AVL_NULL_PTR) {
        return (pFoundNode);
    }


    for (;;) {

        iResult = pstTree->pfnCompare(pKey, pstNode->pKey);
        if (iResult > 0) {


            if (pstNode->pstRight == AVL_NULL_PTR) {


                pFoundNode = VOS_AVL_Next(pstNode);
                break;
            }

            pstNode = pstNode->pstRight;
        } else if (iResult < 0) {


            if (pstNode->pstLeft == AVL_NULL_PTR) {

                pFoundNode = pstNode->pSelf;
                break;
            }

            pstNode = pstNode->pstLeft;
        } else {

            if (bValue != 0) {

                pFoundNode = VOS_AVL_Next(pstNode);
            } else {
                pFoundNode = pstNode->pSelf;
            }
            break;
        }
    }

    return (pFoundNode);
}
