
#include <iostream>
#include <cstdio>
#include <cstdint>
#include <ostream>

/*
用3位十进制定点表示浮点：
    0.379 = 0.379 * 10^3 * 10^-3
          = 379 * 10^-3
    0.379 * 0.268 = 379 * 268 * 10^-6
                  = 379 * (3*10^2 + 6*10^1 + 8*10^0) * 10^-6
用3位二进制定点表示浮点：
    0.379 = 0.379 * 2^3 * 2^-3
          = 0b0011 * 2^-3
          = 0x3 * 2^-3
    0.379 * 0.268 = 0b0011 * 0b0010 * 2^-6
                  = 0b0011 * (0*2^3 + 0*2^2 + 1*2^1 + 0*2^0) * 2^-6

N=4位定点计算浮点，最高位为符号位；
使用有符号定点，无需要自己处理负数。

bit 3       2       1       0       range                            precision
Qn                                  [-2^(N-1-n), (2^(N-1)-1)/(2^n)]  1/(2^n)
Q3  (-1)   .(1/2)   (1/4)   (1/8)   [-1, 7/8]
Q2  (-2)    (1)    .(1/2)   (1/4)   [-2, 7/4]
Q1  (-4)    (2)     (1)    .(1/2)   [-4, 7/2]
Q0  (-8)    (4)     (2)     (1).    [-8, 7]
 */

#define N           24          // 1~30
#define Q(v)        (_Q<N>(v))
#define QN(v)       _QN((v), N)
#define QF(v)       _QF((v), N)
#define QMul(a, b)  _QMul((a), (b), N)

int32_t _QN(double val, int n) {
    return (int32_t)(val * (1 << n));
}

double _QF(int32_t val, int n) {
    return (double)(val / (double)(1 << n));
}

int32_t _QMul(int32_t a, int32_t b, int n) {
    return (int32_t)(((int64_t)a * (int64_t)b) >> n);
}

template<int n = 15> class _Q {
public:
    int32_t m;

    _Q(int32_t val) {this->m = val;}
    _Q(double val) {this->m = _QN(val, n);}

    double operator*() {
        return _QF(this->m, n);
    }

    friend std::ostream& operator<<(std::ostream& out, const _Q<n>& rhs) {
        out << rhs.m << ", " << _QF(rhs.m, n);
        return out;
    }

    _Q operator+(const _Q rhs) {
        return _Q<n>(this->m + rhs.m);
    }

    _Q operator-(const _Q rhs) {
        return _Q<n>(this->m - rhs.m);
    }

    _Q operator*(const _Q rhs) {
        return _Q<n>(_QMul(this->m, rhs.m, n));
    }
};

int main() {
    double a = 0.379;
    double b = -0.218;

    //std::cout << _Q<3>(a) << std::endl;
    //printf("%d, %f\n", _Q<3>(a).m, *_Q<3>(a));
    //std::cout << Q(a) << std::endl;
    //printf("%d, %f\n", Q(a).m, *Q(a));

    printf("%f\n", a + b);
    printf("%f\n", a - b);
    printf("%f\n", a * b);
    printf("%f %f\n", *(Q(a) + Q(b)), QF(QN(a) + QN(b)));
    printf("%f %f\n", *(Q(a) - Q(b)), QF(QN(a) - QN(b)));
    printf("%f %f\n", *(Q(a) * Q(b)), QF(QMul(QN(a), QN(b))));

    return 0;
}
