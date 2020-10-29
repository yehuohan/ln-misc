
#include "usestl.h"
#include <unordered_set>


/*!
 * @brief hash����
 *
 */
void container_hash_set(void)
{
    PutTitle("\nContainer hash set begin.\n");

    int arr[] = {3, 5, 2, 3, 8, 2, 3};
    int size = sizeof(arr) / sizeof(int);
    std::unordered_set<int> is;
    std::unordered_multiset<int> ism;

    // ����Ԫ�ص�����
    for (int k = 0; k < size; k ++)
    {
        is.insert(arr[k]);
        ism.insert(arr[k]);
    }

    // ����Ԫ��
    auto result = ism.find(8);
    int result_cnt = ism.count(9);

    // ���
    PutTitle("unordered set:\n");
    for (const auto& it:is)
        PutContent(it << ' ');
    PutTitle("\nunordered multiset:\n");
    for (const auto& it:ism)
        PutContent(it << ' ');
    PutTitle("\nfind result:\n");
    if (result != ism.end())
        PutContent("found: " << *result);
    else
        PutContent("not found");
    PutTitle("\ncount result:\n");
    PutContent("count: " << result_cnt);

    PutTitle("\nContainer hash set end.\n");
}
