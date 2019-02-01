#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

int main() {

    // malloc reserves space on heap for a value
    // and gives back the address of the space.
    printf("======== malloc a========\n");
    uint32_t* a = malloc(2 * sizeof(uint32_t));
    a[0] = 110;
    a[1] = 110;
    printf("a[0]: %d\n", a[0]);
    printf("a[1]: %d\n", a[1]);
    printf("&a: %p\n", &a);
    free(a); // releases the reserved memory 

    // malloc reserves more space on the heap
    // for a value
    printf("======== malloc b ========\n");
    uint32_t* b = malloc(2 * sizeof(uint32_t));
    b[0] = 590;
    b[1] = 590;
    printf("b[0]: %d\n", *b);
    printf("b[1]: %d\n", *b);
    printf("&b: %p\n", &b);

    // What happens if we read a now?
    printf("a[0]: %d\n", *a);
    printf("a[1]: %d\n", *a);

    free(b);

}
