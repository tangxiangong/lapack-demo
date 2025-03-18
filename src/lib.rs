// 显式引用手动创建的绑定
mod bindings;
pub use bindings::*;

// 添加一个简单的测试函数
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lapack_dgesv() {
        // 创建一个简单的线性方程组 Ax = b
        // A = [[1, 2], [3, 4]]
        // b = [5, 6]
        // 预期结果 x = [-4, 4.5]

        let n: i32 = 2;
        let nrhs: i32 = 1;
        let mut a: Vec<f64> = vec![1.0, 3.0, 2.0, 4.0]; // 列主序
        let mut ipiv: Vec<i32> = vec![0; n as usize];
        let mut b: Vec<f64> = vec![5.0, 6.0];
        let lda: i32 = n;
        let ldb: i32 = n;

        unsafe {
            LAPACKE_dgesv(
                LAPACK_COL_MAJOR as i32,
                n,
                nrhs,
                a.as_mut_ptr(),
                lda,
                ipiv.as_mut_ptr(),
                b.as_mut_ptr(),
                ldb,
            );
        }

        // 检查结果
        assert!((b[0] + 4.0).abs() < 1e-10);
        assert!((b[1] - 4.5).abs() < 1e-10);
    }
}
