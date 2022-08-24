typedef struct vec2 {
    float x;
    float y;
} vec2;

typedef struct line2d {
    vec2 start;
    vec2 end;
} line2d;

typedef struct line2dLLN {
    line2d self;
    line2dLLN next;
} line2dLLN;

typedef struct line2dLL {
    line2dLLN* head;
    line2dLLN* tail;
}

line2dLL* newLine2dLL();
void appendLine2dLL(line2dLL* ll, line2d line);
line2d getLine2dLL(line2dLL* ll, int index);