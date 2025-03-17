use crate::bindings;

pub fn qr(m: usize, n: usize, a: &mut [f64]) -> Vec<f64> {
    let mut tau = vec![0.0; n];
    unsafe { bindings::qr(m, n, a.as_mut_ptr(), tau.as_mut_ptr()) }
    tau
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let m = 3;
        let n = 2;
        let mut a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let tau = qr(m, n, &mut a);
        assert_eq!(a, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert_eq!(tau, vec![0.0, 0.0]);
    }
}
