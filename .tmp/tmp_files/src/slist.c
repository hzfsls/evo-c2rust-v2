# 1 ".tmp/tmp_files/src/slist.c"
# 21 ".tmp/tmp_files/src/slist.c"
#include <stdlib.h>

#include "slist.h"



#ifdef ALLOC_TESTING
#include "alloc-testing.h"
#endif



struct _SListEntry {
 SListValue data;
 SListEntry *next;
};

void slist_free(SListEntry *list)
{
 SListEntry *entry;




 entry = list;

 while (entry != NULL) {
  SListEntry *next;

  next = entry->next;

  free(entry);

  entry = next;
 }
}

SListEntry *slist_prepend(SListEntry **list, SListValue data)
{
 SListEntry *newentry;



 newentry = malloc(sizeof(SListEntry));

 if (newentry == NULL) {
  return NULL;
 }

 newentry->data = data;



 newentry->next = *list;
 *list = newentry;

 return newentry;
}

SListEntry *slist_append(SListEntry **list, SListValue data)
{
 SListEntry *rover;
 SListEntry *newentry;



 newentry = malloc(sizeof(SListEntry));

 if (newentry == NULL) {
  return NULL;
 }

 newentry->data = data;
 newentry->next = NULL;



 if (*list == NULL) {



  *list = newentry;

 } else {



  for (rover=*list; rover->next != NULL; rover = rover->next);



  rover->next = newentry;
 }

 return newentry;
}

SListValue slist_data(SListEntry *listentry)
{
 return listentry->data;
}

void slist_set_data(SListEntry *listentry, SListValue data)
{
 if (listentry != NULL) {
  listentry->data = data;
 }
}

SListEntry *slist_next(SListEntry *listentry)
{
 return listentry->next;
}

SListEntry *slist_nth_entry(SListEntry *list, unsigned int n)
{
 SListEntry *entry;
 unsigned int i;




 entry = list;

 for (i=0; i<n; ++i) {

  if (entry == NULL) {
   return NULL;
  }
  entry = entry->next;
 }

 return entry;
}

SListValue slist_nth_data(SListEntry *list, unsigned int n)
{
 SListEntry *entry;



 entry = slist_nth_entry(list, n);



 if (entry == NULL) {
  return SLIST_NULL;
 } else {
  return entry->data;
 }
}

unsigned int slist_length(SListEntry *list)
{
 SListEntry *entry;
 unsigned int length;

 length = 0;
 entry = list;

 while (entry != NULL) {



  ++length;

  entry = entry->next;
 }

 return length;
}

SListValue *slist_to_array(SListEntry *list)
{
 SListEntry *rover;
 SListValue *array;
 unsigned int length;
 unsigned int i;



 length = slist_length(list);

 array = malloc(sizeof(SListValue) * length);

 if (array == NULL) {
  return NULL;
 }



 rover = list;

 for (i=0; i<length; ++i) {



  array[i] = rover->data;



  rover = rover->next;
 }

 return array;
}

int slist_remove_entry(SListEntry **list, SListEntry *entry)
{
 SListEntry *rover;



 if (*list == NULL || entry == NULL) {
  return 0;
 }



 if (*list == entry) {



  *list = entry->next;

 } else {



  rover = *list;

  while (rover != NULL && rover->next != entry) {
   rover = rover->next;
  }

  if (rover == NULL) {



   return 0;

  } else {




   rover->next = entry->next;
  }
 }



 free(entry);



 return 1;
}

unsigned int slist_remove_data(SListEntry **list, SListEqualFunc callback,
                               SListValue data)
{
 SListEntry **rover;
 SListEntry *next;
 unsigned int entries_removed;

 entries_removed = 0;





 rover = list;

 while (*rover != NULL) {



  if (callback((*rover)->data, data) != 0) {



   next = (*rover)->next;
   free(*rover);
   *rover = next;



   ++entries_removed;
  } else {



   rover = &((*rover)->next);
  }
 }

 return entries_removed;
}




static SListEntry *slist_sort_internal(SListEntry **list,
                                       SListCompareFunc compare_func)
{
 SListEntry *pivot;
 SListEntry *rover;
 SListEntry *less_list, *more_list;
 SListEntry *less_list_end, *more_list_end;




 if (*list == NULL || (*list)->next == NULL) {
  return *list;
 }



 pivot = *list;





 less_list = NULL;
 more_list = NULL;
 rover = (*list)->next;

 while (rover != NULL) {
  SListEntry *next = rover->next;

  if (compare_func(rover->data, pivot->data) < 0) {



   rover->next = less_list;
   less_list = rover;

  } else {



   rover->next = more_list;
   more_list = rover;

  }

  rover = next;
 }



 less_list_end = slist_sort_internal(&less_list, compare_func);
 more_list_end = slist_sort_internal(&more_list, compare_func);



 *list = less_list;




 if (less_list == NULL) {
  *list = pivot;
 } else {
  less_list_end->next = pivot;
 }



 pivot->next = more_list;





 if (more_list == NULL) {
  return pivot;
 } else {
  return more_list_end;
 }
}

void slist_sort(SListEntry **list, SListCompareFunc compare_func)
{
 slist_sort_internal(list, compare_func);
}

SListEntry *slist_find_data(SListEntry *list,
                            SListEqualFunc callback,
                            SListValue data)
{
 SListEntry *rover;



 for (rover=list; rover != NULL; rover=rover->next) {
  if (callback(rover->data, data) != 0) {
   return rover;
  }
 }



 return NULL;
}

void slist_iterate(SListEntry **list, SListIterator *iter)
{


 iter->prev_next = list;



 iter->current = NULL;
}

int slist_iter_has_more(SListIterator *iter)
{
 if (iter->current == NULL || iter->current != *iter->prev_next) {






  return *iter->prev_next != NULL;

 } else {




  return iter->current->next != NULL;
 }
}

SListValue slist_iter_next(SListIterator *iter)
{
 if (iter->current == NULL || iter->current != *iter->prev_next) {





  iter->current = *iter->prev_next;

 } else {




  iter->prev_next = &iter->current->next;
  iter->current = iter->current->next;
 }



 if (iter->current == NULL) {
  return SLIST_NULL;
 } else {
  return iter->current->data;
 }
}

void slist_iter_remove(SListIterator *iter)
{
 if (iter->current == NULL || iter->current != *iter->prev_next) {





 } else {



  *iter->prev_next = iter->current->next;
  free(iter->current);
  iter->current = NULL;
 }
}
