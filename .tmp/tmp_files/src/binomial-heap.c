# 1 ".tmp/tmp_files/src/binomial-heap.c"
# 21 ".tmp/tmp_files/src/binomial-heap.c"
#include <stdlib.h>
#include <string.h>
#include <limits.h>

#include "binomial-heap.h"



#ifdef ALLOC_TESTING
#include "alloc-testing.h"
#endif

typedef struct _BinomialTree BinomialTree;

struct _BinomialTree
{
 BinomialHeapValue value;
 unsigned short order;
 unsigned short refcount;
 BinomialTree **subtrees;
};

struct _BinomialHeap
{
 BinomialHeapType heap_type;
 BinomialHeapCompareFunc compare_func;
 unsigned int num_values;
 BinomialTree **roots;
 unsigned int roots_length;
};

static int binomial_heap_cmp(BinomialHeap *heap,
                             BinomialHeapValue data1,
                             BinomialHeapValue data2)
{
 if (heap->heap_type == BINOMIAL_HEAP_TYPE_MIN) {
  return heap->compare_func(data1, data2);
 } else {
  return -(heap->compare_func(data1, data2));
 }
}

static void binomial_tree_ref(BinomialTree *tree)
{
 if (tree != NULL) {
  ++tree->refcount;
 }
}

static void binomial_tree_unref(BinomialTree *tree)
{
 int i;

 if (tree == NULL) {
  return;
 }



 --tree->refcount;




 if (tree->refcount == 0) {

  for (i=0; i<tree->order; ++i) {
   binomial_tree_unref(tree->subtrees[i]);
  }

  free(tree->subtrees);
  free(tree);
 }
}

static BinomialTree *binomial_tree_merge(BinomialHeap *heap,
                                         BinomialTree *tree1,
                                         BinomialTree *tree2)
{
 BinomialTree *new_tree;
 BinomialTree *tmp;
 int i;




 if (binomial_heap_cmp(heap, tree1->value, tree2->value) > 0) {



  tmp = tree1;
  tree1 = tree2;
  tree2 = tmp;
 }



 new_tree = malloc(sizeof(BinomialTree));

 if (new_tree == NULL) {
  return NULL;
 }

 new_tree->refcount = 0;
 new_tree->order = (unsigned short) (tree1->order + 1);



 new_tree->value = tree1->value;




 new_tree->subtrees = malloc(sizeof(BinomialTree *) * new_tree->order);

 if (new_tree->subtrees == NULL) {
  free(new_tree);
  return NULL;
 }

 memcpy(new_tree->subtrees, tree1->subtrees,
        sizeof(BinomialTree *) * tree1->order);
 new_tree->subtrees[new_tree->order - 1] = tree2;



 for (i=0; i<new_tree->order; ++i) {
  binomial_tree_ref(new_tree->subtrees[i]);
 }

 return new_tree;
}





static void binomial_heap_merge_undo(BinomialTree **new_roots,
                                     unsigned int count)
{
 unsigned int i;

 for (i=0; i<=count; ++i) {
  binomial_tree_unref(new_roots[i]);
 }

 free(new_roots);
}




