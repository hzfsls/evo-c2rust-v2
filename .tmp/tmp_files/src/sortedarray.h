# 1 ".tmp/tmp_files/src/sortedarray.h"
# 42 ".tmp/tmp_files/src/sortedarray.h"
#ifndef ALGORITHM_SORTEDARRAY_H
#define ALGORITHM_SORTEDARRAY_H 

#ifdef __cplusplus
extern "C" {
#endif




typedef void *SortedArrayValue;
# 65 ".tmp/tmp_files/src/sortedarray.h"
typedef struct _SortedArray SortedArray;
# 76 ".tmp/tmp_files/src/sortedarray.h"
typedef int (*SortedArrayEqualFunc)(SortedArrayValue value1,
                                    SortedArrayValue value2);
# 88 ".tmp/tmp_files/src/sortedarray.h"
typedef int (*SortedArrayCompareFunc)(SortedArrayValue value1,
                                      SortedArrayValue value2);
# 98 ".tmp/tmp_files/src/sortedarray.h"
SortedArrayValue *sortedarray_get(SortedArray *array, unsigned int i);







unsigned int sortedarray_length(SortedArray *array);
# 121 ".tmp/tmp_files/src/sortedarray.h"
SortedArray *sortedarray_new(unsigned int length,
                             SortedArrayEqualFunc equ_func,
                             SortedArrayCompareFunc cmp_func);






void sortedarray_free(SortedArray *sortedarray);
# 139 ".tmp/tmp_files/src/sortedarray.h"
void sortedarray_remove(SortedArray *sortedarray, unsigned int index);
# 149 ".tmp/tmp_files/src/sortedarray.h"
void sortedarray_remove_range(SortedArray *sortedarray, unsigned int index,
                              unsigned int length);
# 160 ".tmp/tmp_files/src/sortedarray.h"
int sortedarray_insert(SortedArray *sortedarray, SortedArrayValue data);
# 169 ".tmp/tmp_files/src/sortedarray.h"
int sortedarray_index_of(SortedArray *sortedarray, SortedArrayValue data);






void sortedarray_clear(SortedArray *sortedarray);

#ifdef __cplusplus
}
#endif

#endif
