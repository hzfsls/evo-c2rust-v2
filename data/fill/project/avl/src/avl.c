typedef int (*AVL_V2_COMPARE_FUNC)(const void *, const void *);

typedef struct avl_node
{
    struct avl_node *pstParent;
    struct avl_node *pstLeft;
    struct avl_node *pstRight;
    short int sLHeight;
    short int sRHeight;
    void *pSelf;
    void *pKey;
} AVL_NODE;

typedef struct avl_tree
{
    AVL_V2_COMPARE_FUNC pfnCompare;
    AVL_NODE *pstRoot;
    AVL_NODE *pstFirst;
    AVL_NODE *pstLast;
} AVL_TREE;

typedef int (*AVL3_COMPARE)(const void *, const void *);

typedef struct avl3_tree_info
{
    AVL3_COMPARE pfCompare;
    unsigned short int usKeyOffset;
    unsigned short int usNodeOffset;
} AVL3_TREE_INFO;

typedef struct avl3_tree
{
    AVL3_NODE *pstRoot;
    AVL3_NODE *pstFirst;
    AVL3_NODE *pstLast;
} AVL3_TREE;

typedef struct avl3_node
{
    struct avl3_node *pstParent;
    struct avl3_node *pstLeft;
    struct avl3_node *pstRight;
    short int sLHeight;
    short int sRHeight;
} AVL3_NODE;

typedef struct AVLBaseNode
{
    struct AVLBaseNode *pstParent;
    struct AVLBaseNode *pstLeft;
    struct AVLBaseNode *pstRight;
    short int sLHeight;
    short int sRHeight;
} AVLBASE_NODE_S;

typedef struct AVLBaseTree
{
    AVLBASE_NODE_S *pstRoot;
    AVLBASE_NODE_S *pstFirst;
    AVLBASE_NODE_S *pstLast;
} AVLBASE_TREE_S;

#define AVL_NULL_PTR 0L

#define AVL_TRUE 1

#define AVL_FALSE 0

#define VOS_AVL_INIT_TREE(TREE, COMPARE)                                                                               \
    do                                                                                                                 \
    {                                                                                                                  \
        (TREE).pfnCompare = (COMPARE);                                                                                 \
        (TREE).pstFirst = (AVL_NODE *)AVL_NULL_PTR;                                                                    \
        (TREE).pstLast = (AVL_NODE *)AVL_NULL_PTR;                                                                     \
        (TREE).pstRoot = (AVL_NODE *)AVL_NULL_PTR;                                                                     \
    } while (0)

#define VOS_AVL_INIT_NODE(NODE, SELF, KEY)                                                                             \
    do                                                                                                                 \
    {                                                                                                                  \
        (NODE).pstParent = (AVL_NODE *)AVL_NULL_PTR;                                                                   \
        (NODE).pstLeft = (AVL_NODE *)AVL_NULL_PTR;                                                                     \
        (NODE).pstRight = (AVL_NODE *)AVL_NULL_PTR;                                                                    \
        (NODE).pSelf = (SELF);                                                                                         \
        (NODE).pKey = (KEY);                                                                                           \
        (NODE).sLHeight = -1;                                                                                          \
        (NODE).sRHeight = -1;                                                                                          \
    } while (0)

#define VOS_AVL_INSERT(TREE, NODE) (VOS_AVL_Insert_Or_Find(&(TREE), &(NODE)) == AVL_NULL_PTR)


#define VOS_AVL_DELETE(TREE, NODE) VOS_AVL_Delete(&(TREE), &(NODE))

#define VOS_AVL_FIND(TREE, KEY) VOS_AVL_Find(&(TREE), (KEY))

#define VOS_AVL_IN_TREE(NODE) (((NODE).sLHeight != -1) && ((NODE).sRHeight != -1))


#define VOS_AVL_FIND_OR_FIND_NEXT(TREE, KEY) VOS_AVL_Find_Or_Find_Next(&(TREE), (KEY), AVL_FALSE)

#define VOS_V2_AVL_MAX(X, Y) (((X) > (Y)) ? (X) : (Y))

