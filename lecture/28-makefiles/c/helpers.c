#include <helpers.h>

int f(int x) {
    if (x <= 1) {
        return x;
    } else {
        return x * f(x - 1);
    }
}
