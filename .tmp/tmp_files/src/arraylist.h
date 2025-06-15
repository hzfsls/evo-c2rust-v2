# 1 ".tmp/tmp_files/src/arraylist.h"
# 39 ".tmp/tmp_files/src/arraylist.h"
#ifndef ALGORITHM_ARRAYLIST_H
#define ALGORITHM_ARRAYLIST_H 

#ifdef __cplusplus
extern "C" {
#endif





typedef void *ArrayListValue;
# 59 ".tmp/tmp_files/src/arraylist.h"
typedef struct _ArrayList ArrayList;





struct _ArrayList {



 ArrayListValue *data;



 unsigned int length;



 unsigned int _alloced;
};







typedef int (*ArrayListEqualFunc)(ArrayListValue value1,
                                  ArrayListValue value2);
# 101 ".tmp/tmp_files/src/arraylist.h"
typedef int (*ArrayListCompareFunc)(ArrayListValue value1,
                                    ArrayListValue value2);
# 116 ".tmp/tmp_files/src/arraylist.h"
ArrayList *arraylist_new(unsigned int length);







void arraylist_free(ArrayList *arraylist);
# 136 ".tmp/tmp_files/src/arraylist.h"
int arraylist_append(ArrayList *arraylist, ArrayListValue data);
# 148 ".tmp/tmp_files/src/arraylist.h"
int arraylist_prepend(ArrayList *arraylist, ArrayListValue data);
# 157 ".tmp/tmp_files/src/arraylist.h"
void arraylist_remove(ArrayList *arraylist, unsigned int index);
# 167 ".tmp/tmp_files/src/arraylist.h"
void arraylist_remove_range(ArrayList *arraylist, unsigned int index,
                            unsigned int length);
# 183 ".tmp/tmp_files/src/arraylist.h"
int arraylist_insert(ArrayList *arraylist, unsigned int index,
                     ArrayListValue data);
# 197 ".tmp/tmp_files/src/arraylist.h"
int arraylist_index_of(ArrayList *arraylist,
                       ArrayListEqualFunc callback,
                       ArrayListValue data);







void arraylist_clear(ArrayList *arraylist);
# 216 ".tmp/tmp_files/src/arraylist.h"
void arraylist_sort(ArrayList *arraylist, ArrayListCompareFunc compare_func);

#ifdef __cplusplus
}
#endif

#endif
