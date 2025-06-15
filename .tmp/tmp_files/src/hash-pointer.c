# 1 ".tmp/tmp_files/src/hash-pointer.c"
# 21 ".tmp/tmp_files/src/hash-pointer.c"
#include <limits.h>

#include "hash-pointer.h"



unsigned int pointer_hash(void *location)
{
 return (unsigned int) (unsigned long) location;
}
