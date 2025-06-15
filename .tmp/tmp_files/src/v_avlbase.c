# 1 ".tmp/tmp_files/src/v_avlbase.c"
# 9 ".tmp/tmp_files/src/v_avlbase.c"
#include "v_avl_base.h"
#include "v_avlbase.h"


unsigned int VOS_V_AVLBaseInit(const char *pscKey)
{
    (void)pscKey;
    return 0;
}


unsigned int VOS_V_AVLBaseFini(void)
{
    return 0;
}







void VosAvlRotateRight(AVLBASE_NODE_S **ppstSubTree)
{
    AVLBASE_NODE_S *pstLeftSon = (*ppstSubTree)->pstLeft;

    (*ppstSubTree)->pstLeft = pstLeftSon->pstRight;
    if ((*ppstSubTree)->pstLeft != AVL_NULL_PTR) {
        (*ppstSubTree)->pstLeft->pstParent = (*ppstSubTree);
    }

    (*ppstSubTree)->sLHeight = pstLeftSon->sRHeight;
    pstLeftSon->pstParent = (*ppstSubTree)->pstParent;
    pstLeftSon->pstRight = *ppstSubTree;
    pstLeftSon->pstRight->pstParent = pstLeftSon;
    pstLeftSon->sRHeight = (1 + VOS_V2_AVL_MAX((*ppstSubTree)->sRHeight, (*ppstSubTree)->sLHeight));

    *ppstSubTree = pstLeftSon;

    return;
}







void VosAvlRotateLeft(AVLBASE_NODE_S **ppstSubTree)
{
    AVLBASE_NODE_S *pstRightSon = (*ppstSubTree)->pstRight;

    (*ppstSubTree)->pstRight = pstRightSon->pstLeft;
    if ((*ppstSubTree)->pstRight != AVL_NULL_PTR) {
        (*ppstSubTree)->pstRight->pstParent = (*ppstSubTree);
    }

    (*ppstSubTree)->sRHeight = pstRightSon->sLHeight;
    pstRightSon->pstParent = (*ppstSubTree)->pstParent;
    pstRightSon->pstLeft = *ppstSubTree;
    pstRightSon->pstLeft->pstParent = pstRightSon;
    pstRightSon->sLHeight = (1 + VOS_V2_AVL_MAX((*ppstSubTree)->sRHeight, (*ppstSubTree)->sLHeight));

    *ppstSubTree = pstRightSon;

    return;
}

