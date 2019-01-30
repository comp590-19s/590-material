#include <stdio.h>
#include <stdint.h>

typedef struct Point {
    uint32_t x;
    uint32_t y;
} Point;

Point* point_factory(uint32_t x, uint32_t y) {
    Point p = { x, y };
    return &p;
}

int main() {
    printf("One Point struct, right away!");
    Point* a = point_factory(1.0, 0.0);
    printf("x: %d\n", a->x);
}
