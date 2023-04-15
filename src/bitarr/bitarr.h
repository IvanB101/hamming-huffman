#ifndef BITARR
#define BITARR

/**
 * Set to 1 bit position, counting from address pointed by arr
 * @param arr pointer to data
 * @param position of the bit to set to 1
*/
void set_bit(void *arr, int position);

/**
 * Reset to 0 bit position, counting from address pointed by arr
 * @param arr pointer to data
 * @param position of the bit to reset to 0
*/
void reset_bit(void *arr, int position);

/**
 * Flips bit position, counting from address pointed by arr
 * @param arr pointer to data
 * @param position of the bit to flip
*/
void flip_bit(void *arr, int position);

/**
 * Extends binary operation and for any structure, seen as an array of bytes
 * and puts the result in addresses corresponding to the first operand
 * @param arr1 first operand and recipient of the result of the operation
 * @param arr2 second operand
 * @param size1 size of arr1, in bytes
 *
 * If arr2 is bigger than arr1, the operation takes place with the size bytes
 * of arr2, starting at the address pointed by arr2
 *
 * If arr1 is bigger than arr2, arr1 and arr2 overlap or size is bigger than
 * then actual size of arr1, behavior is undefined
*/
void bit_and(void* arr1, void* arr2, int size1);

/**
 * Extends binary operation or for any structure, seen as an array of bytes
 * and puts the result in addresses corresponding to the first operand
 * @param arr1 first operand and recipient of the result of the operation
 * @param arr2 second operand
 * @param size1 size of arr1, in bytes
 *
 * If arr2 is bigger than arr1, the operation takes place with the size bytes
 * of arr2, starting at the address pointed by arr2
 *
 * If arr1 is bigger than arr2, arr1 and arr2 overlap or size is bigger than
 * then actual size of arr1, behavior is undefined
*/
void bit_or(void* arr1, void* arr2, int size1);

/**
 * Extends binary operation not for any structure, seen as an array of bytes
 * and puts the result in addresses corresponding to the operand
 * @param arr operand and recipient of the result of the operation
 * @param size size of arr, in bytes
 *
 * If arr2 is bigger than arr1, the operation takes place with the size bytes
 * of arr2, starting at the address pointed by arr2
 *
 * If size is bigger than the actual size of arr, behavior is undefined
*/
void bit_not(void* arr, int size);

/**
 * Copies size bits, starting in start_from from from to to, starting at start to,
 * overwriting contents in to
 * @param from structure from which bits are copied
 * @param to structure to which bits are copied
 * @start_from starting bit, counting from address pointed by from, from which
 * bits are copied
 * @start_to starting bit, counting from address pointed by to, to which
 * bits are copied
 * @size amount of bits copied form one structure to another
*/
void move(void* from, void* to, int start_from, int start_to, int size);

/**
 * Return a string with representing size bits, starting at the address pointed
 * by arr
 * @param arr pointer to data
 * @param size the size of the structure, in bytes
 *
 * If size is bigger than the actual size of arr, behavior is undefined
*/
char* to_bit_string(void* arr, int size);

#endif 
