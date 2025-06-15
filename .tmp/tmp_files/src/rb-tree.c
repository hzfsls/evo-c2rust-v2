# 1 ".tmp/tmp_files/src/rb-tree.c"
# 22 ".tmp/tmp_files/src/rb-tree.c"
#include <stdlib.h>

#include "rb-tree.h"



#ifdef ALLOC_TESTING
#include "alloc-testing.h"
#endif

struct _RBTreeNode {
 RBTreeNodeColor color;
 RBTreeKey key;
 RBTreeValue value;
 RBTreeNode *parent;
 RBTreeNode *children[2];
};

struct _RBTree {
 RBTreeNode *root_node;
 RBTreeCompareFunc compare_func;
 int num_nodes;
};

static RBTreeNodeSide rb_tree_node_side(RBTreeNode *node)
{
 if (node->parent->children[RB_TREE_NODE_LEFT] == node) {
  return RB_TREE_NODE_LEFT;
 } else {
  return RB_TREE_NODE_RIGHT;
 }
}

static RBTreeNode *rb_tree_node_sibling(RBTreeNode *node)
{
 RBTreeNodeSide side;

 side = rb_tree_node_side(node);

 return node->parent->children[1 - side];
}

RBTreeNode *rb_tree_node_uncle(RBTreeNode *node)
{
 return rb_tree_node_sibling(node->parent);
}



static void rb_tree_node_replace(RBTree *tree, RBTreeNode *node1,
                                 RBTreeNode *node2)
{
 int side;



 if (node2 != NULL) {
  node2->parent = node1->parent;
 }



 if (node1->parent == NULL) {
  tree->root_node = node2;
 } else {
  side = rb_tree_node_side(node1);
  node1->parent->children[side] = node2;
 }
}
# 110 ".tmp/tmp_files/src/rb-tree.c"
static RBTreeNode *rb_tree_rotate(RBTree *tree, RBTreeNode *node,
                                  RBTreeNodeSide direction)
{
 RBTreeNode *new_root;




 new_root = node->children[1-direction];



 rb_tree_node_replace(tree, node, new_root);



 node->children[1-direction] = new_root->children[direction];
 new_root->children[direction] = node;



 node->parent = new_root;

 if (node->children[1-direction] != NULL) {
  node->children[1-direction]->parent = node;
 }

 return new_root;
}


RBTree *rb_tree_new(RBTreeCompareFunc compare_func)
{
 RBTree *new_tree;

 new_tree = malloc(sizeof(RBTree));

 if (new_tree == NULL) {
  return NULL;
 }

 new_tree->root_node = NULL;
 new_tree->num_nodes = 0;
 new_tree->compare_func = compare_func;

 return new_tree;
}

static void rb_tree_free_subtree(RBTreeNode *node)
{
 if (node != NULL) {


  rb_tree_free_subtree(node->children[RB_TREE_NODE_LEFT]);
  rb_tree_free_subtree(node->children[RB_TREE_NODE_RIGHT]);



  free(node);
 }
}

void rb_tree_free(RBTree *tree)
{


 rb_tree_free_subtree(tree->root_node);



 free(tree);
}

static void rb_tree_insert_case1(RBTree *tree, RBTreeNode *node);
static void rb_tree_insert_case2(RBTree *tree, RBTreeNode *node);
static void rb_tree_insert_case3(RBTree *tree, RBTreeNode *node);
static void rb_tree_insert_case4(RBTree *tree, RBTreeNode *node);
static void rb_tree_insert_case5(RBTree *tree, RBTreeNode *node);




static void rb_tree_insert_case1(RBTree *tree, RBTreeNode *node)
{
 if (node->parent == NULL) {



  node->color = RB_TREE_NODE_BLACK;

 } else {



  rb_tree_insert_case2(tree, node);
 }
}





static void rb_tree_insert_case2(RBTree *tree, RBTreeNode *node)
{



 if (node->parent->color != RB_TREE_NODE_BLACK) {
  rb_tree_insert_case3(tree, node);
 }
}




