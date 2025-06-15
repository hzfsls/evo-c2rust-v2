# 1 ".tmp/tmp_files/src/avl-tree.c"
# 21 ".tmp/tmp_files/src/avl-tree.c"
#include <stdlib.h>

#include "avl-tree.h"



#ifdef ALLOC_TESTING
#include "alloc-testing.h"
#endif



struct _AVLTreeNode {
 AVLTreeNode *children[2];
 AVLTreeNode *parent;
 AVLTreeKey key;
 AVLTreeValue value;
 int height;
};

struct _AVLTree {
 AVLTreeNode *root_node;
 AVLTreeCompareFunc compare_func;
 unsigned int num_nodes;
};

AVLTree *avl_tree_new(AVLTreeCompareFunc compare_func)
{
 AVLTree *new_tree;

 new_tree = (AVLTree *) malloc(sizeof(AVLTree));

 if (new_tree == NULL) {
  return NULL;
 }

 new_tree->root_node = NULL;
 new_tree->compare_func = compare_func;
 new_tree->num_nodes = 0;

 return new_tree;
}

static void avl_tree_free_subtree(AVLTree *tree, AVLTreeNode *node)
{
 if (node == NULL) {
  return;
 }

 avl_tree_free_subtree(tree, node->children[AVL_TREE_NODE_LEFT]);
 avl_tree_free_subtree(tree, node->children[AVL_TREE_NODE_RIGHT]);

 free(node);
}

void avl_tree_free(AVLTree *tree)
{


 avl_tree_free_subtree(tree, tree->root_node);



 free(tree);
}

int avl_tree_subtree_height(AVLTreeNode *node)
{
 if (node == NULL) {
  return 0;
 } else {
  return node->height;
 }
}





static void avl_tree_update_height(AVLTreeNode *node)
{
 AVLTreeNode *left_subtree;
 AVLTreeNode *right_subtree;
 int left_height, right_height;

 left_subtree = node->children[AVL_TREE_NODE_LEFT];
 right_subtree = node->children[AVL_TREE_NODE_RIGHT];
 left_height = avl_tree_subtree_height(left_subtree);
 right_height = avl_tree_subtree_height(right_subtree);

 if (left_height > right_height) {
  node->height = left_height + 1;
 } else {
  node->height = right_height + 1;
 }
}



static AVLTreeNodeSide avl_tree_node_parent_side(AVLTreeNode *node)
{
 if (node->parent->children[AVL_TREE_NODE_LEFT] == node) {
  return AVL_TREE_NODE_LEFT;
 } else {
  return AVL_TREE_NODE_RIGHT;
 }
}



static void avl_tree_node_replace(AVLTree *tree, AVLTreeNode *node1,
                                  AVLTreeNode *node2)
{
 int side;



 if (node2 != NULL) {
  node2->parent = node1->parent;
 }



 if (node1->parent == NULL) {
  tree->root_node = node2;
 } else {
  side = avl_tree_node_parent_side(node1);
  node1->parent->children[side] = node2;

  avl_tree_update_height(node1->parent);
 }
}
# 172 ".tmp/tmp_files/src/avl-tree.c"
static AVLTreeNode *avl_tree_rotate(AVLTree *tree, AVLTreeNode *node,
                                    AVLTreeNodeSide direction)
{
 AVLTreeNode *new_root;




 new_root = node->children[1-direction];



 avl_tree_node_replace(tree, node, new_root);



 node->children[1-direction] = new_root->children[direction];
 new_root->children[direction] = node;



 node->parent = new_root;

 if (node->children[1-direction] != NULL) {
  node->children[1-direction]->parent = node;
 }



 avl_tree_update_height(new_root);
 avl_tree_update_height(node);

 return new_root;
}







static AVLTreeNode *avl_tree_node_balance(AVLTree *tree, AVLTreeNode *node)
{
 AVLTreeNode *left_subtree;
 AVLTreeNode *right_subtree;
 AVLTreeNode *child;
 int diff;

 left_subtree = node->children[AVL_TREE_NODE_LEFT];
 right_subtree = node->children[AVL_TREE_NODE_RIGHT];





 diff = avl_tree_subtree_height(right_subtree)
      - avl_tree_subtree_height(left_subtree);

 if (diff >= 2) {



  child = right_subtree;

  if (avl_tree_subtree_height(child->children[AVL_TREE_NODE_RIGHT])
    < avl_tree_subtree_height(child->children[AVL_TREE_NODE_LEFT])) {





   avl_tree_rotate(tree, right_subtree,
                   AVL_TREE_NODE_RIGHT);
  }




  node = avl_tree_rotate(tree, node, AVL_TREE_NODE_LEFT);

 } else if (diff <= -2) {



  child = node->children[AVL_TREE_NODE_LEFT];

  if (avl_tree_subtree_height(child->children[AVL_TREE_NODE_LEFT])
    < avl_tree_subtree_height(child->children[AVL_TREE_NODE_RIGHT])) {





   avl_tree_rotate(tree, left_subtree,
                   AVL_TREE_NODE_LEFT);
  }




  node = avl_tree_rotate(tree, node, AVL_TREE_NODE_RIGHT);
 }



 avl_tree_update_height(node);

 return node;
}



static void avl_tree_balance_to_root(AVLTree *tree, AVLTreeNode *node)
{
 AVLTreeNode *rover;

 rover = node;

 while (rover != NULL) {



  rover = avl_tree_node_balance(tree, rover);



  rover = rover->parent;
 }
}

