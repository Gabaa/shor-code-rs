#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use nalgebra::ArrayStorage;
use nalgebra::Const;
use nalgebra::Vector;
use num::complex::Complex64;
use num::One;
use num::Zero;

type ComplexVector<const N: usize> = Vector<Complex64, Const<N>, ArrayStorage<Complex64, N, 1>>;

pub struct State<const N: usize> {
    coefficients: ComplexVector<N>,
}

impl State<2> {
    pub fn qubit_zero() -> State<2> {
        let coefficients =
            ComplexVector::<2>::from_iterator([One::one(), Zero::zero()].into_iter());
        State { coefficients }
    }

    pub fn qubit_one() -> State<2> {
        let coefficients =
            ComplexVector::<2>::from_iterator([Zero::zero(), One::one()].into_iter());
        State { coefficients }
    }
}

impl<const N: usize> State<N> {
    pub fn product<const M: usize>(self, other: State<M>) -> State<{ N * M }> {
        let mut coefficients = ComplexVector::<{ N * M }>::from_element(One::one());

        for (i, c1) in self.coefficients.iter().enumerate() {
            for (j, c2) in other.coefficients.iter().enumerate() {
                coefficients[j + M * i] = c1 * c2;
            }
        }

        State { coefficients }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_works() {
        let zero = State::qubit_zero();
        let one = State::qubit_one();
        let prod = zero.product(one);

        assert_eq!(prod.coefficients.len(), 4);

        assert_eq!(prod.coefficients[0], Complex64::zero());
        assert_eq!(prod.coefficients[1], Complex64::one());
        assert_eq!(prod.coefficients[2], Complex64::zero());
        assert_eq!(prod.coefficients[3], Complex64::zero());
    }
}
