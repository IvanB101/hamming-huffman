#ifndef BITARR
#define BITARR

void set_bit(void *arr, int posicion);

void reset_bit(void *arr, int posicion);

void flip_bit(void *arr, int posicion);

void bit_and(void* arr1, void* arr2, int size1);

void bit_or(void* arr1, void* arr2, int size1);

void bit_not(void* arr1, int size);

char* to_bit_string(void* arr, int size);

#endif 
