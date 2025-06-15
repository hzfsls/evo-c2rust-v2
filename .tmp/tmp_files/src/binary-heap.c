# 1 ".tmp/tmp_files/src/binary-heap.c"
# 21 ".tmp/tmp_files/src/binary-heap.c"
#include <stdlib.h>

#include "binary-heap.h"



#ifdef ALLOC_TESTING
#include "alloc-testing.h"
#endif

struct _BinaryHeap {
 BinaryHeapType heap_type;
 BinaryHeapValue *values;
 unsigned int num_values;
 unsigned int alloced_size;
 BinaryHeapCompareFunc compare_func;
};

static int binary_heap_cmp(BinaryHeap *heap, BinaryHeapValue data1,
                           BinaryHeapValue data2)
{
 if (heap->heap_type == BINARY_HEAP_TYPE_MIN) {
  return heap->compare_func(data1, data2);
 } else {
  return -heap->compare_func(data1, data2);
 }
}

BinaryHeap *binary_heap_new(BinaryHeapType heap_type,
                            BinaryHeapCompareFunc compare_func)
{
 BinaryHeap *heap;

 heap = malloc(sizeof(BinaryHeap));

 if (heap == NULL) {
  return NULL;
 }

 heap->heap_type = heap_type;
 heap->num_values = 0;
 heap->compare_func = compare_func;



 heap->alloced_size = 16;
 heap->values = malloc(sizeof(BinaryHeapValue) * heap->alloced_size);

 if (heap->values == NULL) {
  free(heap);
  return NULL;
 }

 return heap;
}

void binary_heap_free(BinaryHeap *heap)
{
 free(heap->values);
 free(heap);
}

int binary_heap_insert(BinaryHeap *heap, BinaryHeapValue value)
{
 BinaryHeapValue *new_values;
 unsigned int index;
 unsigned int new_size;
 unsigned int parent;



 if (heap->num_values >= heap->alloced_size) {



  new_size = heap->alloced_size * 2;
  new_values = realloc(heap->values,
                       sizeof(BinaryHeapValue) * new_size);

  if (new_values == NULL) {
   return 0;
  }

  heap->alloced_size = new_size;
  heap->values = new_values;
 }



 index = heap->num_values;
 ++heap->num_values;



 while (index > 0) {



  parent = (index - 1) / 2;



  if (binary_heap_cmp(heap, heap->values[parent], value) < 0) {



   break;

  } else {



   heap->values[index] = heap->values[parent];



   index = parent;
  }
 }



 heap->values[index] = value;

 return 1;
}

BinaryHeapValue binary_heap_pop(BinaryHeap *heap)
{
 BinaryHeapValue result;
 BinaryHeapValue new_value;
 unsigned int index;
 unsigned int next_index;
 unsigned int child1, child2;



 if (heap->num_values == 0) {
  return BINARY_HEAP_NULL;
 }



 result = heap->values[0];




 new_value = heap->values[heap->num_values - 1];
 --heap->num_values;



 index = 0;

 for (;;) {



  child1 = index * 2 + 1;
  child2 = index * 2 + 2;

  if (child1 < heap->num_values
   && binary_heap_cmp(heap,
                      new_value,
                      heap->values[child1]) > 0) {




   if (child2 < heap->num_values
    && binary_heap_cmp(heap,
                       heap->values[child1],
                       heap->values[child2]) > 0) {
    next_index = child2;
   } else {
    next_index = child1;
   }

  } else if (child2 < heap->num_values
          && binary_heap_cmp(heap,
                             new_value,
                             heap->values[child2]) > 0) {




   next_index = child2;

  } else {




   heap->values[index] = new_value;
   break;
  }



  heap->values[index] = heap->values[next_index];



  index = next_index;
 }

 return result;
}

unsigned int binary_heap_num_entries(BinaryHeap *heap)
{
 return heap->num_values;
}
