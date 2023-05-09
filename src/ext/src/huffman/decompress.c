#include "huffman.h"

#include "../bitarr/bitarr.h"

#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

decoding_tree* init_node();

decoding_tree* build_tree(encoding_tree tree);

char *decompress(char *path, char *dest) {
    FILE *fd, *res;
    uint64_t info_bytes, file_size, buff_offset;
    
    fd = fopen(path, "rb");
    if (!fd) {
        return strerror(errno);
    }
    
    res = fopen(dest, "wb");
    if (!res) {
        return strerror(errno);
    }

    encoding_tree tree;
    fread((void *)&tree.distinct, sizeof(uint32_t), 1, fd);
    tree.nodes = (char_info*)malloc(tree.distinct);
    for (int i = 0; i < tree.distinct; i++) {
        char_info entry = tree.nodes[i];
        // Character for the table entry
        fread((void *)&entry.orig, 1, 1, fd);
        // Length of character code
        fread((void *)&entry.code_length, 1, 1, fd);
        uint8_t len = entry.code_length / 8;
        if(len % 8) {
            len++;
        }
        // New code for character
        entry.code = (char*)malloc(len);
        fread((void *)&entry.code, len, 1, fd);
        // Fraction of original files characters equal to this one
        fread((void *)&entry.prob, sizeof(double), 1, fd);
    }

    // Size in bytes of uncompressed information
    fread((void *)&file_size, 1, sizeof(uint64_t), fd);
    // Size in bytes of compressed information
    fread((void *)&info_bytes, 1, sizeof(uint64_t), fd);
    // Size in bites of compressed information
    fread((void *)&buff_offset, 1, sizeof(uint64_t), fd);

    void *result = malloc(file_size);
    void *buffer = malloc(info_bytes);
    
    // Compresed information
    fread(buffer, 1, info_bytes, fd);

    decoding_tree *root = build_tree(tree);

    printf("1\n");
    uint16_t buff_index = 0;
    printf("2\n");
    uint8_t bit = 0;
    printf("3\n");
    for (int i= 0; i < buff_offset; i++){
        printf("Llegue\n");
        decoding_tree *aux = root;
        
        printf("Llegue\n");
        while (!aux->caract){
            move(buffer, (void*)&bit, buff_offset, 0, 1);

            if (bit) {
                aux = aux->der;
            }
            else{
                aux = aux->izq;
            }
        }

        move((void *)aux->caract, result, 0, buff_index, 8);
        buff_index += 8;
    }

    fwrite(result, 1, file_size, res);

    return NULL;
}

decoding_tree* init_node(){
    decoding_tree *node = (decoding_tree*)malloc(sizeof(decoding_tree));

    node->caract = NULL;
    node->der = NULL;
    node->izq = NULL;

    return node;
}

decoding_tree* build_tree(encoding_tree tree){
    decoding_tree *root = init_node();

    for (int i=0; i < tree.distinct; i++){
        decoding_tree *aux = root;
        char_info info = tree.nodes[i];
        uint8_t bit;
        for(int j=0; j < info.code_length; j++){
            move((void*)info.code, (void*)&bit, j, 7, 1);
            printf("length: %d, index: %d\n", info.code_length, j);

            if (bit){
                if(aux->der == NULL){
                    aux->der = init_node();
                } else {
                    printf("Codigo no instantaneo");
                }
                aux = aux->der;
            }
            else{
                if(aux->izq == NULL){
                    aux->izq = init_node();
                } else {
                    printf("Codigo no instantaneo");
                }
                aux = aux->izq;
            }
        }
        
        printf("distinct: %d, index: %d", tree.distinct, i);
        aux->caract = (char*)malloc(1);
        *aux->caract = info.orig;
    }

    return root;
}

