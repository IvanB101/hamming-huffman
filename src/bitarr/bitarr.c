#include <math.h>
#include <stdlib.h>

#include "bitarr.h"

int min(int a, int b) {
    return (a < b)? a : b;
}

void set_bit(void *arr, int position) {
    char mask = 1 << (7 - position % 8);

    ((char*)arr)[position / 8] |= mask;
}

void reset_bit(void *arr, int position) {
    char mask = 1 << (7 - position % 8);
    mask = ~mask;

    ((char*)arr)[position / 8] &= mask;
}

void flip_bit(void *arr, int position) {
    char mask = 1 << (7 - position % 8);

    if(((char*)arr)[position / 8] &= mask) {
        reset_bit(arr, position);
    } else {
        set_bit(arr, position);
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

void move(void* from, void* to, int start_from, int start_to, int size) {
    int passed = 0, current_from = start_from, current_to = start_to;

    while(passed < size) {
        int to_move = min(min(current_from % 32, current_to % 32), size - passed);

        int mask = (1 << to_move) - 1;
        
        int int_from = ((int*)from)[current_from / 32];
        int temp = int_from & (mask << (current_from - to_move));

        mask <<= current_to - to_move;
        temp <<= current_to - to_move;
        
        int int_to = ((int*)to)[current_to / 32];
        int_to &= ~mask;
        int_to |= temp;

        passed += to_move;
        current_from += to_move;
        current_to += to_move;
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

