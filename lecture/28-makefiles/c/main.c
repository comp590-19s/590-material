#include <stdio.h>

#include <helpers.h>

char format[] = "f(%d) = %d\n";

int main(int argc, char const *argv[]) {
    int i = 7;
    int j;

    j = f(i);

    printf(format, i, j);

    return 0;
}
