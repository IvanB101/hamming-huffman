#ifndef BITVEC
#define BITVEC

typedef struct bitvec {
    int* vector;
    int primero;
    int ultimo;
};

bitvec take(bitvec vector, int cantidad);

bool put(bitvec vector_a, bitvec vector_de);

bitvec and(bitvec a, bitvec b);

bitvec or(bitvec a, bitvec b);

bitvec xor(bitvec a, bitvec b);

#endif 
