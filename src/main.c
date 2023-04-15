#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>
#include "coding/coder.h"
#include "noise/generator.h"
#include "bitarr/bitarr.h"

int main() {
    char arr1[] = {0xff, 0x00, 0xff, 0x00, 0xff, 0xff, 0x00, 0x00}, arr2[] = {0,0,0,0,0,0,0,0};

    printf("Arr1: %s\n", to_bit_string((void*)arr1, sizeof(arr1)));
    printf("Arr2: %s\n", to_bit_string((void*)arr2, sizeof(arr2)));

    move((void*)arr1, (void*)arr2, 32, 7, 16);
    printf("Res:  %s\n", to_bit_string((void*)arr2, sizeof(arr2)));

    return 0;
}
