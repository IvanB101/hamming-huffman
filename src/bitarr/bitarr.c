#include <stdlib.h>

#include "bitarr.h"

void set_bit(void *arr, int posicion) {
    char mask = 1 << (7 - posicion % 8);

    ((char*)arr)[posicion / 8] |= mask;
}

void reset_bit(void *arr, int posicion) {
    char mask = 1 << (7 - posicion % 8);
    mask = ~mask;

    ((char*)arr)[posicion / 8] &= mask;
}

void flip_bit(void *arr, int posicion) {
    char mask = 1 << (7 - posicion % 8);

    if(((char*)arr)[posicion / 8] &= mask) {
        reset_bit(arr, posicion);
    } else {
        set_bit(arr, posicion);
    }
}

void bit_and(void* arr1, void* arr2, int size1) {
    int i;
    // Los and se realizan de a palabras del procesador
    for(i = 0; i < size1 / sizeof(int); i++) {
        ((int*) arr1)[i] &= ((int*) arr2)[i];
    }
    // Se realiza la parte que no es multiplo del tamanio de int
    for(i = i * sizeof(int); i < size1; i++) {
        ((char*) arr1)[i] &= ((char*) arr2)[i];
    }
}

void bit_or(void* arr1, void* arr2, int size1) {
    int i;
    // Los and se realizan de a palabras del procesador
    for(i = 0; i < size1 / sizeof(int); i++) {
        ((int*) arr1)[i] |= ((int*) arr2)[i];
    }
    // Se realiza la parte que no es multiplo del tamanio de int
    for(i = i * sizeof(int); i < size1; i++) {
        ((char*) arr1)[i] |= ((char*) arr2)[i];
    }
}

void bit_not(void* arr1, int size) {
    int i;
    // Los and se realizan de a palabras del procesador
    for(i = 0; i < size / sizeof(int); i++) {
        ((int*) arr1)[i] = ~((int*) arr1)[i];
    }
    // Se realiza la parte que no es multiplo del tamanio de int
    for(i = i * sizeof(int); i < size; i++) {
        ((char*) arr1)[i] = ~((char*) arr1)[i];
    }
}

char* to_bit_string(void* arr, int size) {
    char* ret = (char*)malloc(size * 9 + 1);

    int i, j, m;
    for(i = 0; i < size; i++) {
        char temp = ((char*)arr)[i];

        for(j = 0, m = 1 << 7; j < 8; j++) {
            ret[i * 9 + j] = ((temp & m) != 0) + '0';

            m >>= 1;
        }
        ret[i * 9 + j] = ' ';
    }
    ret[size * 9] = '\0';

    return ret;
}
