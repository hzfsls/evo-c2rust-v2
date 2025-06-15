# 1 ".tmp/tmp_files/src/hash-string.c"
# 21 ".tmp/tmp_files/src/hash-string.c"
#include <ctype.h>

#include "hash-string.h"



unsigned int string_hash(void *string)
{


 unsigned int result = 5381;
 unsigned char *p;

 p = (unsigned char *) string;

 while (*p != '\0') {
  result = (result << 5) + result + *p;
  ++p;
 }

 return result;
}




unsigned int string_nocase_hash(void *string)
{
 unsigned int result = 5381;
 unsigned char *p;

 p = (unsigned char *) string;

 while (*p != '\0') {
  result = (result << 5) + result + (unsigned int) tolower(*p);
  ++p;
 }

 return result;
}
