
#include <stdio.h>

typedef struct Ext {
    int id;
    const char* name;
} Ext;

int ext_neg_square(int input) {
    return - input * input;
}

int ext_call(int(*pfunc)(int), int input) {
    return pfunc(input);
}

void ext_print(Ext* e) {
    printf("%d: %s\n", e->id, e->name);
}

#if 0
/* example */
#include <stdio.h>
int main(int argc, char *argv[]) {
    printf("%d\n", ext_call(ext_neg_square, 3));
    Ext e = {
        .id = 5,
        .name = "EXT",
    };
    ext_print(&e);
    return 0;
}
#endif
