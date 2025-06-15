# 1 ".tmp/tmp_files/src/slist.h"
# 67 ".tmp/tmp_files/src/slist.h"
#ifndef ALGORITHM_SLIST_H
#define ALGORITHM_SLIST_H 

#ifdef __cplusplus
extern "C" {
#endif
# 81 ".tmp/tmp_files/src/slist.h"
typedef struct _SListEntry SListEntry;





typedef struct _SListIterator SListIterator;





typedef void *SListValue;





struct _SListIterator {
 SListEntry **prev_next;
 SListEntry *current;
};





#define SLIST_NULL ((void *) 0)
# 118 ".tmp/tmp_files/src/slist.h"
typedef int (*SListCompareFunc)(SListValue value1, SListValue value2);
# 128 ".tmp/tmp_files/src/slist.h"
typedef int (*SListEqualFunc)(SListValue value1, SListValue value2);







void slist_free(SListEntry *list);
# 147 ".tmp/tmp_files/src/slist.h"
SListEntry *slist_prepend(SListEntry **list, SListValue data);
# 158 ".tmp/tmp_files/src/slist.h"
SListEntry *slist_append(SListEntry **list, SListValue data);
# 167 ".tmp/tmp_files/src/slist.h"
SListEntry *slist_next(SListEntry *listentry);
# 176 ".tmp/tmp_files/src/slist.h"
SListValue slist_data(SListEntry *listentry);
# 185 ".tmp/tmp_files/src/slist.h"
void slist_set_data(SListEntry *listentry, SListValue value);
# 195 ".tmp/tmp_files/src/slist.h"
SListEntry *slist_nth_entry(SListEntry *list, unsigned int n);
# 206 ".tmp/tmp_files/src/slist.h"
SListValue slist_nth_data(SListEntry *list, unsigned int n);
# 215 ".tmp/tmp_files/src/slist.h"
unsigned int slist_length(SListEntry *list);
# 227 ".tmp/tmp_files/src/slist.h"
SListValue *slist_to_array(SListEntry *list);
# 238 ".tmp/tmp_files/src/slist.h"
int slist_remove_entry(SListEntry **list, SListEntry *entry);
# 250 ".tmp/tmp_files/src/slist.h"
unsigned int slist_remove_data(SListEntry **list,
                               SListEqualFunc callback,
                               SListValue data);
# 261 ".tmp/tmp_files/src/slist.h"
void slist_sort(SListEntry **list, SListCompareFunc compare_func);
# 275 ".tmp/tmp_files/src/slist.h"
SListEntry *slist_find_data(SListEntry *list,
                            SListEqualFunc callback,
                            SListValue data);
# 287 ".tmp/tmp_files/src/slist.h"
void slist_iterate(SListEntry **list, SListIterator *iter);
# 298 ".tmp/tmp_files/src/slist.h"
int slist_iter_has_more(SListIterator *iterator);
# 308 ".tmp/tmp_files/src/slist.h"
SListValue slist_iter_next(SListIterator *iterator);
# 317 ".tmp/tmp_files/src/slist.h"
void slist_iter_remove(SListIterator *iterator);

#ifdef __cplusplus
}
#endif

#endif
