use std::ops::MulAssign;

use nalgebra::Matrix4;
use nalgebra::Unit;
use nalgebra::Vector4;
use num::complex::Complex64;
use num::One;
use num::Zero;

use crate::State;

pub fn hadamard(state: &mut State<2>) {
    todo!()
}

pub fn x(state: &mut State<2>) {
    todo!()
}

pub fn z(state: &mut State<2>) {
    todo!()
}

pub fn cnot(state: &mut State<4>) {
    let u = Matrix4::<Complex64>::from_iterator(
        [
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
        ]
        .into_iter(),
    );

    state.coefficients = u * state.coefficients;
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(State::qubit_zero().product(State::qubit_zero()), State::qubit_zero().product(State::qubit_zero()) ; "cnot 00 -> 00")]
    #[test_case(State::qubit_zero().product(State::qubit_one()), State::qubit_zero().product(State::qubit_one()) ; "cnot 01 -> 01")]
    #[test_case(State::qubit_one().product(State::qubit_zero()), State::qubit_one().product(State::qubit_one()) ; "cnot 10 -> 11")]
    #[test_case(State::qubit_one().product(State::qubit_one()), State::qubit_one().product(State::qubit_zero()) ; "cnot 11 -> 10")]
    fn cnot_works(mut state: State<4>, expected: State<4>) {
        cnot(&mut state);
        assert_eq!(state, expected);
    }
}