#define VOS_AVL3_INIT_TREE(TREE, TREE_INFO)                                                                            \
    do                                                                                                                 \
    {                                                                                                                  \
        (TREE).pstFirst = (AVL3_NODE *)AVL_NULL_PTR;                                                                   \
        (TREE).pstLast = (AVL3_NODE *)AVL_NULL_PTR;                                                                    \
        (TREE).pstRoot = (AVL3_NODE *)AVL_NULL_PTR;                                                                    \
    } while (0)

#define VOS_AVL3_INIT_NODE(NODE)                                                                                       \
    do                                                                                                                 \
    {                                                                                                                  \
        (NODE).pstParent = (AVL3_NODE *)AVL_NULL_PTR;                                                                  \
        (NODE).pstLeft = (AVL3_NODE *)AVL_NULL_PTR;                                                                    \
        (NODE).pstRight = (AVL3_NODE *)AVL_NULL_PTR;                                                                   \
        (NODE).sLHeight = -1;                                                                                          \
        (NODE).sRHeight = -1;                                                                                          \
    } while (0)

#define VOS_AVL3_INSERT(TREE, NODE, TREE_INFO) (AVL_NULL_PTR == VOS_AVL3_Insert_Or_Find(&(TREE), &(NODE), &(TREE_INFO)))


#define VOS_AVL3_INSERT_OR_FIND(TREE, NODE, TREE_INFO) VOS_AVL3_Insert_Or_Find(&(TREE), &(NODE), &(TREE_INFO))


#define VOS_AVL3_DELETE(TREE, NODE) VOS_AVL3_Delete(&(TREE), &(NODE))


#define VOS_AVL3_FIND(TREE, KEY, TREE_INFO) VOS_AVL3_Find(&(TREE), (KEY), &(TREE_INFO))


#define VOS_AVL3_NEXT(NODE, TREE_INFO) VOS_AVL3_Next(&(NODE), &(TREE_INFO))


#define VOS_AVL3_PREV(NODE, TREE_INFO) VOS_AVL3_Prev(&(NODE), &(TREE_INFO))


#define VOS_AVL3_FIRST(TREE, TREE_INFO) VOS_AVL3_First(&(TREE), &(TREE_INFO))


#define VOS_AVL3_LAST(TREE, TREE_INFO) VOS_AVL3_Last(&(TREE), &(TREE_INFO))


#define VOS_AVL3_IN_TREE(NODE) (((NODE).sLHeight != -1) && ((NODE).sRHeight != -1))


#define VOS_AVL3_FIND_NEXT(TREE, KEY, TREE_INFO) AVL3_Find_Or_Find_Next(&(TREE), (KEY), AVL_TRUE, &(TREE_INFO))


#define VOS_AVL3_FIND_OR_FIND_NEXT(TREE, KEY, TREE_INFO) AVL3_Find_Or_Find_Next(&(TREE), (KEY), AVL_FALSE, &(TREE_INFO))


#define TREE_OR_TREEINFO_IS_NULL(pstTree, pstTreeInfo) (((pstTree) == AVL_NULL_PTR) || ((pstTreeInfo) == AVL_NULL_PTR))


#define GET_NODE_START_ADDRESS(pstNode, usOffset)                                                                      \
    (((pstNode) != AVL_NULL_PTR) ? (void *)((unsigned char *)(pstNode) - (usOffset)) : AVL_NULL_PTR)


#define GET_KEYOFFSET(pstTreeInfo) ((int)((pstTreeInfo)->usKeyOffset - (pstTreeInfo)->usNodeOffset))


#define FIND_LEFTMOST_NODE(pstNode)                                                                                    \
    do                                                                                                                 \
    {                                                                                                                  \
        while ((pstNode)->pstLeft != AVL_NULL_PTR)                                                                     \
        {                                                                                                              \
            (pstNode) = (pstNode)->pstLeft;                                                                            \
        }                                                                                                              \
    } while (0)

#define FIND_RIGHTMOST_NODE(pstNode)                                                                                   \
    do                                                                                                                 \
    {                                                                                                                  \
        while ((pstNode)->pstRight != AVL_NULL_PTR)                                                                    \
        {                                                                                                              \
            (pstNode) = (pstNode)->pstRight;                                                                           \
        }                                                                                                              \
    } while (0)

