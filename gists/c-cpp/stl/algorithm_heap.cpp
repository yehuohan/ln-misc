
#include "usestl.h"
#include <vector>
#include <algorithm>
#include <cstdlib>

/*!
 * @brief ����make_heap�ıȽϺ���
 *
 */
int cmp_heap(int a, int b)
{
    // ����
    return (a < b);
    // ��С��
    //return (a < b);
}

/*!
 * @brief �����㷨
 *
 */
void algorithm_heap(void)
{
    PutTitle("\nAlgorithm heap cegin.\n");

    int arr[] = {2, 5, 1, 4, 3};
    int size = sizeof(arr) / sizeof(int);
    std::vector<int> vec(arr, arr + size);

    // �����ѣ����ṩcmp_heapʱ��Ĭ��Ϊ����
    std::make_heap(vec.begin(), vec.end(), cmp_heap);

    // ����ѣ���ԭ��0~n-1������һ���ѣ�����n���������
    vec.push_back(9);
    std::push_heap(vec.begin(), vec.end(), cmp_heap);

    // �����ѣ���0���Ѷ��ŵ�����ֽ�0~n-1����һ����
    std::pop_heap(vec.begin(), vec.end(), cmp_heap);

    // ��������sort_heapǰ����Ҫmake_heap
    //std::sort_heap(vec.begin(), vec.end());

    // ���
    PutTitle("arr:\n");
    for (int k = 0; k < size; k ++)
        PutContent(arr[k] << ' ');
    PutTitle("\nheap:\n");
    for (int k = 0; k < vec.size(); k ++)
        PutContent(vec[k] << ' ');

    PutTitle("\nAlgorithm heap end.\n");
}
