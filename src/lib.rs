#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod gates;

use nalgebra::ArrayStorage;
use nalgebra::Const;
use nalgebra::Matrix;
use nalgebra::Vector;
use num::complex::Complex64;
use num::One;
use num::Zero;

/// A vector of complex numbers with N entries.
type ComplexVector<const N: usize> = Vector<Complex64, Const<N>, ArrayStorage<Complex64, N, 1>>;
type ComplexMatrix<const N: usize> =
    Matrix<Complex64, Const<N>, Const<N>, ArrayStorage<Complex64, N, N>>;

#[derive(Debug, PartialEq)]
/// A quantum state with log2(N) qubits (requiring N entries in complex state vector).
pub struct State<const N: usize> {
    coefficients: ComplexVector<N>,
}

impl State<2> {
    pub fn zero() -> State<2> {
        let coefficients = ComplexVector::<2>::from_vec(vec![One::one(), Zero::zero()]);
        State { coefficients }
    }

    pub fn one() -> State<2> {
        let coefficients = ComplexVector::<2>::from_vec(vec![Zero::zero(), One::one()]);
        State { coefficients }
    }
}

impl<const N: usize> State<N> {
    pub fn product<const M: usize>(self, other: State<M>) -> State<{ N * M }> {
        let mut coefficients = ComplexVector::<{ N * M }>::repeat(One::one());

        for (i, c1) in self.coefficients.iter().enumerate() {
            for (j, c2) in other.coefficients.iter().enumerate() {
                coefficients[j + M * i] = c1 * c2;
            }
        }

        State { coefficients }
    }

    pub fn permute_qubits(&mut self, permutation: &[usize]) {
        assert_eq!(permutation.len(), N);

        let mut perm_mat = ComplexMatrix::<N>::zeros();
        for (i, &j) in permutation.iter().enumerate() {
            *perm_mat.get_mut((i, j)).unwrap() = One::one();
        }

        self.coefficients = perm_mat * self.coefficients;
    }

    pub fn number_of_qubits(&self) -> usize {
        if N == 0 {
            // shouldn't be possible, but let's be thorough
            return 0;
        }

        let mut n = N;
        let mut r = 0;
        while n != 1 {
            n >>= 1;
            r += 1;
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_works() {
        let prod = State::zero().product(State::one());
        let expected = ComplexVector::<2>::from_vec(vec![
            Complex64::zero(),
            Complex64::one(),
            Complex64::zero(),
            Complex64::zero(),
        ]);

        assert_eq!(prod.coefficients.len(), 4);
        assert_eq!(prod.coefficients, expected);
    }
}
