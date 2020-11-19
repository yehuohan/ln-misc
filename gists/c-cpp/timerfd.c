
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <unistd.h>
#include <sys/timerfd.h>
#include <sys/poll.h>

int main(int argc, char *argv[]) {
    uint64_t exp, tot_exp;
    ssize_t s;

    struct pollfd pfd[1];

    // 设置timer
    struct itimerspec itv;
    itv.it_value.tv_sec = 2; // 2秒后开始
    itv.it_value.tv_nsec = 0;
    itv.it_interval.tv_sec = 1; // 1秒循环
    itv.it_interval.tv_nsec = 0;

    //fd = timerfd_create(CLOCK_REALTIME, 0); // 使用系统时间（修改系统时间会影响timer）
    pfd[0].fd = timerfd_create(CLOCK_MONOTONIC, 0); // 使用单调递增时间（不受系统时间影响）
    timerfd_settime(pfd[0].fd, 0, &itv, NULL); // flags = 0，使用相对定时器，itv
    pfd[0].events = POLLIN;

    for (tot_exp = 0; tot_exp < 5;) {
        // read为阻塞读取，所以用poll实现异步机制
        poll(pfd, 1, 10);

        if (pfd[0].revents & POLLIN) {
            // s 为读取字节长度（即 s = sizeof(exp) = sizeof(uint64_t)）
            // exp为超时次数
            s = read(pfd[0].fd, &exp, sizeof(uint64_t));
            tot_exp += exp;
            printf("size: %d, exp: %llu, total=%llu  ", (int)s, exp, tot_exp);

            usleep(1000 * 500);
            timerfd_gettime(pfd[0].fd, &itv); // 距离下次定时的时间
            printf("reset: %lus, %lums, %lus, %lums\n",
                    itv.it_value.tv_sec, itv.it_value.tv_nsec / 1000000,
                    itv.it_interval.tv_sec, itv.it_interval.tv_nsec / 1000000
                    );
        }

        static int cnt = 0;
        if ((++cnt) % 10 == 0) {
            printf("cnt: %d\n", cnt);
        }
    }

    return 0;
}
