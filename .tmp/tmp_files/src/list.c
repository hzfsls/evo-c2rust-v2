# 1 ".tmp/tmp_files/src/list.c"
# 21 ".tmp/tmp_files/src/list.c"
#include <stdlib.h>

#include "list.h"



#ifdef ALLOC_TESTING
#include "alloc-testing.h"
#endif



struct _ListEntry {
 ListValue data;
 ListEntry *prev;
 ListEntry *next;
};

void list_free(ListEntry *list)
{
 ListEntry *entry;




 entry = list;

 while (entry != NULL) {
  ListEntry *next;

  next = entry->next;

  free(entry);

  entry = next;
 }
}

ListEntry *list_prepend(ListEntry **list, ListValue data)
{
 ListEntry *newentry;

 if (list == NULL) {



  return NULL;
 }



 newentry = malloc(sizeof(ListEntry));

 if (newentry == NULL) {
  return NULL;
 }

 newentry->data = data;



 if (*list != NULL) {
  (*list)->prev = newentry;
 }
 newentry->prev = NULL;
 newentry->next = *list;
 *list = newentry;

 return newentry;
}

ListEntry *list_append(ListEntry **list, ListValue data)
{
 ListEntry *rover;
 ListEntry *newentry;

 if (list == NULL) {
  return NULL;
 }



 newentry = malloc(sizeof(ListEntry));

 if (newentry == NULL) {
  return NULL;
 }

 newentry->data = data;
 newentry->next = NULL;



 if (*list == NULL) {



  *list = newentry;
  newentry->prev = NULL;

 } else {



  for (rover=*list; rover->next != NULL; rover = rover->next);



  newentry->prev = rover;
  rover->next = newentry;
 }

 return newentry;
}

ListValue list_data(ListEntry *listentry)
{
 if (listentry == NULL) {
  return LIST_NULL;
 }

 return listentry->data;
}

void list_set_data(ListEntry *listentry, ListValue value)
{
 if (listentry != NULL) {
  listentry->data = value;
 }
}

ListEntry *list_prev(ListEntry *listentry)
{
 if (listentry == NULL) {
  return NULL;
 }

 return listentry->prev;
}

ListEntry *list_next(ListEntry *listentry)
{
 if (listentry == NULL) {
  return NULL;
 }

 return listentry->next;
}

ListEntry *list_nth_entry(ListEntry *list, unsigned int n)
{
 ListEntry *entry;
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

ListValue list_nth_data(ListEntry *list, unsigned int n)
{
 ListEntry *entry;



 entry = list_nth_entry(list, n);



 if (entry == NULL) {
  return LIST_NULL;
 } else {
  return entry->data;
 }
}

unsigned int list_length(ListEntry *list)
{
 ListEntry *entry;
 unsigned int length;

 length = 0;
 entry = list;

 while (entry != NULL) {



  ++length;

  entry = entry->next;
 }

 return length;
}

ListValue *list_to_array(ListEntry *list)
{
 ListEntry *rover;
 ListValue *array;
 unsigned int length;
 unsigned int i;



 length = list_length(list);

 array = malloc(sizeof(ListValue) * length);

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

int list_remove_entry(ListEntry **list, ListEntry *entry)
{


 if (list == NULL || *list == NULL || entry == NULL) {
  return 0;
 }



 if (entry->prev == NULL) {



  *list = entry->next;




  if (entry->next != NULL) {
   entry->next->prev = NULL;
  }

 } else {





  entry->prev->next = entry->next;




  if (entry->next != NULL) {
   entry->next->prev = entry->prev;
  }
 }



 free(entry);



 return 1;
}

unsigned int list_remove_data(ListEntry **list, ListEqualFunc callback,
                              ListValue data)
{
 unsigned int entries_removed;
 ListEntry *rover;
 ListEntry *next;

 if (list == NULL || callback == NULL) {
  return 0;
 }

 entries_removed = 0;



 rover = *list;

 while (rover != NULL) {

  next = rover->next;

  if (callback(rover->data, data)) {




   if (rover->prev == NULL) {



    *list = rover->next;
   } else {




    rover->prev->next = rover->next;
   }

   if (rover->next != NULL) {
    rover->next->prev = rover->prev;
   }



   free(rover);

   ++entries_removed;
  }



  rover = next;
 }

 return entries_removed;
}




static ListEntry *list_sort_internal(ListEntry **list,
                                     ListCompareFunc compare_func)
{
 ListEntry *pivot;
 ListEntry *rover;
 ListEntry *less_list, *more_list;
 ListEntry *less_list_end, *more_list_end;

 if (list == NULL || compare_func == NULL) {
  return NULL;
 }




 if (*list == NULL || (*list)->next == NULL) {
  return *list;
 }



 pivot = *list;





 less_list = NULL;
 more_list = NULL;
 rover = (*list)->next;

 while (rover != NULL) {
  ListEntry *next = rover->next;

  if (compare_func(rover->data, pivot->data) < 0) {



   rover->prev = NULL;
   rover->next = less_list;
   if (less_list != NULL) {
    less_list->prev = rover;
   }
   less_list = rover;

  } else {



   rover->prev = NULL;
   rover->next = more_list;
   if (more_list != NULL) {
    more_list->prev = rover;
   }
   more_list = rover;
  }

  rover = next;
 }



 less_list_end = list_sort_internal(&less_list, compare_func);
 more_list_end = list_sort_internal(&more_list, compare_func);



 *list = less_list;




 if (less_list == NULL) {
  pivot->prev = NULL;
  *list = pivot;
 } else {
  pivot->prev = less_list_end;
  less_list_end->next = pivot;
 }



 pivot->next = more_list;
 if (more_list != NULL) {
  more_list->prev = pivot;
 }





 if (more_list == NULL) {
  return pivot;
 } else {
  return more_list_end;
 }
}

void list_sort(ListEntry **list, ListCompareFunc compare_func)
{
 list_sort_internal(list, compare_func);
}

ListEntry *list_find_data(ListEntry *list,
                          ListEqualFunc callback,
                          ListValue data)
{
 ListEntry *rover;



 for (rover=list; rover != NULL; rover=rover->next) {
  if (callback(rover->data, data) != 0) {
   return rover;
  }
 }



 return NULL;
}

void list_iterate(ListEntry **list, ListIterator *iter)
{


 iter->prev_next = list;



 iter->current = NULL;
}

int list_iter_has_more(ListIterator *iter)
{
 if (iter->current == NULL || iter->current != *iter->prev_next) {






  return *iter->prev_next != NULL;

 } else {




  return iter->current->next != NULL;
 }
}

ListValue list_iter_next(ListIterator *iter)
{
 if (iter->current == NULL || iter->current != *iter->prev_next) {





  iter->current = *iter->prev_next;

 } else {




  iter->prev_next = &iter->current->next;
  iter->current = iter->current->next;
 }



 if (iter->current == NULL) {
  return LIST_NULL;
 } else {
  return iter->current->data;
 }
}

void list_iter_remove(ListIterator *iter)
{
 if (iter->current == NULL || iter->current != *iter->prev_next) {





 } else {



  *iter->prev_next = iter->current->next;

  if (iter->current->next != NULL) {
   iter->current->next->prev = iter->current->prev;
  }

  free(iter->current);
  iter->current = NULL;
 }
}
