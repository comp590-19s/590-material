#include <stdio.h>
#include <stdint.h>

int main() {

    // Initialize two 2-element arrays
    uint16_t a[2] = { 110, 110 };
    uint16_t b[2] = { 590, 590 };

    // Print their addresses and contents
    printf("&a:   %p\n", &a);
    printf("a[0]: %d\n", a[0]);
    printf("a[1]: %d\n", a[1]);
    printf("a[2]: %d\n", a[2]);

    printf("&b:   %p\n", &b);
    printf("b[0]: %d\n", b[0]);
    printf("b[1]: %d\n", b[1]);
    printf("b[2]: %d\n", b[2]);
    printf("b[-1]: %d\n", b[-1]);

}
