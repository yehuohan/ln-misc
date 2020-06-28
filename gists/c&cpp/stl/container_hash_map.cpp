
#include "usestl.h"
#include <unordered_map>
#include <utility>

/*!
 * @brief hash����
 *
 */
void container_hash_map(void)
{
    PutTitle("\nContainer hash map begin.\n");

    int arr[] = {3, 5, 2, 3, 8, 2, 3};
    int size = sizeof(arr) / sizeof(int);
    std::unordered_map<int, int> ii;
    std::unordered_multimap<int, int> iim;

    // �����ֵ��
    // ������std::pair, std::make_pair,std::unordered_map<int, int>::value_type(),[]
    for (int k = 0; k < size; k ++)
    {
        // ��mapͳ��Ƶ��
        if (ii.count(arr[k]))
            ii[arr[k]] += 1;
        else
            ii[arr[k]] = 1;

        // ��multimapӳ���±�
        iim.insert(std::pair<int, int>(arr[k], k));
    }

    // ���
    PutTitle("unordered map count:\n");
    for (const auto& it:ii)
        PutContent("key: " << it.first << ", value(count): " << it.second << std::endl);
    PutTitle("unordered multimap index:\n");
    for (const auto& it:iim)
        PutContent("key: " << it.first << ", value(index): " << it.second << std::endl);
    PutTitle("unordered multimap count:\n");
    for (int k = 0; k < size; k ++)
        PutContent("key: " << arr[k] << ", count: " << iim.count(arr[k])<< std::endl);
    std::unordered_multimap<int, int>::iterator result = iim.find(5);
    if (result != iim.end())
        PutContent("find result: " << result->first << ' ' << result->second);

    PutTitle("\nContainer hash map end.\n");
}
