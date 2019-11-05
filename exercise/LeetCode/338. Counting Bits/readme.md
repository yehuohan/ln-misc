## Counting Bits
Given a non negative integer number num. For every numbers i in the range 0 �� i �� num calculate the number of 1's in their binary representation and return them as an array.

 - Example:

For num = 5 you should return [0,1,1,2,1,2].

 - Follow up:

It is very easy to come up with a solution with run time O(n*sizeof(integer)). But can you do it in linear time O(n) /possibly in a single pass?
Space complexity should be O(n).
Can you do it like a boss? Do it without using any builtin function like \_\_builtin\_popcount in c++ or in any other language.

 - Hint:

You should make use of what you have produced already.
Divide the numbers in ranges like [2-3], [4-7], [8-15] and so on. And try to generate new range from previous.
Or does the odd/even status of the number help you in calculating the number of 1s?

## Code

```
rcb[0] = 0;
rcb[1] = 1;
rcb[k] = rcb[k >> 1] + rcb[k & 1];

��k >> 1�����:
  ������һλ��'1'�ĸ������䣻
  ������һλ����2�����յĽ������0����1������Ҫ�ȸ�rcb[0]��rcb[1]��ֵ��

��k & 1����⣺
  ������һλ������͵�һλ�������������������'1'����Ҫ���ϣ�
```
