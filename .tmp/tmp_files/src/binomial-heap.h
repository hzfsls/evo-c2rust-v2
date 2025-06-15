# 1 ".tmp/tmp_files/src/binomial-heap.h"
# 38 ".tmp/tmp_files/src/binomial-heap.h"
#ifndef ALGORITHM_BINOMIAL_HEAP_H
#define ALGORITHM_BINOMIAL_HEAP_H 

#ifdef __cplusplus
extern "C" {
#endif
# 53 ".tmp/tmp_files/src/binomial-heap.h"
typedef enum {


 BINOMIAL_HEAP_TYPE_MIN,



 BINOMIAL_HEAP_TYPE_MAX
} BinomialHeapType;





typedef void *BinomialHeapValue;





#define BINOMIAL_HEAP_NULL ((void *) 0)
# 85 ".tmp/tmp_files/src/binomial-heap.h"
typedef int (*BinomialHeapCompareFunc)(BinomialHeapValue value1,
                                       BinomialHeapValue value2);





typedef struct _BinomialHeap BinomialHeap;
# 104 ".tmp/tmp_files/src/binomial-heap.h"
BinomialHeap *binomial_heap_new(BinomialHeapType heap_type,
                                BinomialHeapCompareFunc compare_func);







void binomial_heap_free(BinomialHeap *heap);
# 125 ".tmp/tmp_files/src/binomial-heap.h"
int binomial_heap_insert(BinomialHeap *heap, BinomialHeapValue value);
# 135 ".tmp/tmp_files/src/binomial-heap.h"
BinomialHeapValue binomial_heap_pop(BinomialHeap *heap);
# 144 ".tmp/tmp_files/src/binomial-heap.h"
unsigned int binomial_heap_num_entries(BinomialHeap *heap);

#ifdef __cplusplus
}
#endif

#endif
