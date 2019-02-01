#include <stdio.h>
#include <stdint.h>

int main() {

    // Initialize a 3-element arrays
    uint16_t a[3] = { 101, 110, 590 };

    // Arrays are pointers? Prove it.
    printf("a:         %p\n", a);
    printf("*a:        %d\n", *a);
    printf("*(a + 0):  %d\n", *(a + 0));
    printf("*(a + 1):  %d\n", *(a + 1));
    printf("*(a + 2):  %d\n", *(a + 2));

}
