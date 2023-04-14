#ifndef BITVEC
#define BITVEC

typedef struct {
    int* vector;
    int primero;
    int ultimo;
} bitvec;

int bit_len(bitvec vector);

bitvec* take(bitvec vector, int cantidad);

int put(bitvec vector_a, bitvec vector_de);

#endif 
