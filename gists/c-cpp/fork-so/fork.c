
#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>
#include <errno.h>
#include <dlfcn.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <signal.h>

typedef int(*func)(int);

bool loop = true;

void signal_handler_loop(int sig) {
    if (SIGINT == sig)
        loop = false;
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

    bool is_parent = true;
    int cnt = 0;
    while (is_parent && cnt < 100) { // 只有父进程，才会执行while循环
        printf("fork cnt: %d\n", ++cnt);
        loop = true; // 子进程可能办为loop退出，需要在父进程重置loop
        pid_t pid = fork();

        // 父子进程的代码逻辑是一样的，只是pid值不同
        if (pid < 0) {
            printf("Failed to fork\n");
        } else if (pid > 0) {
            is_parent = true; // 将父进程中的is_parent置为true
            printf("This is parent process: %d\n", getpid());
            // code executed in parent process
            int status;
            waitpid(pid, &status, 0);
        } else {
            is_parent = false; // 将子进程中的is_parent置为true，防止子进程再次fork出子进程的子进程
            printf("This is child process: %d of %d\n", getpid(), getppid());
            // code executed in child process
            task();
        }
        printf("End from %s\n", is_parent ? "parent" : "child");
    }
}
