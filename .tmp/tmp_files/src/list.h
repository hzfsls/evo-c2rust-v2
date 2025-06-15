# 1 ".tmp/tmp_files/src/list.h"
# 55 ".tmp/tmp_files/src/list.h"
#ifndef ALGORITHM_LIST_H
#define ALGORITHM_LIST_H 

#ifdef __cplusplus
extern "C" {
#endif
# 69 ".tmp/tmp_files/src/list.h"
typedef struct _ListEntry ListEntry;





typedef struct _ListIterator ListIterator;





typedef void *ListValue;





struct _ListIterator {
 ListEntry **prev_next;
 ListEntry *current;
};





#define LIST_NULL ((void *) 0)
# 108 ".tmp/tmp_files/src/list.h"
typedef int (*ListCompareFunc)(ListValue value1, ListValue value2);
# 120 ".tmp/tmp_files/src/list.h"
typedef int (*ListEqualFunc)(ListValue value1, ListValue value2);







void list_free(ListEntry *list);
# 139 ".tmp/tmp_files/src/list.h"
ListEntry *list_prepend(ListEntry **list, ListValue data);
# 150 ".tmp/tmp_files/src/list.h"
ListEntry *list_append(ListEntry **list, ListValue data);
# 160 ".tmp/tmp_files/src/list.h"
ListEntry *list_prev(ListEntry *listentry);
# 170 ".tmp/tmp_files/src/list.h"
ListEntry *list_next(ListEntry *listentry);
# 179 ".tmp/tmp_files/src/list.h"
ListValue list_data(ListEntry *listentry);
# 188 ".tmp/tmp_files/src/list.h"
void list_set_data(ListEntry *listentry, ListValue value);
# 198 ".tmp/tmp_files/src/list.h"
ListEntry *list_nth_entry(ListEntry *list, unsigned int n);
# 209 ".tmp/tmp_files/src/list.h"
ListValue list_nth_data(ListEntry *list, unsigned int n);
# 218 ".tmp/tmp_files/src/list.h"
unsigned int list_length(ListEntry *list);
# 230 ".tmp/tmp_files/src/list.h"
ListValue *list_to_array(ListEntry *list);
# 241 ".tmp/tmp_files/src/list.h"
int list_remove_entry(ListEntry **list, ListEntry *entry);
# 253 ".tmp/tmp_files/src/list.h"
unsigned int list_remove_data(ListEntry **list, ListEqualFunc callback,
                              ListValue data);
# 263 ".tmp/tmp_files/src/list.h"
void list_sort(ListEntry **list, ListCompareFunc compare_func);
# 276 ".tmp/tmp_files/src/list.h"
ListEntry *list_find_data(ListEntry *list,
                          ListEqualFunc callback,
                          ListValue data);
# 287 ".tmp/tmp_files/src/list.h"
void list_iterate(ListEntry **list, ListIterator *iter);
# 298 ".tmp/tmp_files/src/list.h"
int list_iter_has_more(ListIterator *iterator);
# 308 ".tmp/tmp_files/src/list.h"
ListValue list_iter_next(ListIterator *iterator);
# 317 ".tmp/tmp_files/src/list.h"
void list_iter_remove(ListIterator *iterator);

#ifdef __cplusplus
}
#endif

#endif
