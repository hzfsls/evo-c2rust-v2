# 1 ".tmp/tmp_files/src/set.h"
# 47 ".tmp/tmp_files/src/set.h"
#ifndef ALGORITHM_SET_H
#define ALGORITHM_SET_H 

#ifdef __cplusplus
extern "C" {
#endif






typedef struct _Set Set;







typedef struct _SetIterator SetIterator;





typedef struct _SetEntry SetEntry;





typedef void *SetValue;





struct _SetIterator {
 Set *set;
 SetEntry *next_entry;
 unsigned int next_chain;
};





#define SET_NULL ((void *) 0)





typedef unsigned int (*SetHashFunc)(SetValue value);






typedef int (*SetEqualFunc)(SetValue value1, SetValue value2);






typedef void (*SetFreeFunc)(SetValue value);
# 127 ".tmp/tmp_files/src/set.h"
Set *set_new(SetHashFunc hash_func, SetEqualFunc equal_func);







void set_free(Set *set);
# 146 ".tmp/tmp_files/src/set.h"
void set_register_free_function(Set *set, SetFreeFunc free_func);
# 159 ".tmp/tmp_files/src/set.h"
int set_insert(Set *set, SetValue data);
# 171 ".tmp/tmp_files/src/set.h"
int set_remove(Set *set, SetValue data);
# 182 ".tmp/tmp_files/src/set.h"
int set_query(Set *set, SetValue data);
# 191 ".tmp/tmp_files/src/set.h"
unsigned int set_num_entries(Set *set);
# 202 ".tmp/tmp_files/src/set.h"
SetValue *set_to_array(Set *set);
# 214 ".tmp/tmp_files/src/set.h"
Set *set_union(Set *set1, Set *set2);
# 226 ".tmp/tmp_files/src/set.h"
Set *set_intersection(Set *set1, Set *set2);
# 236 ".tmp/tmp_files/src/set.h"
void set_iterate(Set *set, SetIterator *iter);
# 247 ".tmp/tmp_files/src/set.h"
int set_iter_has_more(SetIterator *iterator);
# 257 ".tmp/tmp_files/src/set.h"
SetValue set_iter_next(SetIterator *iterator);

#ifdef __cplusplus
}
#endif

#endif
