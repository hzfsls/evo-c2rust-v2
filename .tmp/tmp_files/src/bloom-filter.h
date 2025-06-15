# 1 ".tmp/tmp_files/src/bloom-filter.h"
# 40 ".tmp/tmp_files/src/bloom-filter.h"
#ifndef ALGORITHM_BLOOM_FILTER_H
#define ALGORITHM_BLOOM_FILTER_H 

#ifdef __cplusplus
extern "C" {
#endif





typedef struct _BloomFilter BloomFilter;





typedef void *BloomFilterValue;
# 67 ".tmp/tmp_files/src/bloom-filter.h"
typedef unsigned int (*BloomFilterHashFunc)(BloomFilterValue data);
# 89 ".tmp/tmp_files/src/bloom-filter.h"
BloomFilter *bloom_filter_new(unsigned int table_size,
                              BloomFilterHashFunc hash_func,
                              unsigned int num_functions);







void bloom_filter_free(BloomFilter *bloomfilter);
# 108 ".tmp/tmp_files/src/bloom-filter.h"
void bloom_filter_insert(BloomFilter *bloomfilter, BloomFilterValue value);
# 121 ".tmp/tmp_files/src/bloom-filter.h"
int bloom_filter_query(BloomFilter *bloomfilter, BloomFilterValue value);
# 132 ".tmp/tmp_files/src/bloom-filter.h"
void bloom_filter_read(BloomFilter *bloomfilter, unsigned char *array);
# 146 ".tmp/tmp_files/src/bloom-filter.h"
void bloom_filter_load(BloomFilter *bloomfilter, unsigned char *array);
# 165 ".tmp/tmp_files/src/bloom-filter.h"
BloomFilter *bloom_filter_union(BloomFilter *filter1,
                                BloomFilter *filter2);
# 185 ".tmp/tmp_files/src/bloom-filter.h"
BloomFilter *bloom_filter_intersection(BloomFilter *filter1,
                                       BloomFilter *filter2);

#ifdef __cplusplus
}
#endif

#endif
