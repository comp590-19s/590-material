#include <stdio.h>
#include <stdint.h>

int main() {
    // Establish an unsigned 16 bit integer
    uint16_t a = 0;
    printf("a: %d\n", a);

    // Establish a pointer to a
    uint16_t* a_ptr = &a;
    printf("a_ptr: %p\n", a_ptr);
    printf("*a_ptr: %d\n", *a_ptr);
    *a_ptr = 1;

    // Did a actually change?
    printf("a: %d\n", a);
}
