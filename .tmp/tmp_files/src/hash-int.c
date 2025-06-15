# 1 ".tmp/tmp_files/src/hash-int.c"
# 21 ".tmp/tmp_files/src/hash-int.c"
#include "hash-int.h"



unsigned int int_hash(void *vlocation)
{
 int *location;

 location = (int *) vlocation;

 return (unsigned int) *location;
}
