
#include "usestl.h"
#include <list>

/*!
 * @brief ˫������
 *
 */
void container_list(void)
{
    PutTitle("\nContainer list begin.\n");

    std::list<int> li;

    // ����Ԫ��
    li.push_front(10);
    li.push_back(3);
    li.push_back(1);
    li.push_back(5);
    li.push_back(-1);

    // ���
    PutTitle("list:\n");
    for (auto it:li)
        PutContent(it);
    li.sort();
    PutTitle("\nlist sort:\n");
    for (auto it:li)
        PutContent(it);

    PutTitle("\nContainer list end.\n");
}
