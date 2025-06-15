# 1 ".tmp/tmp_files/src/queue.c"
# 21 ".tmp/tmp_files/src/queue.c"
#include <stdlib.h>

#include "queue.h"



#ifdef ALLOC_TESTING
#include "alloc-testing.h"
#endif



typedef struct _QueueEntry QueueEntry;

struct _QueueEntry {
 QueueValue data;
 QueueEntry *prev;
 QueueEntry *next;
};

struct _Queue {
 QueueEntry *head;
 QueueEntry *tail;
};

Queue *queue_new(void)
{
 Queue *queue;

 queue = (Queue *) malloc(sizeof(Queue));

 if (queue == NULL) {
  return NULL;
 }

 queue->head = NULL;
 queue->tail = NULL;

 return queue;
}

void queue_free(Queue *queue)
{


 while (!queue_is_empty(queue)) {
  queue_pop_head(queue);
 }



 free(queue);
}

int queue_push_head(Queue *queue, QueueValue data)
{
 QueueEntry *new_entry;



 new_entry = malloc(sizeof(QueueEntry));

 if (new_entry == NULL) {
  return 0;
 }

 new_entry->data = data;
 new_entry->prev = NULL;
 new_entry->next = queue->head;



 if (queue->head == NULL) {




  queue->head = new_entry;
  queue->tail = new_entry;

 } else {




  queue->head->prev = new_entry;



  queue->head = new_entry;
 }

 return 1;
}

QueueValue queue_pop_head(Queue *queue)
{
 QueueEntry *entry;
 QueueValue result;



 if (queue_is_empty(queue)) {
  return QUEUE_NULL;
 }



 entry = queue->head;
 queue->head = entry->next;
 result = entry->data;

 if (queue->head == NULL) {




  queue->tail = NULL;
 } else {



  queue->head->prev = NULL;
 }



 free(entry);

 return result;
}

QueueValue queue_peek_head(Queue *queue)
{
 if (queue_is_empty(queue)) {
  return QUEUE_NULL;
 } else {
  return queue->head->data;
 }
}

int queue_push_tail(Queue *queue, QueueValue data)
{
 QueueEntry *new_entry;



 new_entry = malloc(sizeof(QueueEntry));

 if (new_entry == NULL) {
  return 0;
 }

 new_entry->data = data;
 new_entry->prev = queue->tail;
 new_entry->next = NULL;



 if (queue->tail == NULL) {




  queue->head = new_entry;
  queue->tail = new_entry;

 } else {




  queue->tail->next = new_entry;



  queue->tail = new_entry;
 }

 return 1;
}

QueueValue queue_pop_tail(Queue *queue)
{
 QueueEntry *entry;
 QueueValue result;



 if (queue_is_empty(queue)) {
  return QUEUE_NULL;
 }



 entry = queue->tail;
 queue->tail = entry->prev;
 result = entry->data;

 if (queue->tail == NULL) {




  queue->head = NULL;

 } else {



  queue->tail->next = NULL;
 }



 free(entry);

 return result;
}

QueueValue queue_peek_tail(Queue *queue)
{
 if (queue_is_empty(queue)) {
  return QUEUE_NULL;
 } else {
  return queue->tail->data;
 }
}

int queue_is_empty(Queue *queue)
{
 return queue->head == NULL;
}