static int binomial_heap_merge(BinomialHeap *heap, BinomialHeap *other)
{
 BinomialTree **new_roots;
 unsigned int new_roots_length;
 BinomialTree *vals[3];
 int num_vals;
 BinomialTree *carry;
 BinomialTree *new_carry;
 unsigned int max;
 unsigned int i;




 if (heap->roots_length > other->roots_length) {
  max = heap->roots_length + 1;
 } else {
  max = other->roots_length + 1;
 }



 new_roots = malloc(sizeof(BinomialTree *) * max);

 if (new_roots == NULL) {
  return 0;
 }




 new_roots_length = 0;
 carry = NULL;

 for (i=0; i<max; ++i) {




  num_vals = 0;



  if (i < heap->roots_length && heap->roots[i] != NULL) {
   vals[num_vals] = heap->roots[i];
   ++num_vals;
  }



  if (i < other->roots_length && other->roots[i] != NULL) {
   vals[num_vals] = other->roots[i];
   ++num_vals;
  }




  if (carry != NULL) {
   vals[num_vals] = carry;
   ++num_vals;
  }



  if ((num_vals & 1) != 0) {



   new_roots[i] = vals[num_vals - 1];
   binomial_tree_ref(new_roots[i]);
   new_roots_length = i + 1;

  } else {



   new_roots[i] = NULL;
  }




  if ((num_vals & 2) != 0) {




   new_carry = binomial_tree_merge(heap,
                                   vals[0],
                                   vals[1]);

   if (new_carry == NULL) {





    binomial_heap_merge_undo(new_roots, i);



    binomial_tree_unref(carry);

    return 0;
   }

  } else {



   new_carry = NULL;
  }



  binomial_tree_unref(carry);



  carry = new_carry;

  binomial_tree_ref(carry);
 }




 for (i=0; i<heap->roots_length; ++i) {
  if (heap->roots[i] != NULL) {
   binomial_tree_unref(heap->roots[i]);
  }
 }



 free(heap->roots);
 heap->roots = new_roots;
 heap->roots_length = new_roots_length;



 return 1;
}

BinomialHeap *binomial_heap_new(BinomialHeapType heap_type,
                                BinomialHeapCompareFunc compare_func)
{
 BinomialHeap *new_heap;



 new_heap = calloc(1, sizeof(BinomialHeap));

 if (new_heap == NULL) {
  return NULL;
 }



 new_heap->heap_type = heap_type;
 new_heap->compare_func = compare_func;

 return new_heap;
}

void binomial_heap_free(BinomialHeap *heap)
{
 unsigned int i;




 for (i=0; i<heap->roots_length; ++i) {
  binomial_tree_unref(heap->roots[i]);
 }



 free(heap->roots);
 free(heap);
}

int binomial_heap_insert(BinomialHeap *heap, BinomialHeapValue value)
{
 BinomialHeap fake_heap;
 BinomialTree *new_tree;
 int result;



 new_tree = malloc(sizeof(BinomialTree));

 if (new_tree == NULL) {
  return 0;
 }





 new_tree->value = value;
 new_tree->order = 0;
 new_tree->refcount = 1;
 new_tree->subtrees = NULL;



 fake_heap.heap_type = heap->heap_type;
 fake_heap.compare_func = heap->compare_func;
 fake_heap.num_values = 1;
 fake_heap.roots = &new_tree;
 fake_heap.roots_length = 1;



 result = binomial_heap_merge(heap, &fake_heap);

 if (result != 0) {
  ++heap->num_values;
 }



 binomial_tree_unref(new_tree);

 return result;
}

BinomialHeapValue binomial_heap_pop(BinomialHeap *heap)
{
 BinomialTree *least_tree;
 BinomialHeap fake_heap;
 BinomialHeapValue result;
 unsigned int i;
 unsigned int least_index;

 if (heap->num_values == 0) {
  return BINOMIAL_HEAP_NULL;
 }



 least_index = UINT_MAX;

 for (i=0; i<heap->roots_length; ++i) {

  if (heap->roots[i] == NULL) {
   continue;
  }

  if (least_index == UINT_MAX
   || binomial_heap_cmp(heap,
                        heap->roots[i]->value,
                        heap->roots[least_index]->value) < 0) {
   least_index = i;
  }
 }



 least_tree = heap->roots[least_index];
 heap->roots[least_index] = NULL;



 fake_heap.heap_type = heap->heap_type;
 fake_heap.compare_func = heap->compare_func;
 fake_heap.roots = least_tree->subtrees;
 fake_heap.roots_length = least_tree->order;



 if (binomial_heap_merge(heap, &fake_heap)) {





  result = least_tree->value;
  binomial_tree_unref(least_tree);



  --heap->num_values;

  return result;

 } else {



  heap->roots[least_index] = least_tree;



  return BINOMIAL_HEAP_NULL;
 }
}

unsigned int binomial_heap_num_entries(BinomialHeap *heap)
{
 return heap->num_values;
}