void *AVL3_Find_Or_Find_Next(AVL3_TREE *pstTree, const void *pKey, unsigned int bFlag, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNode;
    void *pFoundNode = AVL_NULL_PTR;
    int iResult;
    int iKeyOffset;
    if (TREE_OR_TREEINFO_IS_NULL(pstTree, pstTreeInfo))
    {
        return AVL_NULL_PTR;
    }
    pstNode = pstTree->pstRoot;
    if (pstNode == AVL_NULL_PTR)
    {
        return AVL_NULL_PTR;
    }
    iKeyOffset = GET_KEYOFFSET(pstTreeInfo);
    for (;;)
    {
        iResult = pstTreeInfo->pfCompare(pKey, (void *)((unsigned char *)pstNode + iKeyOffset));
        if (iResult > 0)
        {
            if (pstNode->pstRight == AVL_NULL_PTR)
            {
                pFoundNode = VOS_AVL3_Next(pstNode, pstTreeInfo);
                break;
            }
            pstNode = pstNode->pstRight;
        }
        else if (iResult < 0)
        {
            if (pstNode->pstLeft == AVL_NULL_PTR)
            {
                pFoundNode = (void *)((unsigned char *)pstNode - pstTreeInfo->usNodeOffset);
                break;
            }
            pstNode = pstNode->pstLeft;
        }
        else
        {
            if (bFlag != 0)
            {
                pFoundNode = VOS_AVL3_Next(pstNode, pstTreeInfo);
            }
            else
            {
                pFoundNode = (void *)((unsigned char *)pstNode - pstTreeInfo->usNodeOffset);
            }
            break;
        }
    }
    return pFoundNode;
}

