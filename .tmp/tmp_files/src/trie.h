# 1 ".tmp/tmp_files/src/trie.h"
# 40 ".tmp/tmp_files/src/trie.h"
#ifndef ALGORITHM_TRIE_H
#define ALGORITHM_TRIE_H 

#ifdef __cplusplus
extern "C" {
#endif





typedef struct _Trie Trie;





typedef void *TrieValue;





#define TRIE_NULL ((void *) 0)
# 73 ".tmp/tmp_files/src/trie.h"
Trie *trie_new(void);







void trie_free(Trie *trie);
# 95 ".tmp/tmp_files/src/trie.h"
int trie_insert(Trie *trie, char *key, TrieValue value);
# 110 ".tmp/tmp_files/src/trie.h"
int trie_insert_binary(Trie *trie, unsigned char *key,
                       int key_length, TrieValue value);
# 124 ".tmp/tmp_files/src/trie.h"
TrieValue trie_lookup(Trie *trie, char *key);
# 138 ".tmp/tmp_files/src/trie.h"
TrieValue trie_lookup_binary(Trie *trie, unsigned char *key, int key_length);
# 151 ".tmp/tmp_files/src/trie.h"
int trie_remove(Trie *trie, char *key);
# 165 ".tmp/tmp_files/src/trie.h"
int trie_remove_binary(Trie *trie, unsigned char *key, int key_length);
# 174 ".tmp/tmp_files/src/trie.h"
unsigned int trie_num_entries(Trie *trie);

#ifdef __cplusplus
}
#endif

#endif
