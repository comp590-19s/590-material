typedef struct Node {
    int value;
    struct Node* next;
} Node;

Node* cons(int value, Node* next);
int first(Node* next);
Node* rest(Node* next);
void destroy(Node* next);
