#include <stdlib.h>
#include <stdio.h>
#include "node.h"

Node* cons(int value, Node* next) {
    Node* n = (Node*)malloc(sizeof(Node));
    n->value = value;
    n->next = next;
    return n;
}

int first(Node* n) {
    if (n != NULL) {
        return n->value;
    } else {
        fprintf(stderr, "Error: cannot call first on empty (NULL) Node");
        exit(EXIT_FAILURE);
    }
}

Node* rest(Node* n) {
    if (n != NULL) {
        return n->next;
    } else {
        fprintf(stderr, "Error: cannot call rest on empty (NULL) Node");
        exit(EXIT_FAILURE);
    }
}

void destroy(Node* n) {
    Node* cursor = n;
    Node* next;
    while (cursor != NULL) {
        next = cursor->next;
        free(cursor);
        cursor = next;
    }
}
