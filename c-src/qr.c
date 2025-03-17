#include "qr.h"
#include "lapacke.h"

// 使用extern "C"确保C++编译器不会修改函数名
#ifdef __cplusplus
extern "C" {
#endif

// 原始函数实现
void qr_double(size_t m, size_t n, double *a, double *tau) {
    LAPACKE_dgeqr2(LAPACK_COL_MAJOR, m, n, a, m, tau);
}

// 为了兼容性，保留原始函数名
void qr(size_t m, size_t n, double *a, double *tau) {
    qr_double(m, n, a, tau);
}

#ifdef __cplusplus
}
#endif