static void rb_tree_insert_case3(RBTree *tree, RBTreeNode *node)
{
 RBTreeNode *grandparent;
 RBTreeNode *uncle;




 grandparent = node->parent->parent;
 uncle = rb_tree_node_uncle(node);

 if (uncle != NULL && uncle->color == RB_TREE_NODE_RED) {

  node->parent->color = RB_TREE_NODE_BLACK;
  uncle->color = RB_TREE_NODE_BLACK;
  grandparent->color = RB_TREE_NODE_RED;



  rb_tree_insert_case1(tree, grandparent);

 } else {
  rb_tree_insert_case4(tree, node);
 }
}
# 266 ".tmp/tmp_files/src/rb-tree.c"
void rb_tree_insert_case4(RBTree *tree, RBTreeNode *node)
{
 RBTreeNode *next_node;
 RBTreeNodeSide side;







 side = rb_tree_node_side(node);

 if (side != rb_tree_node_side(node->parent)) {




  next_node = node->parent;




  rb_tree_rotate(tree, node->parent, 1-side);
 } else {
  next_node = node;
 }

 rb_tree_insert_case5(tree, next_node);
}
# 310 ".tmp/tmp_files/src/rb-tree.c"
void rb_tree_insert_case5(RBTree *tree, RBTreeNode *node)
{
 RBTreeNode *parent;
 RBTreeNode *grandparent;
 RBTreeNodeSide side;

 parent = node->parent;
 grandparent = parent->parent;




 side = rb_tree_node_side(node);



 rb_tree_rotate(tree, grandparent, 1-side);



 parent->color = RB_TREE_NODE_BLACK;
 grandparent->color = RB_TREE_NODE_RED;
}

RBTreeNode *rb_tree_insert(RBTree *tree, RBTreeKey key, RBTreeValue value)
{
 RBTreeNode *node;
 RBTreeNode **rover;
 RBTreeNode *parent;
 RBTreeNodeSide side;



 node = malloc(sizeof(RBTreeNode));

 if (node == NULL) {
  return NULL;
 }



 node->key = key;
 node->value = value;
 node->color = RB_TREE_NODE_RED;
 node->children[RB_TREE_NODE_LEFT] = NULL;
 node->children[RB_TREE_NODE_RIGHT] = NULL;



 parent = NULL;
 rover = &tree->root_node;

 while (*rover != NULL) {



  parent = *rover;



  if (tree->compare_func(value, (*rover)->value) < 0) {
   side = RB_TREE_NODE_LEFT;
  } else {
   side = RB_TREE_NODE_RIGHT;
  }

  rover = &(*rover)->children[side];
 }



 *rover = node;
 node->parent = parent;



 rb_tree_insert_case1(tree, node);



 ++tree->num_nodes;

 return node;
}

RBTreeNode *rb_tree_lookup_node(RBTree *tree, RBTreeKey key)
{
 RBTreeNode *node;
 RBTreeNodeSide side;
 int diff;

 node = tree->root_node;



 while (node != NULL) {
  diff = tree->compare_func(key, node->key);

  if (diff == 0) {
   return node;
  } else if (diff < 0) {
   side = RB_TREE_NODE_LEFT;
  } else {
   side = RB_TREE_NODE_RIGHT;
  }

  node = node->children[side];
 }



 return NULL;
}

RBTreeValue rb_tree_lookup(RBTree *tree, RBTreeKey key)
{
 RBTreeNode *node;



 node = rb_tree_lookup_node(tree, key);

 if (node == NULL) {
  return RB_TREE_NULL;
 } else {
  return node->value;
 }
}

void rb_tree_remove_node(RBTree *tree, RBTreeNode *node)
{

}

int rb_tree_remove(RBTree *tree, RBTreeKey key)
{
 RBTreeNode *node;



 node = rb_tree_lookup_node(tree, key);

 if (node == NULL) {
  return 0;
 }

 rb_tree_remove_node(tree, node);

 return 1;
}

RBTreeNode *rb_tree_root_node(RBTree *tree)
{
 return tree->root_node;
}

RBTreeKey rb_tree_node_key(RBTreeNode *node)
{
 return node->key;
}

RBTreeValue rb_tree_node_value(RBTreeNode *node)
{
 return node->value;
}

RBTreeNode *rb_tree_node_child(RBTreeNode *node, RBTreeNodeSide side)
{
 if (side == RB_TREE_NODE_LEFT || side == RB_TREE_NODE_RIGHT) {
  return node->children[side];
 } else {
  return NULL;
 }
}

RBTreeNode *rb_tree_node_parent(RBTreeNode *node)
{
 return node->parent;
}

RBTreeValue *rb_tree_to_array(RBTree *tree)
{

 return NULL;
}

int rb_tree_num_entries(RBTree *tree)
{
 return tree->num_nodes;
}
