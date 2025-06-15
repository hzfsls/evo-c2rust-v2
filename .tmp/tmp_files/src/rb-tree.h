# 1 ".tmp/tmp_files/src/rb-tree.h"
# 55 ".tmp/tmp_files/src/rb-tree.h"
#ifndef ALGORITHM_RB_TREE_H
#define ALGORITHM_RB_TREE_H 

#ifdef __cplusplus
extern "C" {
#endif







typedef struct _RBTree RBTree;





typedef void *RBTreeKey;





typedef void *RBTreeValue;





#define RB_TREE_NULL ((void *) 0)
# 98 ".tmp/tmp_files/src/rb-tree.h"
typedef struct _RBTreeNode RBTreeNode;
# 111 ".tmp/tmp_files/src/rb-tree.h"
typedef int (*RBTreeCompareFunc)(RBTreeValue data1, RBTreeValue data2);





typedef enum {
 RB_TREE_NODE_RED,
 RB_TREE_NODE_BLACK,
} RBTreeNodeColor;





typedef enum {
 RB_TREE_NODE_LEFT = 0,
 RB_TREE_NODE_RIGHT = 1
} RBTreeNodeSide;
# 139 ".tmp/tmp_files/src/rb-tree.h"
RBTree *rb_tree_new(RBTreeCompareFunc compare_func);







void rb_tree_free(RBTree *tree);
# 160 ".tmp/tmp_files/src/rb-tree.h"
RBTreeNode *rb_tree_insert(RBTree *tree, RBTreeKey key, RBTreeValue value);
# 169 ".tmp/tmp_files/src/rb-tree.h"
void rb_tree_remove_node(RBTree *tree, RBTreeNode *node);
# 182 ".tmp/tmp_files/src/rb-tree.h"
int rb_tree_remove(RBTree *tree, RBTreeKey key);
# 194 ".tmp/tmp_files/src/rb-tree.h"
RBTreeNode *rb_tree_lookup_node(RBTree *tree, RBTreeKey key);
# 209 ".tmp/tmp_files/src/rb-tree.h"
RBTreeValue rb_tree_lookup(RBTree *tree, RBTreeKey key);
# 219 ".tmp/tmp_files/src/rb-tree.h"
RBTreeNode *rb_tree_root_node(RBTree *tree);
# 228 ".tmp/tmp_files/src/rb-tree.h"
RBTreeKey rb_tree_node_key(RBTreeNode *node);
# 237 ".tmp/tmp_files/src/rb-tree.h"
RBTreeValue rb_tree_node_value(RBTreeNode *node);
# 248 ".tmp/tmp_files/src/rb-tree.h"
RBTreeNode *rb_tree_node_child(RBTreeNode *node, RBTreeNodeSide side);
# 258 ".tmp/tmp_files/src/rb-tree.h"
RBTreeNode *rb_tree_node_parent(RBTreeNode *node);
# 267 ".tmp/tmp_files/src/rb-tree.h"
int rb_tree_subtree_height(RBTreeNode *node);
# 280 ".tmp/tmp_files/src/rb-tree.h"
RBTreeValue *rb_tree_to_array(RBTree *tree);
# 289 ".tmp/tmp_files/src/rb-tree.h"
int rb_tree_num_entries(RBTree *tree);

#ifdef __cplusplus
}
#endif

#endif
