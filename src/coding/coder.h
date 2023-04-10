#ifndef CODER
#define CODER

#define SUCCESS 0
#define DOUBLE_ERROR -1
#define IO_ERROR -2

#include <fstream>

class Coder {
    private:
        int block_size;
        int control_bits;
    public:
        int encode(std::fstream to_protect);
        int decode(std::fstream to_protect, bool correct);
};

#endif 
