#include <stdio.h>
#include <stdint.h>
#include <pthread.h>

#define HALF_A_MIL 500000

// This function is actually given a uint32_t*, but c's type
// system and the thread library forces a void pointer that
// gets typecast to the correct type inside the function.
void* incr(void *input) {
    uint32_t* target = (uint32_t*) input;
    for (uint32_t i = 0; i < HALF_A_MIL; i++) {
        *target += 1;
    }
}

int main() {
    // We'll use this counter variable.
    uint32_t counter = 0;
    
    // Let's use 2 threads to parallelize the work of
    // incr and spread the load across multiple CPU cores.
    
    // So that we can later wait on threads to complete
    // we need two thread ids.
    pthread_t t1, t2;

    // Create two threads. Important args are:
    // 1. Reference to the thread id (&t1, &t2)
    // 3. Name of function the thread will call (incr)
    // 4. Argument value to send to the function (&counter)
    pthread_create(&t1, NULL, incr, &counter);
    pthread_create(&t2, NULL, incr, &counter);

    // "Join" waits for threads to complete work.
    pthread_join(t1, NULL);
    pthread_join(t2, NULL);

    printf("counter: %d\n", counter);
}
