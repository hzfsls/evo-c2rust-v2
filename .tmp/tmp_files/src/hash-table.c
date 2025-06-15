# 1 ".tmp/tmp_files/src/hash-table.c"
# 23 ".tmp/tmp_files/src/hash-table.c"
#include <stdlib.h>
#include <string.h>

#include "hash-table.h"



#ifdef ALLOC_TESTING
#include "alloc-testing.h"
#endif

struct _HashTableEntry {
 HashTablePair pair;
 HashTableEntry *next;
};

struct _HashTable {
 HashTableEntry **table;
 unsigned int table_size;
 HashTableHashFunc hash_func;
 HashTableEqualFunc equal_func;
 HashTableKeyFreeFunc key_free_func;
 HashTableValueFreeFunc value_free_func;
 unsigned int entries;
 unsigned int prime_index;
};






static const unsigned int hash_table_primes[] = {
 193, 389, 769, 1543, 3079, 6151, 12289, 24593, 49157, 98317,
 196613, 393241, 786433, 1572869, 3145739, 6291469,
 12582917, 25165843, 50331653, 100663319, 201326611,
 402653189, 805306457, 1610612741,
};

static const unsigned int hash_table_num_primes
 = sizeof(hash_table_primes) / sizeof(int);




static int hash_table_allocate_table(HashTable *hash_table)
{
 unsigned int new_table_size;






 if (hash_table->prime_index < hash_table_num_primes) {
  new_table_size = hash_table_primes[hash_table->prime_index];
 } else {
  new_table_size = hash_table->entries * 10;
 }

 hash_table->table_size = new_table_size;



 hash_table->table = calloc(hash_table->table_size,
                            sizeof(HashTableEntry *));

 return hash_table->table != NULL;
}



static void hash_table_free_entry(HashTable *hash_table, HashTableEntry *entry)
{
 HashTablePair *pair;

 pair = &(entry->pair);




 if (hash_table->key_free_func != NULL) {
  hash_table->key_free_func(pair->key);
 }



 if (hash_table->value_free_func != NULL) {
  hash_table->value_free_func(pair->value);
 }



 free(entry);
}

HashTable *hash_table_new(HashTableHashFunc hash_func,
                          HashTableEqualFunc equal_func)
{
 HashTable *hash_table;



 hash_table = (HashTable *) malloc(sizeof(HashTable));

 if (hash_table == NULL) {
  return NULL;
 }

 hash_table->hash_func = hash_func;
 hash_table->equal_func = equal_func;
 hash_table->key_free_func = NULL;
 hash_table->value_free_func = NULL;
 hash_table->entries = 0;
 hash_table->prime_index = 0;



 if (!hash_table_allocate_table(hash_table)) {
  free(hash_table);

  return NULL;
 }

 return hash_table;
}

void hash_table_free(HashTable *hash_table)
{
 HashTableEntry *rover;
 HashTableEntry *next;
 unsigned int i;



 for (i=0; i<hash_table->table_size; ++i) {
  rover = hash_table->table[i];
  while (rover != NULL) {
   next = rover->next;
   hash_table_free_entry(hash_table, rover);
   rover = next;
  }
 }



 free(hash_table->table);



 free(hash_table);
}

void hash_table_register_free_functions(HashTable *hash_table,
                                        HashTableKeyFreeFunc key_free_func,
                                        HashTableValueFreeFunc value_free_func)
{
 hash_table->key_free_func = key_free_func;
 hash_table->value_free_func = value_free_func;
}


static int hash_table_enlarge(HashTable *hash_table)
{
 HashTableEntry **old_table;
 unsigned int old_table_size;
 unsigned int old_prime_index;
 HashTableEntry *rover;
 HashTablePair *pair;
 HashTableEntry *next;
 unsigned int index;
 unsigned int i;



 old_table = hash_table->table;
 old_table_size = hash_table->table_size;
 old_prime_index = hash_table->prime_index;



 ++hash_table->prime_index;

 if (!hash_table_allocate_table(hash_table)) {



  hash_table->table = old_table;
  hash_table->table_size = old_table_size;
  hash_table->prime_index = old_prime_index;

  return 0;
 }



 for (i=0; i<old_table_size; ++i) {
  rover = old_table[i];

  while (rover != NULL) {
   next = rover->next;



   pair = &(rover->pair);



   index = hash_table->hash_func(pair->key) % hash_table->table_size;



   rover->next = hash_table->table[index];
   hash_table->table[index] = rover;



   rover = next;
  }
 }



 free(old_table);

 return 1;
}

