#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <time.h>

int main() {

    // Seed a random number generator
    srandom(time(0));

    // Request memory on the heap, assign random
    // value. Never free the memory. Loop... infinite.
    for (;;) {
        uint32_t* ptr = malloc(1000 * sizeof(uint32_t));
        *ptr = (random() % 100) + 100;
        printf("%d\n", *ptr);
    }

}
