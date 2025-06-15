# 1 ".tmp/tmp_files/src/binary-heap.h"
# 38 ".tmp/tmp_files/src/binary-heap.h"
#ifndef ALGORITHM_BINARY_HEAP_H
#define ALGORITHM_BINARY_HEAP_H 

#ifdef __cplusplus
extern "C" {
#endif
# 53 ".tmp/tmp_files/src/binary-heap.h"
typedef enum {


 BINARY_HEAP_TYPE_MIN,



 BINARY_HEAP_TYPE_MAX
} BinaryHeapType;





typedef void *BinaryHeapValue;





#define BINARY_HEAP_NULL ((void *) 0)
# 85 ".tmp/tmp_files/src/binary-heap.h"
typedef int (*BinaryHeapCompareFunc)(BinaryHeapValue value1,
                                     BinaryHeapValue value2);





typedef struct _BinaryHeap BinaryHeap;
# 104 ".tmp/tmp_files/src/binary-heap.h"
BinaryHeap *binary_heap_new(BinaryHeapType heap_type,
                            BinaryHeapCompareFunc compare_func);







void binary_heap_free(BinaryHeap *heap);
# 125 ".tmp/tmp_files/src/binary-heap.h"
int binary_heap_insert(BinaryHeap *heap, BinaryHeapValue value);
# 135 ".tmp/tmp_files/src/binary-heap.h"
BinaryHeapValue binary_heap_pop(BinaryHeap *heap);
# 144 ".tmp/tmp_files/src/binary-heap.h"
unsigned int binary_heap_num_entries(BinaryHeap *heap);

#ifdef __cplusplus
}
#endif

#endif
