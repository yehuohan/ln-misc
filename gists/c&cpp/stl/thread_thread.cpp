
#include "usestl.h"
#include <thread>
#include <mutex>

int cnt = 0;
std::mutex mtx;

void func(int k)
{
    std::this_thread::sleep_for(std::chrono::milliseconds(10));

    mtx.lock();
    PutContent("k: " << k << "  cnt: " << cnt << std::endl);
    mtx.unlock();
    cnt ++;
}

/*!
 * @brief join��ʽ���ж��߳�
 *
 */
void thread_join(void)
{
    std::thread* th[10];
    int k = 0;
    for (k = 0; k < 10; k ++)
    {
        th[k] = new std::thread(func, k);    // ��ֵ���ݲ���k
        //th[k]->join();          // ������ǰ���̣߳�ֱ�� *this ����ʶ���߳������ִ�С�
                                // ������thread���������join��Ч������˳��ִ��
    }
    for (k = 0; k < 10; k ++)
    {
        th[k]->join();          // ������ǰ���̣߳�ֱ�� *this ����ʶ���߳������ִ�С�
                                // ������thread�����ͳһjoin��ʵ�ֶ��߳����С�
    }

    // ��Ϊ���̻߳��join���߳̽�����
    // ���Բ�������cnt�������ж϶��߳��Ƿ�ȫ��ִ����ϡ�
    //while (cnt < 10);
}

/*!
 * @brief detach��ʽ���ж��߳�
 *
 */
void thread_detach(void)
{
    std::thread* th[10];
    int k = 0;
    for (k = 0; k < 10; k ++)
    {
        th[k] = new std::thread(func, k);    // ��ֵ���ݲ���k
        th[k]->detach();        // �� thread �������ִ�е��̣߳�����ִ�ж����س�����
    }

    // ��Ϊ���̲߳���ȴ�detach���߳̽�����
    // ���Բ��ȴ�cnt�ﵽ10��˵��detach���̶߳�������ϣ���
    // �򿴲����߳�����������Ϊ���߳��Ѿ���������
    while (cnt < 10);
}


/*!
 * @brief �̴߳���
 *
 */
void thread_thread(void)
{
    PutTitle("\nThread thread begin.\n");
    //thread_join();
    thread_detach();
    PutTitle("\nThread thread end.\n");
}
