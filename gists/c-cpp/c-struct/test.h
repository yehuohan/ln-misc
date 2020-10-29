#include <stdio.h>

// test.h中只提供结构体名称test，和用于test访问接口
typedef struct _test test;

void init(test** t);
void print_test(test* t);
