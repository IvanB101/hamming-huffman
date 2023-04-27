#include "c_procedures.h"

#include "coding/coder.h"
#include "noise/generator.h"
#include "bitarr/bitarr.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>

char* get_extention(char* path);

void change_extention(char* path, char* new_ext);

char* encode(char* path, uint32_t block_size) {
    char* err;
    char* ext = get_extention(path);
    char* valid = "txt";

    if(strcmp(ext, "txt")) {
        return "Invalid file extention";
    }

    uint32_t exponent;
    char* new_ext;
    switch (block_size) {
        case 32:
            new_ext = "HA1";
            exponent = 5;
            break;
        case 2048:
            new_ext = "HA2";
            exponent = 11;
            break;
        case 65536:
            new_ext = "HA3";
            exponent = 16;
            break;
        default:
            return "Invalid block size";
    }

    FILE *fd, *res;
    fd = fopen(path, "rb");

    change_extention(path, new_ext);
    res = fopen(path, "wb");

    if(errno) {
        return strerror(errno);
    }

    err = encode_i(fd, res, block_size, exponent);

    fclose(fd);
    fclose(res);

    if(err) {
        return err;
    }

    return NULL;
}

char* decode(char* path, int correct) {
    char* err;
    char* ext = get_extention(path);
    uint32_t ext_len = strlen(ext);

    uint32_t exponent, block_size;
    char* new_ext;
    if (!strncmp(ext, "HA1", ext_len) || !strncmp(ext, "HE1", ext_len)) {
        block_size = 32;
        exponent = 5;
    } else if (!strncmp(ext, "HA2", ext_len) || !strncmp(ext, "HE2", ext_len)) {
        block_size = 2048;
        exponent = 11;
    } else if (!strncmp(ext, "HA3", ext_len) || !strncmp(ext, "HE3", ext_len)) {
        block_size = 65536;
        exponent = 16;
    } else {
        return "Invalid file extention";
    }

    FILE *fd, *res;
    fd = fopen(path, "rb");

    ext[0] = 'D';
    if(correct) {
        ext[1] = 'C';
    } else {
        ext[1] = 'E';
    }

    res = fopen(path, "wb");

    if(errno) {
        return strerror(errno);
    }

    err = decode_i(fd, res, block_size, exponent, correct);

    fclose(fd);
    fclose(res);

    if(err) {
        return err;
    }

    return NULL;
}

char* corrupt(char* path) {
    char* err;
    char* ext = get_extention(path);
    uint32_t ext_len = strlen(ext);

    uint32_t exponent, block_size;
    char* new_ext;
    if (!strncmp(ext, "HA1", ext_len)) {
        block_size = 32;
        exponent = 5;
    } else if (!strncmp(ext, "HA2", ext_len)) {
        block_size = 2048;
        exponent = 11;
    } else if (!strncmp(ext, "HA3", ext_len)) {
        block_size = 65536;
        exponent = 16;
    } else {
        return "Invalid file extention";
    }

    FILE *fd, *res;
    fd = fopen(path, "rb");

    ext[1] = 'E';
    res = fopen(path, "wb");

    if(errno) {
        return strerror(errno);
    }

    err = corrupt_i(fd, res, block_size, exponent);

    fclose(fd);
    fclose(res);

    if(err) {
        return err;
    }

    return NULL;
}

char* compress(char* path) {
    return "Not yet implemented";
}

char* decompress(char* path) {
    return "Not yet implemented";
}

char* get_extention(char* path) {
    for(int i = strlen(path); i >= 0; i--) {
        if(path[i] == '.') {
            return (char*)(path + i + 1);
            
            break;
        }
    }
    
    return NULL;
}

void change_extention(char* path, char* new_ext) {
    char* old_ext = get_extention(path);

    uint32_t diff;
    if((diff = strlen(new_ext) - strlen(old_ext)) > 0) {
        char* temp = (char*)malloc(strlen(path + diff));

        temp[0] = '\0';
        
        strcat(temp, path);

        free((void*)path);

        path = temp;
    }
    
    path[strlen(path) - strlen(old_ext)] = '\0';

    strcat(path, new_ext);
}

int equal(char* str1, char* str2) {
    int i = 0;
    while(str1[i] != 0 && str2[i] != 0) {
        if(str1[i] != str2[i]) {
            return 0;
        }
    }

    return str1[i] != str2[i];
}
