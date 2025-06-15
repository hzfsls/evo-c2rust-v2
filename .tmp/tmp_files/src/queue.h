# 1 ".tmp/tmp_files/src/queue.h"
# 41 ".tmp/tmp_files/src/queue.h"
#ifndef ALGORITHM_QUEUE_H
#define ALGORITHM_QUEUE_H 

#ifdef __cplusplus
extern "C" {
#endif





typedef struct _Queue Queue;





typedef void *QueueValue;





#define QUEUE_NULL ((void *) 0)
# 73 ".tmp/tmp_files/src/queue.h"
Queue *queue_new(void);







void queue_free(Queue *queue);
# 93 ".tmp/tmp_files/src/queue.h"
int queue_push_head(Queue *queue, QueueValue data);
# 103 ".tmp/tmp_files/src/queue.h"
QueueValue queue_pop_head(Queue *queue);
# 114 ".tmp/tmp_files/src/queue.h"
QueueValue queue_peek_head(Queue *queue);
# 126 ".tmp/tmp_files/src/queue.h"
int queue_push_tail(Queue *queue, QueueValue data);
# 136 ".tmp/tmp_files/src/queue.h"
QueueValue queue_pop_tail(Queue *queue);
# 147 ".tmp/tmp_files/src/queue.h"
QueueValue queue_peek_tail(Queue *queue);
# 157 ".tmp/tmp_files/src/queue.h"
int queue_is_empty(Queue *queue);

#ifdef __cplusplus
}
#endif

#endif