void *VOS_AVL3_Insert_Or_Find(AVL3_TREE *pstTree, AVL3_NODE *pstNode, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstParentNode;
    int iResult;
    int iKeyOffset;
    if (TREE_OR_TREEINFO_IS_NULL(pstTree, pstTreeInfo) || (pstNode == AVL_NULL_PTR))
    {
        return AVL_NULL_PTR;
    }
    pstNode->sRHeight = 0;
    pstNode->sLHeight = 0;
    if (pstTree->pstRoot == AVL_NULL_PTR)
    {
        pstTree->pstRoot = pstNode;
        pstTree->pstFirst = pstNode;
        pstTree->pstLast = pstNode;
        return AVL_NULL_PTR;
    }
    pstParentNode = pstTree->pstRoot;
    iKeyOffset = GET_KEYOFFSET(pstTreeInfo);
    while (pstParentNode != AVL_NULL_PTR)
    {
        iResult = pstTreeInfo->pfCompare((void *)((unsigned char *)pstNode + iKeyOffset),
                                         (void *)((unsigned char *)pstParentNode + iKeyOffset));
        if (iResult > 0)
        {
            if (pstParentNode->pstRight != AVL_NULL_PTR)
            {
                pstParentNode = pstParentNode->pstRight;
                continue;
            }
            VosAvlNodeRightInsert((AVLBASE_TREE_S *)pstTree, (AVLBASE_NODE_S *)pstParentNode,
                                  (AVLBASE_NODE_S *)pstNode);
        }
        else if (iResult < 0)
        {
            if (pstParentNode->pstLeft != AVL_NULL_PTR)
            {
                pstParentNode = pstParentNode->pstLeft;
                continue;
            }
            VosAvlNodeLeftInsert((AVLBASE_TREE_S *)pstTree, (AVLBASE_NODE_S *)pstParentNode, (AVLBASE_NODE_S *)pstNode);
        }
        else
        {
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
    if ((pstTree == AVL_NULL_PTR) || (pstNode == AVL_NULL_PTR))
    {
        return;
    }
    pstBaseNode = (AVLBASE_NODE_S *)pstNode;
    pstBaseTree = (AVLBASE_TREE_S *)pstTree;
    VosAvlDelete(pstBaseNode, pstBaseTree);
}

void *VOS_AVL3_Find(AVL3_TREE *pstTree, const void *pstKey, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNode;
    int iResult;
    int iKeyOffset;
    if (TREE_OR_TREEINFO_IS_NULL(pstTree, pstTreeInfo))
    {
        return AVL_NULL_PTR;
    }
    pstNode = pstTree->pstRoot;
    iKeyOffset = GET_KEYOFFSET(pstTreeInfo);
    while (pstNode != AVL_NULL_PTR)
    {
        iResult = pstTreeInfo->pfCompare(pstKey, (void *)((unsigned char *)pstNode + iKeyOffset));
        if (iResult > 0)
        {
            pstNode = pstNode->pstRight;
        }
        else if (iResult < 0)
        {
            pstNode = pstNode->pstLeft;
        }
        else
        {
            break;
        }
    }
    return GET_NODE_START_ADDRESS(pstNode, pstTreeInfo->usNodeOffset);
}

void *VOS_AVL3_First(AVL3_TREE *pstTree, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNode;
    if (TREE_OR_TREEINFO_IS_NULL(pstTree, pstTreeInfo))
    {
        return AVL_NULL_PTR;
    }
    pstNode = pstTree->pstFirst;
    return GET_NODE_START_ADDRESS(pstNode, pstTreeInfo->usNodeOffset);
}

void *VOS_AVL3_Last(AVL3_TREE *pstTree, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNode;
    if (TREE_OR_TREEINFO_IS_NULL(pstTree, pstTreeInfo))
    {
        return AVL_NULL_PTR;
    }
    pstNode = pstTree->pstLast;
    return GET_NODE_START_ADDRESS(pstNode, pstTreeInfo->usNodeOffset);
}

void *VOS_AVL3_Next(AVL3_NODE *pstNode, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNodeTmp = pstNode;
    if ((pstNodeTmp == AVL_NULL_PTR) || (pstTreeInfo == AVL_NULL_PTR))
    {
        return AVL_NULL_PTR;
    }
    if (pstNodeTmp->pstRight != AVL_NULL_PTR)
    {
        pstNodeTmp = pstNodeTmp->pstRight;
        FIND_LEFTMOST_NODE(pstNodeTmp);
    }
    else
    {
        while (pstNodeTmp != AVL_NULL_PTR)
        {
            if ((pstNodeTmp->pstParent == AVL_NULL_PTR) || (pstNodeTmp->pstParent->pstLeft == pstNodeTmp))
            {
                pstNodeTmp = pstNodeTmp->pstParent;
                break;
            }
            pstNodeTmp = pstNodeTmp->pstParent;
        }
    }
    return GET_NODE_START_ADDRESS(pstNodeTmp, pstTreeInfo->usNodeOffset);
}

void *VOS_AVL3_Prev(AVL3_NODE *pstNode, AVL3_TREE_INFO *pstTreeInfo)
{
    AVL3_NODE *pstNodeTmp = pstNode;
    if ((pstNodeTmp == AVL_NULL_PTR) || (pstTreeInfo == AVL_NULL_PTR))
    {
        return AVL_NULL_PTR;
    }
    if (pstNodeTmp->pstLeft != AVL_NULL_PTR)
    {
        pstNodeTmp = pstNodeTmp->pstLeft;
        FIND_RIGHTMOST_NODE(pstNodeTmp);
    }
    else
    {
        while (pstNodeTmp != AVL_NULL_PTR)
        {
            if ((pstNodeTmp->pstParent == AVL_NULL_PTR) || (pstNodeTmp->pstParent->pstRight == pstNodeTmp))
            {
                pstNodeTmp = pstNodeTmp->pstParent;
                break;
            }
            pstNodeTmp = pstNodeTmp->pstParent;
        }
    }
    return GET_NODE_START_ADDRESS(pstNodeTmp, pstTreeInfo->usNodeOffset);
}

static inline void VosAvlNodeRightInsert(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstParentNode,
                                         AVLBASE_NODE_S *pstNode)
{
    pstNode->pstParent = pstParentNode;
    pstParentNode->pstRight = pstNode;
    pstParentNode->sRHeight = 1;
    if (pstParentNode == pstTree->pstLast)
    {
        pstTree->pstLast = pstNode;
    }
}

static inline void VosAvlNodeLeftInsert(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstParentNode, AVLBASE_NODE_S *pstNode)
{
    pstNode->pstParent = pstParentNode;
    pstParentNode->pstLeft = pstNode;
    pstParentNode->sLHeight = 1;
    if (pstParentNode == pstTree->pstFirst)
    {
        pstTree->pstFirst = pstNode;
    }
}

void VosAvlRotateRight(AVLBASE_NODE_S **ppstSubTree)
{
    AVLBASE_NODE_S *pstLeftSon = (*ppstSubTree)->pstLeft;
    (*ppstSubTree)->pstLeft = pstLeftSon->pstRight;
    if ((*ppstSubTree)->pstLeft != AVL_NULL_PTR)
    {
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
    if ((*ppstSubTree)->pstRight != AVL_NULL_PTR)
    {
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
    if (pstBaseNode->pstParent == AVL_NULL_PTR)
    {
        pstTree->pstRoot = pstSwapNode;
    }
    else if (pstBaseNode->pstParent->pstRight == pstBaseNode)
    {
        pstSwapNode->pstParent->pstRight = pstSwapNode;
    }
    else
    {
        pstSwapNode->pstParent->pstLeft = pstSwapNode;
    }
}

void VosAvlMoveNodeToNewPos(AVLBASE_NODE_S *pstNode, AVLBASE_NODE_S *pstNewParent, AVLBASE_NODE_S *pstNewLeftSon,
                            AVLBASE_NODE_S *pstNewRightSon)
{
    pstNode->pstParent = pstNewParent;
    pstNode->pstLeft = pstNewLeftSon;
    pstNode->pstRight = pstNewRightSon;
    pstNode->sLHeight = 0;
    pstNode->sRHeight = 0;
    if (pstNewLeftSon != AVL_NULL_PTR)
    {
        pstNode->pstLeft->pstParent = pstNode;
        pstNode->sLHeight = 1;
    }
    if (pstNewRightSon != AVL_NULL_PTR)
    {
        pstNode->pstRight->pstParent = pstNode;
        pstNode->sRHeight = 1;
    }
}

void VosAvlSwapRightMost(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstSubTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstSwapNode = pstSubTree;
    AVLBASE_NODE_S *pstSwapParent;
    AVLBASE_NODE_S *pstSwapLeft;
    FIND_RIGHTMOST_NODE(pstSwapNode);
    if ((pstSwapNode->sRHeight != 0) || (pstSwapNode->sLHeight > 1))
    {
        return;
    }
    pstSwapParent = pstSwapNode->pstParent;
    pstSwapLeft = pstSwapNode->pstLeft;
    VosAvlUpdateSwapNode(pstTree, pstSwapNode, pstNode);
    VosAvlMoveNodeToNewPos(pstNode, pstSwapParent, pstSwapLeft, AVL_NULL_PTR);
    pstNode->pstParent->pstRight = pstNode;
    return;
}

void VosAvlSwapLeftMost(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstSubTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstSwapNode = pstSubTree;
    AVLBASE_NODE_S *pstSwapParent;
    AVLBASE_NODE_S *pstSwapRight;
    FIND_LEFTMOST_NODE(pstSwapNode);
    if ((pstSwapNode->sLHeight != 0) || (pstSwapNode->sRHeight > 1))
    {
        return;
    }
    pstSwapParent = pstSwapNode->pstParent;
    pstSwapRight = pstSwapNode->pstRight;
    VosAvlUpdateSwapNode(pstTree, pstSwapNode, pstNode);
    VosAvlMoveNodeToNewPos(pstNode, pstSwapParent, AVL_NULL_PTR, pstSwapRight);
    pstNode->pstParent->pstLeft = pstNode;
    return;
}

void VosAvlRebalance(AVLBASE_NODE_S **ppstSubTree)
{
    int iMoment;
    iMoment = (*ppstSubTree)->sRHeight - (*ppstSubTree)->sLHeight;
    if (iMoment > 1)
    {
        if ((*ppstSubTree)->pstRight->sLHeight > (*ppstSubTree)->pstRight->sRHeight)
        {
            VosAvlRotateRight(&(*ppstSubTree)->pstRight);
        }
        VosAvlRotateLeft(ppstSubTree);
    }
    else if (iMoment < -1)
    {
        if ((*ppstSubTree)->pstLeft->sRHeight > (*ppstSubTree)->pstLeft->sLHeight)
        {
            VosAvlRotateLeft(&(*ppstSubTree)->pstLeft);
        }
        VosAvlRotateRight(ppstSubTree);
    }
    return;
}

void VosAvlBalanceTree(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstNodeTmp = pstNode;
    while (pstNodeTmp->pstParent != AVL_NULL_PTR)
    {
        if (pstNodeTmp->pstParent->pstRight == pstNodeTmp)
        {
            pstNodeTmp = pstNodeTmp->pstParent;
            VosAvlRebalance(&pstNodeTmp->pstRight);
            pstNodeTmp->sRHeight = (1 + VOS_V2_AVL_MAX(pstNodeTmp->pstRight->sRHeight, pstNodeTmp->pstRight->sLHeight));
        }
        else
        {
            pstNodeTmp = pstNodeTmp->pstParent;
            VosAvlRebalance(&pstNodeTmp->pstLeft);
            pstNodeTmp->sLHeight = (1 + VOS_V2_AVL_MAX(pstNodeTmp->pstLeft->sRHeight, pstNodeTmp->pstLeft->sLHeight));
        }
    }
    if (pstNodeTmp->sLHeight != pstNodeTmp->sRHeight)
    {
        VosAvlRebalance(&pstTree->pstRoot);
    }
    return;
}

AVLBASE_NODE_S *VosAVLSearchReplaceNodeInRTree(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstReplaceNode;
    if (pstNode->pstRight->pstLeft == AVL_NULL_PTR)
    {
        pstReplaceNode = pstNode->pstRight;
        pstReplaceNode->pstLeft = pstNode->pstLeft;
        pstReplaceNode->pstLeft->pstParent = pstReplaceNode;
        pstReplaceNode->sLHeight = pstNode->sLHeight;
    }
    else
    {
        VosAvlSwapLeftMost(pstTree, pstNode->pstRight, pstNode);
        pstReplaceNode = pstNode->pstRight;
    }
    return pstReplaceNode;
}

AVLBASE_NODE_S *VosAvlSearchReplaceNodeInLTree(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstReplaceNode;
    if (pstNode->pstLeft->pstRight == AVL_NULL_PTR)
    {
        pstReplaceNode = pstNode->pstLeft;
        pstReplaceNode->pstRight = pstNode->pstRight;
        pstReplaceNode->pstRight->pstParent = pstReplaceNode;
        pstReplaceNode->sRHeight = pstNode->sRHeight;
    }
    else
    {
        VosAvlSwapRightMost(pstTree, pstNode->pstLeft, pstNode);
        pstReplaceNode = pstNode->pstLeft;
    }
    return pstReplaceNode;
}

AVLBASE_NODE_S *VosAvlSearchReplaceNode(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstReplaceNode;
    if (pstNode->sRHeight > pstNode->sLHeight)
    {
        pstReplaceNode = VosAVLSearchReplaceNodeInRTree(pstTree, pstNode);
    }
    else
    {
        pstReplaceNode = VosAvlSearchReplaceNodeInLTree(pstTree, pstNode);
    }
    return pstReplaceNode;
}

AVLBASE_NODE_S *VosAvlDeleteCheck(AVLBASE_TREE_S *pstTree, AVLBASE_NODE_S *pstNode)
{
    AVLBASE_NODE_S *pstReplaceNode;
    if ((pstNode->pstLeft == AVL_NULL_PTR) && (pstNode->pstRight == AVL_NULL_PTR))
    {
        pstReplaceNode = AVL_NULL_PTR;
        if (pstTree->pstFirst == pstNode)
        {
            pstTree->pstFirst = pstNode->pstParent;
        }
        if (pstTree->pstLast == pstNode)
        {
            pstTree->pstLast = pstNode->pstParent;
        }
    }
    else if (pstNode->pstLeft == AVL_NULL_PTR)
    {
        pstReplaceNode = pstNode->pstRight;
        if (pstTree->pstFirst == pstNode)
        {
            pstTree->pstFirst = pstReplaceNode;
        }
    }
    else if (pstNode->pstRight == AVL_NULL_PTR)
    {
        pstReplaceNode = pstNode->pstLeft;
        if (pstTree->pstLast == pstNode)
        {
            pstTree->pstLast = pstReplaceNode;
        }
    }
    else
    {
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
    if (pstReplaceNode != AVL_NULL_PTR)
    {
        pstReplaceNode->pstParent = pstParentNode;
        sNewHeight = (1 + VOS_V2_AVL_MAX(pstReplaceNode->sLHeight, pstReplaceNode->sRHeight));
    }
    if (pstParentNode != AVL_NULL_PTR)
    {
        if (pstParentNode->pstRight == pstBaseNode)
        {
            pstParentNode->pstRight = pstReplaceNode;
            pstParentNode->sRHeight = sNewHeight;
        }
        else
        {
            pstParentNode->pstLeft = pstReplaceNode;
            pstParentNode->sLHeight = sNewHeight;
        }
        VosAvlBalanceTree(pstBaseTree, pstParentNode);
    }
    else
    {
        pstBaseTree->pstRoot = pstReplaceNode;
    }
    return;
}

void *VOS_AVL_Insert_Or_Find(AVL_TREE *pstTree, AVL_NODE *pstNode)
{
    AVL_NODE *pstParentNode;
    int iResult;
    if ((pstTree == AVL_NULL_PTR) || (pstNode == AVL_NULL_PTR) || (VOS_AVL_IN_TREE(*pstNode)))
    {
        return AVL_NULL_PTR;
    }
    pstNode->sRHeight = 0;
    pstNode->sLHeight = 0;
    if (pstTree->pstRoot == AVL_NULL_PTR)
    {
        pstTree->pstRoot = pstNode;
        pstTree->pstFirst = pstNode;
        pstTree->pstLast = pstNode;
        return AVL_NULL_PTR;
    }
    for (pstParentNode = pstTree->pstRoot; pstParentNode != AVL_NULL_PTR;)
    {
        iResult = pstTree->pfnCompare(pstNode->pKey, pstParentNode->pKey);
        if (iResult > 0)
        {
            if (pstParentNode->pstRight != AVL_NULL_PTR)
            {
                pstParentNode = pstParentNode->pstRight;
                continue;
            }
            VosAvlNodeRightInsert((AVLBASE_TREE_S *)(void *)(&(pstTree->pstRoot)), (AVLBASE_NODE_S *)pstParentNode,
                                  (AVLBASE_NODE_S *)pstNode);
            break;
        }
        else if (iResult < 0)
        {
            if (pstParentNode->pstLeft != AVL_NULL_PTR)
            {
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
    if (pstParentNode != AVL_NULL_PTR)
    {
        VosAvlBalanceTree((AVLBASE_TREE_S *)(void *)(&(pstTree->pstRoot)), (AVLBASE_NODE_S *)pstParentNode);
    }
    return AVL_NULL_PTR;
}

void VOS_AVL_Delete(AVL_TREE *pstTree, AVL_NODE *pstNode)
{
    AVLBASE_NODE_S *pstBaseNode;
    AVLBASE_TREE_S *pstBaseTree;
    if ((pstTree == AVL_NULL_PTR) || (pstNode == AVL_NULL_PTR) || (!VOS_AVL_IN_TREE(*pstNode)))
    {
        return;
    }
    pstBaseNode = (AVLBASE_NODE_S *)pstNode;
    pstBaseTree = (AVLBASE_TREE_S *)(void *)(&(pstTree->pstRoot));
    VosAvlDelete(pstBaseNode, pstBaseTree);
    return;
}

void *VOS_AVL_Find(AVL_TREE *pstTree, const void *pKey)
{
    AVL_NODE *pstNode;
    int iResult;
    if (pstTree == AVL_NULL_PTR)
    {
        return AVL_NULL_PTR;
    }
    pstNode = pstTree->pstRoot;
    while (pstNode != AVL_NULL_PTR)
    {
        iResult = pstTree->pfnCompare(pKey, pstNode->pKey);
        if (iResult > 0)
        {
            pstNode = pstNode->pstRight;
        }
        else if (iResult < 0)
        {
            pstNode = pstNode->pstLeft;
        }
        else
        {
            break;
        }
    }
    return ((pstNode != AVL_NULL_PTR) ? pstNode->pSelf : AVL_NULL_PTR);
}

void *VOS_AVL_Next(AVL_NODE *pstNode)
{
    AVL_NODE *pstNodeTmp = pstNode;
    if ((pstNodeTmp == AVL_NULL_PTR) || (!VOS_AVL_IN_TREE(*pstNodeTmp)))
    {
        return AVL_NULL_PTR;
    }
    if (pstNodeTmp->pstRight != AVL_NULL_PTR)
    {
        pstNodeTmp = pstNodeTmp->pstRight;
        FIND_LEFTMOST_NODE(pstNodeTmp);
    }
    else
    {
        while (pstNodeTmp != AVL_NULL_PTR)
        {
            if ((pstNodeTmp->pstParent == AVL_NULL_PTR) || (pstNodeTmp->pstParent->pstLeft == pstNodeTmp))
            {
                pstNodeTmp = pstNodeTmp->pstParent;
                break;
            }
            pstNodeTmp = pstNodeTmp->pstParent;
        }
    }
    return ((pstNodeTmp != AVL_NULL_PTR) ? pstNodeTmp->pSelf : AVL_NULL_PTR);
}

void *VOS_AVL_Prev(AVL_NODE *pstNode)
{
    AVL_NODE *pstNodeTmp = pstNode;
    if ((pstNodeTmp == AVL_NULL_PTR) || (!VOS_AVL_IN_TREE(*pstNodeTmp)))
    {
        return AVL_NULL_PTR;
    }
    if (pstNodeTmp->pstLeft != AVL_NULL_PTR)
    {
        pstNodeTmp = pstNodeTmp->pstLeft;
        FIND_RIGHTMOST_NODE(pstNodeTmp);
    }
    else
    {
        while (pstNodeTmp != AVL_NULL_PTR)
        {
            if ((pstNodeTmp->pstParent == AVL_NULL_PTR) || (pstNodeTmp->pstParent->pstRight == pstNodeTmp))
            {
                pstNodeTmp = pstNodeTmp->pstParent;
                break;
            }
            pstNodeTmp = pstNodeTmp->pstParent;
        }
    }
    return ((pstNodeTmp != AVL_NULL_PTR) ? pstNodeTmp->pSelf : AVL_NULL_PTR);
}

void *VOS_AVL_Find_Or_Find_Next(AVL_TREE *pstTree, const void *pKey, unsigned int bValue)
{
    AVL_NODE *pstNode;
    void *pFoundNode = AVL_NULL_PTR;
    int iResult;
    if (pstTree == AVL_NULL_PTR)
    {
        return AVL_NULL_PTR;
    }
    pstNode = pstTree->pstRoot;
    if (pstNode == AVL_NULL_PTR)
    {
        return (pFoundNode);
    }
    for (;;)
    {
        iResult = pstTree->pfnCompare(pKey, pstNode->pKey);
        if (iResult > 0)
        {
            if (pstNode->pstRight == AVL_NULL_PTR)
            {
                pFoundNode = VOS_AVL_Next(pstNode);
                break;
            }
            pstNode = pstNode->pstRight;
        }
        else if (iResult < 0)
        {
            if (pstNode->pstLeft == AVL_NULL_PTR)
            {
                pFoundNode = pstNode->pSelf;
                break;
            }
            pstNode = pstNode->pstLeft;
        }
        else
        {
            if (bValue != 0)
            {
                pFoundNode = VOS_AVL_Next(pstNode);
            }
            else
            {
                pFoundNode = pstNode->pSelf;
            }
            break;
        }
    }
    return (pFoundNode);
}

