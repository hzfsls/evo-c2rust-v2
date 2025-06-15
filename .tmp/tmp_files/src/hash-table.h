# 1 ".tmp/tmp_files/src/hash-table.h"
# 45 ".tmp/tmp_files/src/hash-table.h"
#ifndef ALGORITHM_HASH_TABLE_H
#define ALGORITHM_HASH_TABLE_H 

#ifdef __cplusplus
extern "C" {
#endif





typedef struct _HashTable HashTable;





typedef struct _HashTableIterator HashTableIterator;





typedef struct _HashTableEntry HashTableEntry;





typedef void *HashTableKey;





typedef void *HashTableValue;






typedef struct _HashTablePair{
 HashTableKey key;
 HashTableValue value;
} HashTablePair;





struct _HashTableIterator {
 HashTable *hash_table;
 HashTableEntry *next_entry;
 unsigned int next_chain;
};





#define HASH_TABLE_NULL ((void *) 0)
# 116 ".tmp/tmp_files/src/hash-table.h"
typedef unsigned int (*HashTableHashFunc)(HashTableKey value);
# 125 ".tmp/tmp_files/src/hash-table.h"
typedef int (*HashTableEqualFunc)(HashTableKey value1, HashTableKey value2);






typedef void (*HashTableKeyFreeFunc)(HashTableKey value);






typedef void (*HashTableValueFreeFunc)(HashTableValue value);
# 153 ".tmp/tmp_files/src/hash-table.h"
HashTable *hash_table_new(HashTableHashFunc hash_func,
                          HashTableEqualFunc equal_func);







void hash_table_free(HashTable *hash_table);
# 173 ".tmp/tmp_files/src/hash-table.h"
void hash_table_register_free_functions(HashTable *hash_table,
                                        HashTableKeyFreeFunc key_free_func,
                                        HashTableValueFreeFunc value_free_func);
# 189 ".tmp/tmp_files/src/hash-table.h"
int hash_table_insert(HashTable *hash_table,
                      HashTableKey key,
                      HashTableValue value);
# 202 ".tmp/tmp_files/src/hash-table.h"
HashTableValue hash_table_lookup(HashTable *hash_table,
                                 HashTableKey key);
# 214 ".tmp/tmp_files/src/hash-table.h"
int hash_table_remove(HashTable *hash_table, HashTableKey key);
# 223 ".tmp/tmp_files/src/hash-table.h"
unsigned int hash_table_num_entries(HashTable *hash_table);
# 233 ".tmp/tmp_files/src/hash-table.h"
void hash_table_iterate(HashTable *hash_table, HashTableIterator *iter);
# 245 ".tmp/tmp_files/src/hash-table.h"
int hash_table_iter_has_more(HashTableIterator *iterator);
# 262 ".tmp/tmp_files/src/hash-table.h"
HashTablePair hash_table_iter_next(HashTableIterator *iterator);

#ifdef __cplusplus
}
#endif

#endif
