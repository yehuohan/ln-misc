
// build: gcc test_timer.c -lrt

#include <stdio.h>
#include <signal.h>
#include <string.h>
#include <time.h>
#include <sys/time.h>

static int cnt = 0;

void timer_handler(union sigval arg) {
    cnt ++;
    printf("timeout: %d, %d, %p\n", cnt, arg.sival_int, arg.sival_ptr);
}

int main(int argc, char *argv[]) {
    struct sigevent evp;
    struct itimerspec itv;
    timer_t timer;
    int ret;

    // 设置timer
    memset(&evp, 0x00, sizeof(struct sigevent));
    evp.sigev_value.sival_ptr = &timer;
    //evp.sigev_notify = SIGEV_SIGNAL; // 定时器将产生信号给进程
    evp.sigev_notify = SIGEV_THREAD; // 定时器将启动新线程
    evp.sigev_value.sival_ptr = &timer;
    evp.sigev_notify_function = timer_handler;
    evp.sigev_notify_attributes = NULL;

    // 创建timer
    ret = timer_create(
            CLOCK_PROCESS_CPUTIME_ID, // 以进程运行时间计时
            &evp,
            &timer);
    printf("timer: %p\n", &timer);

    // 启动timer
    itv.it_value.tv_sec = 2; // 第1次定时时间
    itv.it_value.tv_nsec = 0;
    itv.it_interval.tv_sec = 1; // 定时间隔
    itv.it_interval.tv_nsec = 0;
    ret = timer_settime(
            timer,
            0, // flags = 0，使用相对定时器
            &itv,
            NULL);

    while (cnt < 5);

    return 0;
}
