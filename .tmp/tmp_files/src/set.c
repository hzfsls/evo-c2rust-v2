# 1 ".tmp/tmp_files/src/set.c"
# 21 ".tmp/tmp_files/src/set.c"
#include <stdlib.h>
#include <string.h>
#include "set.h"



#ifdef ALLOC_TESTING
#include "alloc-testing.h"
#endif



struct _SetEntry {
 SetValue data;
 SetEntry *next;
};

struct _Set {
 SetEntry **table;
 unsigned int entries;
 unsigned int table_size;
 unsigned int prime_index;
 SetHashFunc hash_func;
 SetEqualFunc equal_func;
 SetFreeFunc free_func;
};






static const unsigned int set_primes[] = {
 193, 389, 769, 1543, 3079, 6151, 12289, 24593, 49157, 98317,
 196613, 393241, 786433, 1572869, 3145739, 6291469,
 12582917, 25165843, 50331653, 100663319, 201326611,
 402653189, 805306457, 1610612741,
};

static const unsigned int set_num_primes = sizeof(set_primes) / sizeof(int);

static int set_allocate_table(Set *set)
{





 if (set->prime_index < set_num_primes) {
  set->table_size = set_primes[set->prime_index];
 } else {
  set->table_size = set->entries * 10;
 }



 set->table = calloc(set->table_size, sizeof(SetEntry *));

 return set->table != NULL;
}

static void set_free_entry(Set *set, SetEntry *entry)
{



 if (set->free_func != NULL) {
  set->free_func(entry->data);
 }



 free(entry);
}

Set *set_new(SetHashFunc hash_func, SetEqualFunc equal_func)
{
 Set *new_set;



 new_set = (Set *) malloc(sizeof(Set));

 if (new_set == NULL) {
  return NULL;
 }

 new_set->hash_func = hash_func;
 new_set->equal_func = equal_func;
 new_set->entries = 0;
 new_set->prime_index = 0;
 new_set->free_func = NULL;



 if (!set_allocate_table(new_set)) {
  free(new_set);
  return NULL;
 }

 return new_set;
}

void set_free(Set *set)
{
 SetEntry *rover;
 SetEntry *next;
 unsigned int i;



 for (i=0; i<set->table_size; ++i) {
  rover = set->table[i];

  while (rover != NULL) {
   next = rover->next;



   set_free_entry(set, rover);



   rover = next;
  }
 }



 free(set->table);



 free(set);
}

void set_register_free_function(Set *set, SetFreeFunc free_func)
{
 set->free_func = free_func;
}

static int set_enlarge(Set *set)
{
 SetEntry *rover;
 SetEntry *next;
 SetEntry **old_table;
 unsigned int old_table_size;
 unsigned int old_prime_index;
 unsigned int index;
 unsigned int i;



 old_table = set->table;
 old_table_size = set->table_size;
 old_prime_index = set->prime_index;



 ++set->prime_index;



 if (!set_allocate_table(set)) {
  set->table = old_table;
  set->table_size = old_table_size;
  set->prime_index = old_prime_index;

  return 0;
 }




 for (i=0; i<old_table_size; ++i) {



  rover = old_table[i];

  while (rover != NULL) {

   next = rover->next;



   index = set->hash_func(rover->data) % set->table_size;
   rover->next = set->table[index];
   set->table[index] = rover;



   rover = next;
  }
 }



 free(old_table);



 return 1;
}

int set_insert(Set *set, SetValue data)
{
 SetEntry *newentry;
 SetEntry *rover;
 unsigned int index;




 if ((set->entries * 3) / set->table_size > 0) {




  if (!set_enlarge(set)) {
   return 0;
  }
 }




 index = set->hash_func(data) % set->table_size;




 rover = set->table[index];

 while (rover != NULL) {

  if (set->equal_func(data, rover->data) != 0) {



   return 0;
  }

  rover = rover->next;
 }





 newentry = (SetEntry *) malloc(sizeof(SetEntry));

 if (newentry == NULL) {
  return 0;
 }

 newentry->data = data;



 newentry->next = set->table[index];
 set->table[index] = newentry;



 ++set->entries;



 return 1;
}

