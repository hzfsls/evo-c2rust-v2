# 1 ".tmp/tmp_files/include/v_avl_base.h"
# 19 ".tmp/tmp_files/include/v_avl_base.h"
#ifndef V_AVL_BASE_H
#define V_AVL_BASE_H 

#include "avl_adapt.h"

#ifdef __cplusplus
extern "C" {
#endif
# 37 ".tmp/tmp_files/include/v_avl_base.h"
typedef long (*AVL_V2_COMPARE_FUNC)(const void *, const void *);
# 48 ".tmp/tmp_files/include/v_avl_base.h"
#pragma pack(4)
typedef struct avl_node {
    struct avl_node *pstParent;
    struct avl_node *pstLeft;
    struct avl_node *pstRight;
    short int sLHeight;
    short int sRHeight;
    void *pSelf;
    void *pKey;
} AVL_NODE;
#if defined(AVL_PACKEND_ZERO)

#pragma pack(0)
#else
#pragma pack()
#endif







typedef struct avl_tree {
    AVL_V2_COMPARE_FUNC pfnCompare;
    AVL_NODE *pstRoot;
    AVL_NODE *pstFirst;
    AVL_NODE *pstLast;
} AVL_TREE;
# 101 ".tmp/tmp_files/include/v_avl_base.h"
extern void *VOS_AVL_Insert_Or_Find(AVL_TREE *pstTree, AVL_NODE *pstNode);
# 121 ".tmp/tmp_files/include/v_avl_base.h"
extern void VOS_AVL_Delete(AVL_TREE *pstTree, AVL_NODE *pstNode);
# 141 ".tmp/tmp_files/include/v_avl_base.h"
extern void *VOS_AVL_Find(AVL_TREE *pstTree, const void *pKey);
# 173 ".tmp/tmp_files/include/v_avl_base.h"
extern void *VOS_AVL_Find_Or_Find_Next(AVL_TREE *pstTree, const void *pKey, unsigned int bValue);
# 197 ".tmp/tmp_files/include/v_avl_base.h"
extern void *VOS_AVL_Next(AVL_NODE *pstNode);
# 220 ".tmp/tmp_files/include/v_avl_base.h"
extern void *VOS_AVL_Prev(AVL_NODE *pstNode);






#define VOS_AVL_INIT_TREE(TREE,COMPARE) do { (TREE).pfnCompare = (COMPARE); (TREE).pstFirst = (AVL_NODE *)AVL_NULL_PTR; (TREE).pstLast = (AVL_NODE *)AVL_NULL_PTR; (TREE).pstRoot = (AVL_NODE *)AVL_NULL_PTR; } while (0)





#define VOS_AVL_INIT_NODE(NODE,SELF,KEY) do { (NODE).pstParent = (AVL_NODE *)AVL_NULL_PTR; (NODE).pstLeft = (AVL_NODE *)AVL_NULL_PTR; (NODE).pstRight = (AVL_NODE *)AVL_NULL_PTR; (NODE).pSelf = (SELF); (NODE).pKey = (KEY); (NODE).sLHeight = -1; (NODE).sRHeight = -1; } while (0)






#define VOS_AVL_INSERT(TREE,NODE) (VOS_AVL_Insert_Or_Find(&(TREE), &(NODE)) == AVL_NULL_PTR)




#define VOS_AVL_INSERT_OR_FIND(TREE,NODE) VOS_AVL_Insert_Or_Find(&(TREE), &(NODE))




#define VOS_AVL_DELETE(TREE,NODE) VOS_AVL_Delete(&(TREE), &(NODE))




#define VOS_AVL_FIND(TREE,KEY) VOS_AVL_Find(&(TREE), (KEY))




#define VOS_AVL_NEXT(NODE) VOS_AVL_Next(&(NODE))




#define VOS_AVL_PREV(NODE) VOS_AVL_Prev(&(NODE))




#define VOS_AVL_FIRST(TREE) (((&(TREE))->pstFirst != (AVL_NODE *)AVL_NULL_PTR) ? (&(TREE))->pstFirst->pSelf : AVL_NULL_PTR)




#define VOS_AVL_LAST(TREE) (((&(TREE))->pstLast != (AVL_NODE *)AVL_NULL_PTR) ? (&(TREE))->pstLast->pSelf : AVL_NULL_PTR)




#define VOS_AVL_IN_TREE(NODE) (((NODE).sLHeight != -1) && ((NODE).sRHeight != -1))




#define VOS_AVL_FIND_NEXT(TREE,KEY) VOS_AVL_Find_Or_Find_Next(&(TREE), (KEY), AVL_TRUE)




#define VOS_AVL_FIND_OR_FIND_NEXT(TREE,KEY) VOS_AVL_Find_Or_Find_Next(&(TREE), (KEY), AVL_FALSE)


#define VOS_V2_AVL_MAX(X,Y) (((X) > (Y)) ? (X) : (Y))

#ifdef __cplusplus
}
#endif

#endif
