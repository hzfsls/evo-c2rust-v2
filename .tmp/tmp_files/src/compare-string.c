# 1 ".tmp/tmp_files/src/compare-string.c"
# 21 ".tmp/tmp_files/src/compare-string.c"
#include <ctype.h>
#include <stdlib.h>
#include <string.h>

#include "compare-string.h"



int string_equal(void *string1, void *string2)
{
 return strcmp((char *) string1, (char *) string2) == 0;
}

int string_compare(void *string1, void *string2)
{
 int result;

 result = strcmp((char *) string1, (char *) string2);

 if (result < 0) {
  return -1;
 } else if (result > 0) {
  return 1;
 } else {
  return 0;
 }
}



int string_nocase_equal(void *string1, void *string2)
{
 return string_nocase_compare((char *) string1, (char *) string2) == 0;
}





int string_nocase_compare(void *string1, void *string2)
{
 char *p1;
 char *p2;
 int c1, c2;



 p1 = (char *) string1;
 p2 = (char *) string2;

 for (;;) {

  c1 = tolower(*p1);
  c2 = tolower(*p2);

  if (c1 != c2) {



   if (c1 < c2) {
    return -1;
   } else {
    return 1;
   }
  }



  if (c1 == '\0')
   break;



  ++p1;
  ++p2;
 }



 return 0;
}
