#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

int main() {

    // malloc reserves space on heap for a value
    // and gives back the address of the space.
    uint32_t* a = malloc(sizeof(uint32_t));
    *a = 110;
    printf("a: %d\n", *a);
    free(a); // releases the reserved memory 

    uint32_t* b = malloc(sizeof(uint32_t));
    *b = 590;
    printf("a: %d\n", *a);
    printf("b: %d\n", *b);
    free(b);

}