int set_remove(Set *set, SetValue data)
{
 SetEntry **rover;
 SetEntry *entry;
 unsigned int index;



 index = set->hash_func(data) % set->table_size;



 rover = &set->table[index];

 while (*rover != NULL) {
  if (set->equal_func(data, (*rover)->data) != 0) {



   entry = *rover;



   *rover = entry->next;



   --set->entries;



   set_free_entry(set, entry);

   return 1;
  }



  rover = &((*rover)->next);
 }



 return 0;
}

int set_query(Set *set, SetValue data)
{
 SetEntry *rover;
 unsigned int index;



 index = set->hash_func(data) % set->table_size;



 rover = set->table[index];

 while (rover != NULL) {
  if (set->equal_func(data, rover->data) != 0) {



   return 1;
  }



  rover = rover->next;
 }



 return 0;
}

unsigned int set_num_entries(Set *set)
{
 return set->entries;
}

SetValue *set_to_array(Set *set)
{
 SetValue *array;
 int array_counter;
 unsigned int i;
 SetEntry *rover;



 array = malloc(sizeof(SetValue) * set->entries);

 if (array == NULL) {
  return NULL;
 }

 array_counter = 0;



 for (i=0; i<set->table_size; ++i) {

  rover = set->table[i];

  while (rover != NULL) {



   array[array_counter] = rover->data;
   ++array_counter;



   rover = rover->next;
  }
 }

 return array;
}

Set *set_union(Set *set1, Set *set2)
{
 SetIterator iterator;
 Set *new_set;
 SetValue value;

 new_set = set_new(set1->hash_func, set1->equal_func);

 if (new_set == NULL) {
  return NULL;
 }



 set_iterate(set1, &iterator);

 while (set_iter_has_more(&iterator)) {



  value = set_iter_next(&iterator);



  if (!set_insert(new_set, value)) {



   set_free(new_set);
   return NULL;
  }
 }



 set_iterate(set2, &iterator);

 while (set_iter_has_more(&iterator)) {



  value = set_iter_next(&iterator);




  if (set_query(new_set, value) == 0) {
   if (!set_insert(new_set, value)) {



    set_free(new_set);
    return NULL;
   }
  }
 }

 return new_set;
}

Set *set_intersection(Set *set1, Set *set2)
{
 Set *new_set;
 SetIterator iterator;
 SetValue value;

 new_set = set_new(set1->hash_func, set2->equal_func);

 if (new_set == NULL) {
  return NULL;
 }



 set_iterate(set1, &iterator);

 while (set_iter_has_more(&iterator)) {



  value = set_iter_next(&iterator);




  if (set_query(set2, value) != 0) {




   if (!set_insert(new_set, value)) {
    set_free(new_set);

    return NULL;
   }
  }
 }

 return new_set;
}

void set_iterate(Set *set, SetIterator *iter)
{
 unsigned int chain;

 iter->set = set;
 iter->next_entry = NULL;



 for (chain = 0; chain < set->table_size; ++chain) {



  if (set->table[chain] != NULL) {
   iter->next_entry = set->table[chain];
   break;
  }
 }

 iter->next_chain = chain;
}

SetValue set_iter_next(SetIterator *iterator)
{
 Set *set;
 SetValue result;
 SetEntry *current_entry;
 unsigned int chain;

 set = iterator->set;



 if (iterator->next_entry == NULL) {
  return SET_NULL;
 }



 current_entry = iterator->next_entry;
 result = current_entry->data;



 if (current_entry->next != NULL) {



  iterator->next_entry = current_entry->next;

 } else {



  iterator->next_entry = NULL;



  chain = iterator->next_chain + 1;

  while (chain < set->table_size) {



   if (set->table[chain] != NULL) {



    iterator->next_entry = set->table[chain];

    break;
   }



   ++chain;
  }

  iterator->next_chain = chain;
 }

 return result;
}

int set_iter_has_more(SetIterator *iterator)
{
 return iterator->next_entry != NULL;
}
