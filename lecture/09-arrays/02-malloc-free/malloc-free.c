#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

int main() {

    printf("======== stack space  ========\n");
    uint32_t stack_array[2] = { 101, 110 };
    printf("stack_array:   %p\n", stack_array);
    printf("stack_array[0]:%d\n", stack_array[0]);
    printf("stack_array[1]:%d\n", stack_array[1]);

    // calloc reserves space on heap for based
    // on N values * M sizeof a value. It returns
    // a pointer to the starting address of the
    // memory allocation. The memory is zeroed.
    printf("======== calloc heap space ========\n");
    uint32_t* heap_array = calloc(2, sizeof(uint32_t));
    heap_array[1] = 590;
    printf("heap_array:    %p\n", heap_array);
    printf("heap_array[0]: %d\n", heap_array[0]);
    printf("heap_array[1]: %d\n", heap_array[1]);

    // Release the allocated heap memory
    free(heap_array); 

}
