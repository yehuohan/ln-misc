
#include <stdio.h>
#include <stdbool.h>
#include <pthread.h>
#include <signal.h>
#include <termios.h>
#include <unistd.h>

pthread_t pth;
bool loop = true;

void signal_handler(int sig) {
    if (SIGINT == sig)
        loop = false;
}

static bool on_key(char ch) {
    switch (ch) {
        case 'i':
            printf("'i' for Input");
            break;
        default:
            printf(
"Usage:\n"
"    'q': quit\n"
"    'i': for input\n"
);
    }
    return (ch != 'q');
}

static void* pth_loop(void* arg) {
    bool (*cb)(char) = arg;

    static struct termios oldt, newt;
    tcgetattr(STDIN_FILENO, &oldt);
    newt = oldt;
    newt.c_lflag &= ~(ICANON | ECHO); // '~ICANON': 不缓存输入， '~ECHO':不回显示输入
    tcsetattr(STDIN_FILENO, TCSANOW, &newt); // 设置终端属性

    while (cb((char)getchar()) && loop);
    loop = false;

    tcsetattr(STDIN_FILENO, TCSANOW, &oldt); // 恢复终端属性

    return 0;
}

int main(void) {
    signal(SIGINT, signal_handler);

    if (0 != pthread_create(&pth, (const pthread_attr_t*)NULL, pth_loop, on_key)) {
        return -1;
    }

    while (loop) {
        usleep(1000*100);
    }

    pthread_join(pth, NULL);

    return 0;
}
