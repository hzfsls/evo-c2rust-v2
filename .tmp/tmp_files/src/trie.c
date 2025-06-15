# 1 ".tmp/tmp_files/src/trie.c"
# 23 ".tmp/tmp_files/src/trie.c"
#include <stdlib.h>
#include <string.h>

#include "trie.h"



#ifdef ALLOC_TESTING
#include "alloc-testing.h"
#endif

typedef struct _TrieNode TrieNode;

struct _TrieNode {
 TrieValue data;
 unsigned int use_count;
 TrieNode *next[256];
};

struct _Trie {
 TrieNode *root_node;
};

Trie *trie_new(void)
{
 Trie *new_trie;

 new_trie = (Trie *) malloc(sizeof(Trie));

 if (new_trie == NULL) {
  return NULL;
 }

 new_trie->root_node = NULL;

 return new_trie;
}

static void trie_free_list_push(TrieNode **list, TrieNode *node)
{
 node->data = *list;
 *list = node;
}

static TrieNode *trie_free_list_pop(TrieNode **list)
{
 TrieNode *result;

 result = *list;
 *list = result->data;

 return result;
}

void trie_free(Trie *trie)
{
 TrieNode *free_list;
 TrieNode *node;
 int i;

 free_list = NULL;



 if (trie->root_node != NULL) {
  trie_free_list_push(&free_list, trie->root_node);
 }





 while (free_list != NULL) {
  node = trie_free_list_pop(&free_list);



  for (i=0; i<256; ++i) {
   if (node->next[i] != NULL) {
    trie_free_list_push(&free_list, node->next[i]);
   }
  }



  free(node);
 }



 free(trie);
}

static TrieNode *trie_find_end(Trie *trie, char *key)
{
 TrieNode *node;
 char *p;



 node = trie->root_node;

 for (p=key; *p != '\0'; ++p) {

  if (node == NULL) {


   return NULL;
  }



  node = node->next[(unsigned char) *p];
 }



 return node;
}

static TrieNode *trie_find_end_binary(Trie *trie, unsigned char *key,
                                      int key_length)
{
 TrieNode *node;
 int j;
 int c;



 node = trie->root_node;

 for (j=0; j<key_length; j++) {

  if (node == NULL) {

   return NULL;
  }

  c = (unsigned char) key[j];



  node = node->next[c];
 }



 return node;
}



static void trie_insert_rollback(Trie *trie, unsigned char *key)
{
 TrieNode *node;
 TrieNode **prev_ptr;
 TrieNode *next_node;
 TrieNode **next_prev_ptr;
 unsigned char *p;






 node = trie->root_node;
 prev_ptr = &trie->root_node;
 p = key;

 while (node != NULL) {



  next_prev_ptr = &node->next[(unsigned char) *p];
  next_node = *next_prev_ptr;
  ++p;




  --node->use_count;

  if (node->use_count == 0) {
   free(node);

   if (prev_ptr != NULL) {
    *prev_ptr = NULL;
   }

   next_prev_ptr = NULL;
  }



  node = next_node;
  prev_ptr = next_prev_ptr;
 }
}

int trie_insert(Trie *trie, char *key, TrieValue value)
{
 TrieNode **rover;
 TrieNode *node;
 char *p;
 int c;



 if (value == TRIE_NULL) {
  return 0;
 }



 node = trie_find_end(trie, key);




 if (node != NULL && node->data != TRIE_NULL) {
  node->data = value;
  return 1;
 }




 rover = &trie->root_node;
 p = key;

 for (;;) {

  node = *rover;

  if (node == NULL) {



   node = (TrieNode *) calloc(1, sizeof(TrieNode));

   if (node == NULL) {




    trie_insert_rollback(trie,
                         (unsigned char *) key);

    return 0;
   }

   node->data = TRIE_NULL;



   *rover = node;
  }



  ++node->use_count;



  c = (unsigned char) *p;



  if (c == '\0') {



   node->data = value;

   break;
  }



  rover = &node->next[c];
  ++p;
 }

 return 1;
}


int trie_insert_binary(Trie *trie, unsigned char *key, int key_length,
                       TrieValue value)
{
 TrieNode **rover;
 TrieNode *node;
 int p,c;



 if (value == TRIE_NULL) {
  return 0;
 }



 node = trie_find_end_binary(trie, key, key_length);




 if (node != NULL && node->data != TRIE_NULL) {
  node->data = value;
  return 1;
 }




 rover = &trie->root_node;
 p = 0;

 for (;;) {

  node = *rover;

  if (node == NULL) {



   node = (TrieNode *) calloc(1, sizeof(TrieNode));

   if (node == NULL) {




    trie_insert_rollback(trie, key);

    return 0;
   }

   node->data = TRIE_NULL;



   *rover = node;
  }



  ++node->use_count;



  c = (unsigned char) key[p];



  if (p == key_length) {



   node->data = value;

   break;
  }



  rover = &node->next[c];
  ++p;
 }

 return 1;
}

int trie_remove_binary(Trie *trie, unsigned char *key, int key_length)
{
 TrieNode *node;
 TrieNode *next;
 TrieNode **last_next_ptr;
 int p, c;



 node = trie_find_end_binary(trie, key, key_length);

 if (node != NULL && node->data != TRIE_NULL) {
  node->data = TRIE_NULL;
 } else {
  return 0;
 }




 node = trie->root_node;
 last_next_ptr = &trie->root_node;
 p = 0;

 for (;;) {


  c = (unsigned char) key[p];
  next = node->next[c];



  --node->use_count;

  if (node->use_count <= 0) {
   free(node);







   if (last_next_ptr != NULL) {
    *last_next_ptr = NULL;
    last_next_ptr = NULL;
   }
  }


  if (p == key_length) {
   break;
  } else {
   ++p;
  }





  if (last_next_ptr != NULL) {
   last_next_ptr = &node->next[c];
  }



  node = next;
 }



 return 1;
}

int trie_remove(Trie *trie, char *key)
{
 TrieNode *node;
 TrieNode *next;
 TrieNode **last_next_ptr;
 char *p;
 int c;



 node = trie_find_end(trie, key);

 if (node != NULL && node->data != TRIE_NULL) {
  node->data = TRIE_NULL;
 } else {
  return 0;
 }




 node = trie->root_node;
 last_next_ptr = &trie->root_node;
 p = key;

 for (;;) {



  c = (unsigned char) *p;
  next = node->next[c];



  --node->use_count;

  if (node->use_count <= 0) {
   free(node);







   if (last_next_ptr != NULL) {
    *last_next_ptr = NULL;
    last_next_ptr = NULL;
   }
  }



  if (c == '\0') {
   break;
  } else {
   ++p;
  }





  if (last_next_ptr != NULL) {
   last_next_ptr = &node->next[c];
  }



  node = next;
 }



 return 1;
}

TrieValue trie_lookup(Trie *trie, char *key)
{
 TrieNode *node;

 node = trie_find_end(trie, key);

 if (node != NULL) {
  return node->data;
 } else {
  return TRIE_NULL;
 }
}

TrieValue trie_lookup_binary(Trie *trie, unsigned char *key, int key_length)
{
 TrieNode *node;

 node = trie_find_end_binary(trie, key, key_length);

 if (node != NULL) {
  return node->data;
 } else {
  return TRIE_NULL;
 }
}

unsigned int trie_num_entries(Trie *trie)
{



 if (trie->root_node == NULL) {
  return 0;
 } else {
  return trie->root_node->use_count;
 }
}
