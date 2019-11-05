
#include <stdio.h>
#include <malloc.h>

/**
 * Return an array of size *returnSize.
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* countBits(int num, int* returnSize) {
	int* rcb = (int*)malloc((num + 1)*sizeof(int));

#if(1)
    rcb[0] = 0; rcb[1] = 1;
    // k >> 1 = k / 2
    // k & 1  = k % 2
    for (int k = 2; k <= num; k ++)
        rcb[k] = rcb[k >> 1] + rcb[k & 1];
#else
	int offset = 0;
    rcb[0] = 0;
	for (int n = 1; n <= num; n++)
	{
		if (n == 1 << offset)
		{
			offset++;
            rcb[n] = 1;
		}
		else
            rcb[n] = rcb[n - (1 << (offset-1))] + 1;
	}
#endif

	*returnSize = num + 1;
	return rcb;
}

int main()
{
    int size;
    int* cb = countBits(5, &size);
    for (int k = 0; k < size; k ++)
        printf("%d  ", cb[k]);
    free(cb);
    return 0;
}
