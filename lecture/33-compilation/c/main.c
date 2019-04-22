#include <stdio.h>
#include <stdlib.h>

#include "node.h"

void print_list(Node* cursor);
int sum(Node* cursor);

int main() {
    printf("Linked Lists!\n");

    Node* list = cons(1, cons(2, cons(3, cons(4, NULL))));
    print_list(list);

    printf("Sum: %d\n", sum(list));

    return EXIT_SUCCESS;
}

void print_list(Node* cursor) {
    while (cursor != NULL) {
        printf("%d -> ", first(cursor));
        cursor = rest(cursor);
    }
    printf("NULL");
    printf("\n");
}

int sum(Node* cursor) {
    int sum = 0;
    while (cursor != NULL) {
        sum += first(cursor);
        cursor = rest(cursor);
    }
    return sum;
}
