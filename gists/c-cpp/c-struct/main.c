
/*
 * c使用struct实现对成员的对外封装。
 */
#include <stdio.h>
#include "test.h"

int main(void) {
    test* t;
    init(&t);
    print_test(t);

    // 在main.c中，并不知道test中有哪些成员
    // t->a = -1;
    // printf("%d\n", t->a);
    return 0;
}
