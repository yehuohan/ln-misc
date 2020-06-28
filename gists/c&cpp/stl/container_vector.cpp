
#include "usestl.h"
#include <iostream>
#include <vector>

/*!
 * @brief ��̬����
 *
 */
void container_vector(void)
{
    PutTitle("\nContainer vector begin.\n");

    // 1. ����vector
    int arr[] = {3, 2, 1, 5, 4};
    std::vector<int> vec1(arr, arr+5);  // ����������vector
    std::vector<int> vec2(5, 0);        // ����vectorʱ����ʼ5��ֵΪ0Ԫ��

    // 2. ����Ԫ��
    vec2.front() = -1;
    vec2.at(1) = 1;
    vec2.pop_back();
    vec2.push_back(-10);

    // ������
    PutTitle("vec1:\n");
    for (int k = 0; k < 5; k ++)
        PutContent(vec1[k] << ' ');

    PutTitle("\nvec2:\n");
    for (int k = 0; k < 5; k ++)
        PutContent(vec2[k] << ' ');

    PutTitle("\nContainer vector end.\n");
}
