# 1 ".tmp/tmp_files/include/v_avl3.h"
# 20 ".tmp/tmp_files/include/v_avl3.h"
#ifndef V_AVL3_H
#define V_AVL3_H 

#include "avl_adapt.h"

#ifdef __cplusplus
extern "C" {
#endif
# 37 ".tmp/tmp_files/include/v_avl3.h"
typedef long (*AVL3_COMPARE)(const void *, const void *);
# 48 ".tmp/tmp_files/include/v_avl3.h"
#pragma pack(4)
typedef struct avl3_node {
    struct avl3_node *pstParent;
    struct avl3_node *pstLeft;
    struct avl3_node *pstRight;
    short int sLHeight;
    short int sRHeight;
} AVL3_NODE;
#if defined(AVL_PACKEND_ZERO)

#pragma pack(0)
#else
#pragma pack()
#endif







#pragma pack(4)
typedef struct avl3_tree_info {
    AVL3_COMPARE pfCompare;
    unsigned short int usKeyOffset;
    unsigned short int usNodeOffset;
} AVL3_TREE_INFO;
#if defined(AVL_PACKEND_ZERO)

#pragma pack(0)
#else
#pragma pack()
#endif
# 89 ".tmp/tmp_files/include/v_avl3.h"
typedef struct avl3_tree {
    AVL3_NODE *pstRoot;
    AVL3_NODE *pstFirst;
    AVL3_NODE *pstLast;
} AVL3_TREE;
# 121 ".tmp/tmp_files/include/v_avl3.h"
extern void *VOS_AVL3_Insert_Or_Find(AVL3_TREE *pstTree, AVL3_NODE *pstNode, AVL3_TREE_INFO *pstTreeInfo);
# 142 ".tmp/tmp_files/include/v_avl3.h"
extern void VOS_AVL3_Delete(AVL3_TREE *pstTree, AVL3_NODE *pstNode);
# 166 ".tmp/tmp_files/include/v_avl3.h"
extern void *VOS_AVL3_Find(AVL3_TREE *pstTree, const void *pstKey, AVL3_TREE_INFO *pstTreeInfo);
# 195 ".tmp/tmp_files/include/v_avl3.h"
extern void *AVL3_Find_Or_Find_Next(AVL3_TREE *pstTree, const void *pKey,
                                    unsigned int bFlag, AVL3_TREE_INFO *pstTreeInfo);
# 218 ".tmp/tmp_files/include/v_avl3.h"
extern void *VOS_AVL3_First(AVL3_TREE *pstTree, AVL3_TREE_INFO *pstTreeInfo);
# 241 ".tmp/tmp_files/include/v_avl3.h"
extern void *VOS_AVL3_Last(AVL3_TREE *pstTree, AVL3_TREE_INFO *pstTreeInfo);
# 266 ".tmp/tmp_files/include/v_avl3.h"
extern void *VOS_AVL3_Next(AVL3_NODE *pstNode, AVL3_TREE_INFO *pstTreeInfo);
# 290 ".tmp/tmp_files/include/v_avl3.h"
extern void *VOS_AVL3_Prev(AVL3_NODE *pstNode, AVL3_TREE_INFO *pstTreeInfo);


#define VOS_AVL3_INIT_TREE(TREE,TREE_INFO) do { (TREE).pstFirst = (AVL3_NODE *)AVL_NULL_PTR; (TREE).pstLast = (AVL3_NODE *)AVL_NULL_PTR; (TREE).pstRoot = (AVL3_NODE *)AVL_NULL_PTR; } while (0)

#define VOS_AVL3_INIT_NODE(NODE) do { (NODE).pstParent = (AVL3_NODE *)AVL_NULL_PTR; (NODE).pstLeft = (AVL3_NODE *)AVL_NULL_PTR; (NODE).pstRight = (AVL3_NODE *)AVL_NULL_PTR; (NODE).sLHeight = -1; (NODE).sRHeight = -1; } while (0)

#define VOS_AVL3_INSERT(TREE,NODE,TREE_INFO) (AVL_NULL_PTR == VOS_AVL3_Insert_Or_Find(&(TREE), &(NODE), &(TREE_INFO)))

#define VOS_AVL3_INSERT_OR_FIND(TREE,NODE,TREE_INFO) VOS_AVL3_Insert_Or_Find(&(TREE), &(NODE), &(TREE_INFO))

#define VOS_AVL3_DELETE(TREE,NODE) VOS_AVL3_Delete(&(TREE), &(NODE))

#define VOS_AVL3_FIND(TREE,KEY,TREE_INFO) VOS_AVL3_Find(&(TREE), (KEY), &(TREE_INFO))

#define VOS_AVL3_NEXT(NODE,TREE_INFO) VOS_AVL3_Next(&(NODE), &(TREE_INFO))

#define VOS_AVL3_PREV(NODE,TREE_INFO) VOS_AVL3_Prev(&(NODE), &(TREE_INFO))

#define VOS_AVL3_FIRST(TREE,TREE_INFO) VOS_AVL3_First(&(TREE), &(TREE_INFO))

#define VOS_AVL3_LAST(TREE,TREE_INFO) VOS_AVL3_Last(&(TREE), &(TREE_INFO))

#define VOS_AVL3_IN_TREE(NODE) (((NODE).sLHeight != -1) && ((NODE).sRHeight != -1))

#define VOS_AVL3_FIND_NEXT(TREE,KEY,TREE_INFO) AVL3_Find_Or_Find_Next(&(TREE), (KEY), AVL_TRUE, &(TREE_INFO))

#define VOS_AVL3_FIND_OR_FIND_NEXT(TREE,KEY,TREE_INFO) AVL3_Find_Or_Find_Next(&(TREE), (KEY), AVL_FALSE, &(TREE_INFO))


#define VOS_AVL3_MAX(X,Y) (((X) > (Y)) ? (X) : (Y))

#ifdef __cplusplus
}
#endif

#endif