int hash_table_insert(HashTable *hash_table, HashTableKey key,
                      HashTableValue value)
{
 HashTableEntry *rover;
 HashTablePair *pair;
 HashTableEntry *newentry;
 unsigned int index;





 if ((hash_table->entries * 3) / hash_table->table_size > 0) {



  if (!hash_table_enlarge(hash_table)) {



   return 0;
  }
 }



 index = hash_table->hash_func(key) % hash_table->table_size;




 rover = hash_table->table[index];

 while (rover != NULL) {



  pair = &(rover->pair);

  if (hash_table->equal_func(pair->key, key) != 0) {






   if (hash_table->value_free_func != NULL) {
    hash_table->value_free_func(pair->value);
   }




   if (hash_table->key_free_func != NULL) {
    hash_table->key_free_func(pair->key);
   }

   pair->key = key;
   pair->value = value;



   return 1;
  }

  rover = rover->next;
 }



 newentry = (HashTableEntry *) malloc(sizeof(HashTableEntry));

 if (newentry == NULL) {
  return 0;
 }

 newentry->pair.key = key;
 newentry->pair.value = value;



 newentry->next = hash_table->table[index];
 hash_table->table[index] = newentry;



 ++hash_table->entries;



 return 1;
}

HashTableValue hash_table_lookup(HashTable *hash_table, HashTableKey key)
{
 HashTableEntry *rover;
 HashTablePair *pair;
 unsigned int index;



 index = hash_table->hash_func(key) % hash_table->table_size;




 rover = hash_table->table[index];

 while (rover != NULL) {
  pair = &(rover->pair);

  if (hash_table->equal_func(key, pair->key) != 0) {



   return pair->value;
  }

  rover = rover->next;
 }



 return HASH_TABLE_NULL;
}

int hash_table_remove(HashTable *hash_table, HashTableKey key)
{
 HashTableEntry **rover;
 HashTableEntry *entry;
 HashTablePair *pair;
 unsigned int index;
 int result;



 index = hash_table->hash_func(key) % hash_table->table_size;






 result = 0;
 rover = &hash_table->table[index];

 while (*rover != NULL) {

  pair = &((*rover)->pair);

  if (hash_table->equal_func(key, pair->key) != 0) {



   entry = *rover;



   *rover = entry->next;



   hash_table_free_entry(hash_table, entry);



   --hash_table->entries;

   result = 1;

   break;
  }



  rover = &((*rover)->next);
 }

 return result;
}

unsigned int hash_table_num_entries(HashTable *hash_table)
{
 return hash_table->entries;
}

void hash_table_iterate(HashTable *hash_table, HashTableIterator *iterator)
{
 unsigned int chain;

 iterator->hash_table = hash_table;



 iterator->next_entry = NULL;



 for (chain=0; chain<hash_table->table_size; ++chain) {

  if (hash_table->table[chain] != NULL) {
   iterator->next_entry = hash_table->table[chain];
   iterator->next_chain = chain;
   break;
  }
 }
}

int hash_table_iter_has_more(HashTableIterator *iterator)
{
 return iterator->next_entry != NULL;
}

HashTablePair hash_table_iter_next(HashTableIterator *iterator)
{
 HashTableEntry *current_entry;
 HashTable *hash_table;
 HashTablePair pair = {NULL, NULL};
 unsigned int chain;

 hash_table = iterator->hash_table;

 if (iterator->next_entry == NULL) {
  return pair;
 }



 current_entry = iterator->next_entry;
 pair = current_entry->pair;



 if (current_entry->next != NULL) {



  iterator->next_entry = current_entry->next;

 } else {



  chain = iterator->next_chain + 1;



  iterator->next_entry = NULL;

  while (chain < hash_table->table_size) {



   if (hash_table->table[chain] != NULL) {
    iterator->next_entry = hash_table->table[chain];
    break;
   }



   ++chain;
  }

  iterator->next_chain = chain;
 }

 return pair;
}
