#include "lined2dLL.h"

line2dLL* newLine2dLL() {
    line2dLL* ll = malloc(sizeof(line2dLL));
    ll->head = NULL;
    ll->tail = NULL;
    return ll;
}

void appendLine2dLL(line2dLL* ll, line2d line) {
    line2dLLN* node = malloc(sizeof(line2dLLN));
    node->self = line;
    node->next = NULL;

    if (ll->tail == NULL) {
        ll->head = node;
        ll->tail = node;
    } else {
        ll->tail->next = node;
        ll->tail = node;
    }
}

line2d getLine2dLL(line2dLL* ll, int index) {
    int i = 0;
    line2dLLN* curNode = ll->head;
    while(true) {
        if (i == index) {
            return curNode->self;
        } else if (curNode == ll->tail) {
            return NULL;
        }
        i++;
    }
}