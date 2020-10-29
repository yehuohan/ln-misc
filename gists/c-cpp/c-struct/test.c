#include <stdio.h>
#include <malloc.h>
#include "test.h"

// 在test.c中，test才定义为_test的类型，才能访问结构体_test中的变量
struct _test {
    int a;
};

void init(test** t) {
    *t = (test*)malloc(sizeof(test));
    (*t)->a = 100;
}

void print_test(test* t) {
    printf("%d\n", t->a);
}
