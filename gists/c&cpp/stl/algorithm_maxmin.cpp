
#include "usestl.h"
#include <vector>
#include <algorithm>

/*!
 * @brief ��ȡ��ֵ
 *
 */
void algorithm_maxmin(void)
{
    PutTitle("\nAlgorithm maxmin begin.\n");

    int arr[] = {2, 5, 1, 4, 3};
    int size = sizeof(arr) / sizeof(int);
    std::vector<int> vec(arr, arr + size);

    // ���㼫ֵ
    int max = std::max(arr[0], arr[1]);
    std::vector<int>::iterator iter = std::min_element(vec.begin(), vec.end());
    int min_ele = *iter;

    // ���
    PutTitle("max:\n");
    PutContent(max);
    PutTitle("\nmin element:\n");
    PutContent(min_ele);

    PutTitle("\nAlgorithm maxmin end.\n");
}
