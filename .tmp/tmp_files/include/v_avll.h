# 1 ".tmp/tmp_files/include/v_avll.h"
# 19 ".tmp/tmp_files/include/v_avll.h"
#ifndef V_AVLL_H
#define V_AVLL_H 

#include "v_avl3.h"

#ifdef __cplusplus
extern "C" {
#endif


typedef AVL3_COMPARE AVLL_COMPARE;
# 39 ".tmp/tmp_files/include/v_avll.h"
typedef AVL3_NODE AVLL_NODE;
# 50 ".tmp/tmp_files/include/v_avll.h"
typedef struct avll_tree {
    AVL3_TREE stTree;
    AVL3_TREE_INFO stTreeInfo;
} AVLL_TREE;
# 76 ".tmp/tmp_files/include/v_avll.h"
#define VOS_AVLL_INIT_TREE(TREE,COMPARE,KEY_OFF,NODE_OFF) do { (TREE).stTreeInfo.pfCompare = (COMPARE); (TREE).stTreeInfo.usKeyOffset = (KEY_OFF); (TREE).stTreeInfo.usNodeOffset = (NODE_OFF); VOS_AVL3_INIT_TREE((TREE).stTree, (TREE).stTreeInfo); } while (0)
# 96 ".tmp/tmp_files/include/v_avll.h"
#define VOS_AVLL_INIT_NODE(NODE) VOS_AVL3_INIT_NODE((NODE))
# 119 ".tmp/tmp_files/include/v_avll.h"
#define VOS_AVLL_INSERT(TREE,NODE) VOS_AVL3_INSERT((TREE).stTree, (NODE), (TREE).stTreeInfo)
# 142 ".tmp/tmp_files/include/v_avll.h"
#define VOS_AVLL_INSERT_OR_FIND(TREE,NODE) VOS_AVL3_INSERT_OR_FIND((TREE).stTree, (NODE), (TREE).stTreeInfo)
# 166 ".tmp/tmp_files/include/v_avll.h"
#define VOS_AVLL_DELETE(TREE,NODE) VOS_AVL3_DELETE((TREE).stTree, (NODE))
# 190 ".tmp/tmp_files/include/v_avll.h"
#define VOS_AVLL_FIND(TREE,KEY) VOS_AVL3_FIND((TREE).stTree, (KEY), (TREE).stTreeInfo)
# 217 ".tmp/tmp_files/include/v_avll.h"
#define VOS_AVLL_NEXT(TREE,NODE) VOS_AVL3_NEXT((NODE), (TREE).stTreeInfo)
# 244 ".tmp/tmp_files/include/v_avll.h"
#define VOS_AVLL_PREV(TREE,NODE) VOS_AVL3_PREV((NODE), (TREE).stTreeInfo)
# 268 ".tmp/tmp_files/include/v_avll.h"
#define VOS_AVLL_FIRST(TREE) VOS_AVL3_FIRST((TREE).stTree, (TREE).stTreeInfo)
# 290 ".tmp/tmp_files/include/v_avll.h"
#define VOS_AVLL_LAST(TREE) VOS_AVL3_LAST((TREE).stTree, (TREE).stTreeInfo)
# 313 ".tmp/tmp_files/include/v_avll.h"
#define VOS_AVLL_IN_TREE(NODE) VOS_AVL3_IN_TREE((NODE))
# 342 ".tmp/tmp_files/include/v_avll.h"
#define VOS_AVLL_FIND_NEXT(TREE,KEY) VOS_AVL3_FIND_NEXT((TREE).stTree, (KEY), (TREE).stTreeInfo)
# 369 ".tmp/tmp_files/include/v_avll.h"
#define VOS_AVLL_FIND_OR_FIND_NEXT(TREE,KEY) VOS_AVL3_FIND_OR_FIND_NEXT((TREE).stTree, (KEY), (TREE).stTreeInfo)

#ifdef __cplusplus
}
#endif

#endif
