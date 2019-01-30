#include <stdio.h>
#include <stdint.h>

#define HALF_A_MIL 500000

void incr(uint32_t* target) {
    for (uint32_t i = 0; i < HALF_A_MIL; i++) {
        *target += 1;
    }
}

int main() {
    // Counter variable
    uint32_t counter = 0;
    incr(&counter);
    incr(&counter);
    printf("counter: %d\n", counter);
}
