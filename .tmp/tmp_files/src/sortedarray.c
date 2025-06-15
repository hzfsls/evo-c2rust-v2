# 1 ".tmp/tmp_files/src/sortedarray.c"
# 27 ".tmp/tmp_files/src/sortedarray.c"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "sortedarray.h"

#ifdef ALLOC_TESTING
#include "alloc-testing.h"
#endif




struct _SortedArray {




 SortedArrayValue *data;




 unsigned int length;





 unsigned int _alloced;




 SortedArrayEqualFunc equ_func;




 SortedArrayCompareFunc cmp_func;
};



static unsigned int sortedarray_first_index(SortedArray *sortedarray,
                                   SortedArrayValue data, unsigned int left,
                                   unsigned int right)
{
 unsigned int index = left;

 while (left < right) {
  index = (left + right) / 2;

  int order = sortedarray->cmp_func(data,
                                    sortedarray->data[index]);
  if (order > 0) {
   left = index + 1;
  } else {
   right = index;
  }
 }

 return index;
}



static unsigned int sortedarray_last_index(SortedArray *sortedarray,
                                  SortedArrayValue data, unsigned int left,
                                  unsigned int right)
{
 unsigned int index = right;

 while (left < right) {
  index = (left + right) / 2;

  int order = sortedarray->cmp_func(data,
                                    sortedarray->data[index]);
  if (order <= 0) {
   left = index + 1;
  } else {
   right = index;
  }
 }

 return index;
}

SortedArrayValue *sortedarray_get(SortedArray *array, unsigned int i)
{

 if (array == NULL) {
  return NULL;
 }


 return array->data[i];
}

unsigned int sortedarray_length(SortedArray *array)
{
 return array->length;
}

SortedArray *sortedarray_new(unsigned int length,
                             SortedArrayEqualFunc equ_func,
                             SortedArrayCompareFunc cmp_func)
{

 if (equ_func == NULL || cmp_func == NULL) {
  return NULL;
 }


 if (length == 0) {
  length = 16;
 }

 SortedArrayValue *array = malloc(sizeof(SortedArrayValue) * length);


 if (array == NULL) {
  return NULL;
 }

 SortedArray *sortedarray = malloc(sizeof(SortedArray));


 if (sortedarray == NULL) {
  free(array);
  return NULL;
 }


 sortedarray->data = array;
 sortedarray->length = 0;
 sortedarray->_alloced = length;
 sortedarray->equ_func = equ_func;
 sortedarray->cmp_func = cmp_func;
 return sortedarray;
}

void sortedarray_free(SortedArray *sortedarray)
{
 if (sortedarray != NULL) {
  free(sortedarray->data);
  free(sortedarray);
 }
}

void sortedarray_remove(SortedArray *sortedarray, unsigned int index)
{

 sortedarray_remove_range(sortedarray, index, 1);
}

void sortedarray_remove_range(SortedArray *sortedarray, unsigned int index,
                              unsigned int length)
{



 if (index > sortedarray->length || index + length > sortedarray->length) {
  return;
 }


 memmove(&sortedarray->data[index],
         &sortedarray->data[index + length],
         (sortedarray->length - (index + length))
               * sizeof(SortedArrayValue));

 sortedarray->length -= length;
}

int sortedarray_insert(SortedArray *sortedarray, SortedArrayValue data)
{

 unsigned int left = 0;
 unsigned int right = sortedarray->length;
 unsigned int index = 0;


 right = (right > 1) ? right : 0;

 while (left != right) {
  index = (left + right) / 2;

  int order = sortedarray->cmp_func(data,
                                    sortedarray->data[index]);
  if (order < 0) {

   right = index;
  } else if (order > 0) {

   left = index + 1;
  } else {

   break;
  }
 }


 if (sortedarray->length > 0 && sortedarray->cmp_func(data,
                        sortedarray->data[index]) > 0) {
  index++;
 }


 if (sortedarray->length + 1 > sortedarray->_alloced) {

  unsigned int newsize;
  SortedArrayValue *data;

  newsize = sortedarray->_alloced * 2;
  data = realloc(sortedarray->data, sizeof(SortedArrayValue) * newsize);

  if (data == NULL) {
   return 0;
  } else {
   sortedarray->data = data;
   sortedarray->_alloced = newsize;
  }
 }


 memmove(&sortedarray->data[index + 1],
         &sortedarray->data[index],
         (sortedarray->length - index) * sizeof(SortedArrayValue));


 sortedarray->data[index] = data;
 ++(sortedarray->length);

 return 1;
}

int sortedarray_index_of(SortedArray *sortedarray, SortedArrayValue data)
{
 if (sortedarray == NULL) {
  return -1;
 }


 unsigned int left = 0;
 unsigned int right = sortedarray->length;
 unsigned int index = 0;


 right = (right > 1) ? right : 0;

 while (left != right) {
  index = (left + right) / 2;

  int order = sortedarray->cmp_func(data,
                                    sortedarray->data[index]);
  if (order < 0) {

   right = index;
  } else if (order > 0) {

   left = index + 1;
  } else {


   left = sortedarray_first_index(sortedarray, data, left,
                                  index);
   right = sortedarray_last_index(sortedarray, data,
                                  index, right);

   for (index = left; index <= right; index++) {
    if (sortedarray->equ_func(data,
                    sortedarray->data[index])) {
     return (int) index;
    }
   }


   return -1;
  }
 }

 return -1;
}

void sortedarray_clear(SortedArray *sortedarray)
{

 sortedarray->length = 0;
}