AVLTreeNode *avl_tree_insert(AVLTree *tree, AVLTreeKey key, AVLTreeValue value)
{
 AVLTreeNode **rover;
 AVLTreeNode *new_node;
 AVLTreeNode *previous_node;



 rover = &tree->root_node;
 previous_node = NULL;

 while (*rover != NULL) {
  previous_node = *rover;
  if (tree->compare_func(key, (*rover)->key) < 0) {
   rover = &((*rover)->children[AVL_TREE_NODE_LEFT]);
  } else {
   rover = &((*rover)->children[AVL_TREE_NODE_RIGHT]);
  }
 }



 new_node = (AVLTreeNode *) malloc(sizeof(AVLTreeNode));

 if (new_node == NULL) {
  return NULL;
 }

 new_node->children[AVL_TREE_NODE_LEFT] = NULL;
 new_node->children[AVL_TREE_NODE_RIGHT] = NULL;
 new_node->parent = previous_node;
 new_node->key = key;
 new_node->value = value;
 new_node->height = 1;



 *rover = new_node;



 avl_tree_balance_to_root(tree, previous_node);



 ++tree->num_nodes;

 return new_node;
}





static AVLTreeNode *avl_tree_node_get_replacement(AVLTree *tree,
                                                  AVLTreeNode *node)
{
 AVLTreeNode *left_subtree;
 AVLTreeNode *right_subtree;
 AVLTreeNode *result;
 AVLTreeNode *child;
 int left_height, right_height;
 int side;

 left_subtree = node->children[AVL_TREE_NODE_LEFT];
 right_subtree = node->children[AVL_TREE_NODE_RIGHT];



 if (left_subtree == NULL && right_subtree == NULL) {
  return NULL;
 }




 left_height = avl_tree_subtree_height(left_subtree);
 right_height = avl_tree_subtree_height(right_subtree);

 if (left_height < right_height) {
  side = AVL_TREE_NODE_RIGHT;
 } else {
  side = AVL_TREE_NODE_LEFT;
 }



 result = node->children[side];

 while (result->children[1-side] != NULL) {
  result = result->children[1-side];
 }




 child = result->children[side];
 avl_tree_node_replace(tree, result, child);



 avl_tree_update_height(result->parent);

 return result;
}



void avl_tree_remove_node(AVLTree *tree, AVLTreeNode *node)
{
 AVLTreeNode *swap_node;
 AVLTreeNode *balance_startpoint;
 int i;





 swap_node = avl_tree_node_get_replacement(tree, node);

 if (swap_node == NULL) {






  avl_tree_node_replace(tree, node, NULL);



  balance_startpoint = node->parent;

 } else {





  if (swap_node->parent == node) {
   balance_startpoint = swap_node;
  } else {
   balance_startpoint = swap_node->parent;
  }



  for (i=0; i<2; ++i) {
   swap_node->children[i] = node->children[i];

   if (swap_node->children[i] != NULL) {
    swap_node->children[i]->parent = swap_node;
   }
  }

  swap_node->height = node->height;



  avl_tree_node_replace(tree, node, swap_node);
 }



 free(node);



 --tree->num_nodes;



 avl_tree_balance_to_root(tree, balance_startpoint);
}



int avl_tree_remove(AVLTree *tree, AVLTreeKey key)
{
 AVLTreeNode *node;



 node = avl_tree_lookup_node(tree, key);

 if (node == NULL) {


  return 0;
 }



 avl_tree_remove_node(tree, node);

 return 1;
}

AVLTreeNode *avl_tree_lookup_node(AVLTree *tree, AVLTreeKey key)
{
 AVLTreeNode *node;
 int diff;




 node = tree->root_node;

 while (node != NULL) {

  diff = tree->compare_func(key, node->key);

  if (diff == 0) {



   return node;

  } else if (diff < 0) {
   node = node->children[AVL_TREE_NODE_LEFT];
  } else {
   node = node->children[AVL_TREE_NODE_RIGHT];
  }
 }



 return NULL;
}

AVLTreeValue avl_tree_lookup(AVLTree *tree, AVLTreeKey key)
{
 AVLTreeNode *node;



 node = avl_tree_lookup_node(tree, key);

 if (node == NULL) {
  return AVL_TREE_NULL;
 } else {
  return node->value;
 }
}

AVLTreeNode *avl_tree_root_node(AVLTree *tree)
{
 return tree->root_node;
}

AVLTreeKey avl_tree_node_key(AVLTreeNode *node)
{
 return node->key;
}

AVLTreeValue avl_tree_node_value(AVLTreeNode *node)
{
 return node->value;
}

AVLTreeNode *avl_tree_node_child(AVLTreeNode *node, AVLTreeNodeSide side)
{
 if (side == AVL_TREE_NODE_LEFT || side == AVL_TREE_NODE_RIGHT) {
  return node->children[side];
 } else {
  return NULL;
 }
}

AVLTreeNode *avl_tree_node_parent(AVLTreeNode *node)
{
 return node->parent;
}

unsigned int avl_tree_num_entries(AVLTree *tree)
{
 return tree->num_nodes;
}

static void avl_tree_to_array_add_subtree(AVLTreeNode *subtree,
                                         AVLTreeValue *array,
                                         int *index)
{
 if (subtree == NULL) {
  return;
 }



 avl_tree_to_array_add_subtree(subtree->children[AVL_TREE_NODE_LEFT],
                               array, index);



 array[*index] = subtree->key;
 ++*index;



 avl_tree_to_array_add_subtree(subtree->children[AVL_TREE_NODE_RIGHT],
                               array, index);
}

AVLTreeValue *avl_tree_to_array(AVLTree *tree)
{
 AVLTreeValue *array;
 int index;



 array = malloc(sizeof(AVLTreeValue) * tree->num_nodes);

 if (array == NULL) {
  return NULL;
 }

 index = 0;



 avl_tree_to_array_add_subtree(tree->root_node, array, &index);

 return array;
}
