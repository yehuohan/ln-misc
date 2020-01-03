
#include <stdio.h>
#include <errno.h>
#include <stdint.h>
#include <dlfcn.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <signal.h>

typedef int(*func)(int);

int loop = 1;

void signal_handler_loop(int sig) {
    if (SIGINT == sig)
        loop = 0;
}

int task() {
    char* error;
    void* handle = NULL;

    // 加载so文件
    if (access("so.so", F_OK | R_OK | X_OK) != -1) {
        handle = dlopen("so.so", RTLD_LAZY);
    }
    if (!handle) {
        printf("open so.so failed.\n");
        return -1;
    }

    // 获取so接口
    func calc = (func)dlsym(handle, "calc");
    if ((error = dlerror()) != NULL) {
        printf("no calc: %s\n", error);
        return -1;
    }

    while (loop) {
        printf("%d\n", calc(99));
        usleep(100*1000);
    }

    /*return 0;*/
    return -1;
}

int main() {
    signal(SIGINT, signal_handler_loop);

    int cnt = 0;
    while (1) {
        printf("fork cnt: %d\n", ++cnt);
        loop = 1;
        pid_t pid;
        pid = fork();

        if (pid < 0) {
            printf("Failed to fork\n");
        } else if (pid > 0) {
            printf("This is parent process: %d\n", getpid());
            // code executed in parent process
            int status;
            waitpid(pid, &status, 0);
        } else {
            printf("This is child process: %d of %d\n", getpid(), getppid());
            // code executed in child process
            task();
        }
    }
}