void VosAvlUpdateSwapNode(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstSwapNode, const AVLBASE_NODE_S *pstBaseNode)
{

    pstSwapNode->pstParent = pstBaseNode->pstParent;
    pstSwapNode->pstRight = pstBaseNode->pstRight;
    pstSwapNode->pstLeft = pstBaseNode->pstLeft;
    pstSwapNode->sRHeight = pstBaseNode->sRHeight;
    pstSwapNode->sLHeight = pstBaseNode->sLHeight;
    pstSwapNode->pstRight->pstParent = pstSwapNode;
    pstSwapNode->pstLeft->pstParent = pstSwapNode;

    if (pstBaseNode->pstParent == AVL_NULL_PTR) {

        pstTree->pstRoot = pstSwapNode;
    } else if (pstBaseNode->pstParent->pstRight == pstBaseNode) {

        pstSwapNode->pstParent->pstRight = pstSwapNode;
    } else {

        pstSwapNode->pstParent->pstLeft = pstSwapNode;
    }
}
# 110 ".tmp/tmp_files/src/v_avlbase.c"
void VosAvlMoveNodeToNewPos(AVLBASE_NODE_S *pstNode, AVLBASE_NODE_S *pstNewParent, AVLBASE_NODE_S *pstNewLeftSon,
    AVLBASE_NODE_S *pstNewRightSon)
{
    pstNode->pstParent = pstNewParent;
    pstNode->pstLeft = pstNewLeftSon;
    pstNode->pstRight = pstNewRightSon;
    pstNode->sLHeight = 0;
    pstNode->sRHeight = 0;

    if (pstNewLeftSon != AVL_NULL_PTR) {
        pstNode->pstLeft->pstParent = pstNode;
        pstNode->sLHeight = 1;
    }

    if (pstNewRightSon != AVL_NULL_PTR) {
        pstNode->pstRight->pstParent = pstNode;
        pstNode->sRHeight = 1;
    }
}
# 139 ".tmp/tmp_files/src/v_avlbase.c"
void VosAvlSwapRightMost(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstSubTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstSwapNode = pstSubTree;
    AVLBASE_NODE_S *pstSwapParent;
    AVLBASE_NODE_S *pstSwapLeft;


    FIND_RIGHTMOST_NODE(pstSwapNode);

    if ((pstSwapNode->sRHeight != 0) || (pstSwapNode->sLHeight > 1)) {
        return;
    }


    pstSwapParent = pstSwapNode->pstParent;
    pstSwapLeft = pstSwapNode->pstLeft;

    VosAvlUpdateSwapNode(pstTree, pstSwapNode, pstNode);
    VosAvlMoveNodeToNewPos(pstNode, pstSwapParent, pstSwapLeft, AVL_NULL_PTR);

    pstNode->pstParent->pstRight = pstNode;

    return;
}
# 173 ".tmp/tmp_files/src/v_avlbase.c"
void VosAvlSwapLeftMost(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstSubTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstSwapNode = pstSubTree;
    AVLBASE_NODE_S *pstSwapParent;
    AVLBASE_NODE_S *pstSwapRight;


    FIND_LEFTMOST_NODE(pstSwapNode);

    if ((pstSwapNode->sLHeight != 0) || (pstSwapNode->sRHeight > 1)) {
        return;
    }


    pstSwapParent = pstSwapNode->pstParent;
    pstSwapRight = pstSwapNode->pstRight;

    VosAvlUpdateSwapNode(pstTree, pstSwapNode, pstNode);
    VosAvlMoveNodeToNewPos(pstNode, pstSwapParent, AVL_NULL_PTR, pstSwapRight);

    pstNode->pstParent->pstLeft = pstNode;

    return;
}
# 205 ".tmp/tmp_files/src/v_avlbase.c"
void VosAvlRebalance(AVLBASE_NODE_S **ppstSubTree)
{
    int iMoment;

    iMoment = (*ppstSubTree)->sRHeight - (*ppstSubTree)->sLHeight;

    if (iMoment > 1) {

        if ((*ppstSubTree)->pstRight->sLHeight > (*ppstSubTree)->pstRight->sRHeight) {



            VosAvlRotateRight(&(*ppstSubTree)->pstRight);
        }


        VosAvlRotateLeft(ppstSubTree);
    } else if (iMoment < -1) {

        if ((*ppstSubTree)->pstLeft->sRHeight > (*ppstSubTree)->pstLeft->sLHeight) {



            VosAvlRotateLeft(&(*ppstSubTree)->pstLeft);
        }


        VosAvlRotateRight(ppstSubTree);
    }

    return;
}
# 247 ".tmp/tmp_files/src/v_avlbase.c"
void VosAvlBalanceTree(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{


    AVLBASE_NODE_S *pstNodeTmp = pstNode;
    while (pstNodeTmp->pstParent != AVL_NULL_PTR) {

        if (pstNodeTmp->pstParent->pstRight == pstNodeTmp) {

            pstNodeTmp = pstNodeTmp->pstParent;
            VosAvlRebalance(&pstNodeTmp->pstRight);


            pstNodeTmp->sRHeight = (1 + VOS_V2_AVL_MAX(pstNodeTmp->pstRight->sRHeight, pstNodeTmp->pstRight->sLHeight));
        } else {

            pstNodeTmp = pstNodeTmp->pstParent;
            VosAvlRebalance(&pstNodeTmp->pstLeft);


            pstNodeTmp->sLHeight = (1 + VOS_V2_AVL_MAX(pstNodeTmp->pstLeft->sRHeight, pstNodeTmp->pstLeft->sLHeight));
        }
    }

    if (pstNodeTmp->sLHeight != pstNodeTmp->sRHeight) {

        VosAvlRebalance(&pstTree->pstRoot);
    }

    return;
}

AVLBASE_NODE_S *VosAVLSearchReplaceNodeInRTree(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstReplaceNode;

    if (pstNode->pstRight->pstLeft == AVL_NULL_PTR) {

        pstReplaceNode = pstNode->pstRight;
        pstReplaceNode->pstLeft = pstNode->pstLeft;
        pstReplaceNode->pstLeft->pstParent = pstReplaceNode;
        pstReplaceNode->sLHeight = pstNode->sLHeight;
    } else {

        VosAvlSwapLeftMost(pstTree, pstNode->pstRight, pstNode);
        pstReplaceNode = pstNode->pstRight;
    }

    return pstReplaceNode;
}

AVLBASE_NODE_S *VosAvlSearchReplaceNodeInLTree(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstReplaceNode;

    if (pstNode->pstLeft->pstRight == AVL_NULL_PTR) {

        pstReplaceNode = pstNode->pstLeft;
        pstReplaceNode->pstRight = pstNode->pstRight;
        pstReplaceNode->pstRight->pstParent = pstReplaceNode;
        pstReplaceNode->sRHeight = pstNode->sRHeight;
    } else {

        VosAvlSwapRightMost(pstTree, pstNode->pstLeft, pstNode);
        pstReplaceNode = pstNode->pstLeft;
    }

    return pstReplaceNode;
}

AVLBASE_NODE_S *VosAvlSearchReplaceNode(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstReplaceNode;

    if (pstNode->sRHeight > pstNode->sLHeight) {

        pstReplaceNode = VosAVLSearchReplaceNodeInRTree(pstTree, pstNode);
    } else {

        pstReplaceNode = VosAvlSearchReplaceNodeInLTree(pstTree, pstNode);
    }

    return pstReplaceNode;
}

AVLBASE_NODE_S *VosAvlDeleteCheck(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstReplaceNode;

    if ((pstNode->pstLeft == AVL_NULL_PTR) &&
        (pstNode->pstRight == AVL_NULL_PTR)) {

        pstReplaceNode = AVL_NULL_PTR;

        if (pstTree->pstFirst == pstNode) {

            pstTree->pstFirst = pstNode->pstParent;
        }

        if (pstTree->pstLast == pstNode) {

            pstTree->pstLast = pstNode->pstParent;
        }
    } else if (pstNode->pstLeft == AVL_NULL_PTR) {

        pstReplaceNode = pstNode->pstRight;

        if (pstTree->pstFirst == pstNode) {

            pstTree->pstFirst = pstReplaceNode;
        }
    } else if (pstNode->pstRight == AVL_NULL_PTR) {

        pstReplaceNode = pstNode->pstLeft;

        if (pstTree->pstLast == pstNode) {

            pstTree->pstLast = pstReplaceNode;
        }
    } else {

        pstReplaceNode = VosAvlSearchReplaceNode(pstTree, pstNode);
    }
    return pstReplaceNode;
}

void VosAvlDelete(AVLBASE_NODE_S *pstBaseNode, AVLBASE_TREE_S *pstBaseTree)
{
    AVLBASE_NODE_S *pstReplaceNode;
    AVLBASE_NODE_S *pstParentNode;
    short int sNewHeight = 0;

    pstReplaceNode = VosAvlDeleteCheck(pstBaseTree, pstBaseNode);

    pstParentNode = pstBaseNode->pstParent;


    pstBaseNode->pstParent = AVL_NULL_PTR;
    pstBaseNode->pstRight = AVL_NULL_PTR;
    pstBaseNode->pstLeft = AVL_NULL_PTR;
    pstBaseNode->sRHeight = -1;
    pstBaseNode->sLHeight = -1;

    if (pstReplaceNode != AVL_NULL_PTR) {


        pstReplaceNode->pstParent = pstParentNode;
        sNewHeight = (1 + VOS_V2_AVL_MAX(pstReplaceNode->sLHeight, pstReplaceNode->sRHeight));
    }

    if (pstParentNode != AVL_NULL_PTR) {

        if (pstParentNode->pstRight == pstBaseNode) {

            pstParentNode->pstRight = pstReplaceNode;
            pstParentNode->sRHeight = sNewHeight;
        } else {

            pstParentNode->pstLeft = pstReplaceNode;
            pstParentNode->sLHeight = sNewHeight;
        }


        VosAvlBalanceTree(pstBaseTree, pstParentNode);
    } else {

        pstBaseTree->pstRoot = pstReplaceNode;
    }

    return;
}
