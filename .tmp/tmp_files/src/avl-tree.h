# 1 ".tmp/tmp_files/src/avl-tree.h"
# 54 ".tmp/tmp_files/src/avl-tree.h"
#ifndef ALGORITHM_AVLTREE_H
#define ALGORITHM_AVLTREE_H 

#ifdef __cplusplus
extern "C" {
#endif







typedef struct _AVLTree AVLTree;





typedef void *AVLTreeKey;





typedef void *AVLTreeValue;





#define AVL_TREE_NULL ((void *) 0)
# 97 ".tmp/tmp_files/src/avl-tree.h"
typedef struct _AVLTreeNode AVLTreeNode;





typedef enum {
 AVL_TREE_NODE_LEFT = 0,
 AVL_TREE_NODE_RIGHT = 1
} AVLTreeNodeSide;
# 119 ".tmp/tmp_files/src/avl-tree.h"
typedef int (*AVLTreeCompareFunc)(AVLTreeValue value1, AVLTreeValue value2);
# 129 ".tmp/tmp_files/src/avl-tree.h"
AVLTree *avl_tree_new(AVLTreeCompareFunc compare_func);







void avl_tree_free(AVLTree *tree);
# 150 ".tmp/tmp_files/src/avl-tree.h"
AVLTreeNode *avl_tree_insert(AVLTree *tree, AVLTreeKey key,
                             AVLTreeValue value);
# 160 ".tmp/tmp_files/src/avl-tree.h"
void avl_tree_remove_node(AVLTree *tree, AVLTreeNode *node);
# 173 ".tmp/tmp_files/src/avl-tree.h"
int avl_tree_remove(AVLTree *tree, AVLTreeKey key);
# 185 ".tmp/tmp_files/src/avl-tree.h"
AVLTreeNode *avl_tree_lookup_node(AVLTree *tree, AVLTreeKey key);
# 200 ".tmp/tmp_files/src/avl-tree.h"
AVLTreeValue avl_tree_lookup(AVLTree *tree, AVLTreeKey key);
# 210 ".tmp/tmp_files/src/avl-tree.h"
AVLTreeNode *avl_tree_root_node(AVLTree *tree);
# 219 ".tmp/tmp_files/src/avl-tree.h"
AVLTreeKey avl_tree_node_key(AVLTreeNode *node);
# 228 ".tmp/tmp_files/src/avl-tree.h"
AVLTreeValue avl_tree_node_value(AVLTreeNode *node);
# 239 ".tmp/tmp_files/src/avl-tree.h"
AVLTreeNode *avl_tree_node_child(AVLTreeNode *node, AVLTreeNodeSide side);
# 249 ".tmp/tmp_files/src/avl-tree.h"
AVLTreeNode *avl_tree_node_parent(AVLTreeNode *node);
# 258 ".tmp/tmp_files/src/avl-tree.h"
int avl_tree_subtree_height(AVLTreeNode *node);
# 271 ".tmp/tmp_files/src/avl-tree.h"
AVLTreeValue *avl_tree_to_array(AVLTree *tree);
# 280 ".tmp/tmp_files/src/avl-tree.h"
unsigned int avl_tree_num_entries(AVLTree *tree);

#ifdef __cplusplus
}
#endif

#endif
