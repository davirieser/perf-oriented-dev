
#include <stdlib.h>

#define BLOCK_SIZE 4096

struct Arena {
    void * ptr;
    void * start;
    size_t capacity;
}

Arena arena;

int malloc_init(void)  __attribute__ ((constructor));
int malloc_init(void) {
    arena = malloc();
}

int malloc(size_t size) {
  
}

