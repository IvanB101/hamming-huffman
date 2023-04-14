#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>
#include "coding/coder.h"
#include "noise/generator.h"
#include "bitarr/bitarr.h"

int main() {
    int arr1[] = {1, 2, 128}, arr2[] = {2, 3, 4}, max[] = {UINT_MAX, UINT_MAX, UINT_MAX}, temp[] = {UINT_MAX, UINT_MAX, UINT_MAX};

    printf("Arr1: %s\n", to_bit_string((void*)arr1, sizeof(arr1)));
    printf("Arr2: %s\n", to_bit_string((void*)arr2, sizeof(arr2)));

    bit_and((void*)temp, (void*)arr1, sizeof(temp));
    bit_and((void*)temp, (void*)arr2, sizeof(temp));
    printf("And:  %s\n", to_bit_string((void*)temp, sizeof(temp)));

    bit_and((void*)temp, (void*)max, sizeof(temp));
    bit_and((void*)temp, (void*)arr1, sizeof(temp));
    bit_or((void*)temp, (void*)arr2, sizeof(temp));
    printf("Or:   %s\n", to_bit_string((void*)temp, sizeof(temp)));

    bit_and((void*)temp, (void*)max, sizeof(temp));
    bit_and((void*)temp, (void*)arr1, sizeof(temp));
    bit_not((void*)temp, sizeof(temp));
    printf("Not1: %s\n", to_bit_string((void*)temp, sizeof(temp)));

    bit_and((void*)temp, (void*)max, sizeof(temp));
    bit_and((void*)temp, (void*)arr2, sizeof(temp));
    bit_not((void*)temp, sizeof(temp));
    printf("Not2: %s\n", to_bit_string((void*)temp, sizeof(temp)));

    int i;
    for(i = 0; i < sizeof(temp) * 8; i++) {
        if(i % 2) {
            set_bit((void*)temp, i);
        } else {
            reset_bit((void*)temp, i);
        }
    }
    printf("Set Reset: %s\n", to_bit_string((void*)temp, sizeof(temp)));

    return 0;
}
