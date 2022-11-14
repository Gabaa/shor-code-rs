use nalgebra::Matrix4;
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

    #[test_case(State::zero().product(State::zero()), State::zero().product(State::zero()) ; "cnot 00 -> 00")]
    #[test_case(State::zero().product(State::one()), State::zero().product(State::one()) ; "cnot 01 -> 01")]
    #[test_case(State::one().product(State::zero()), State::one().product(State::one()) ; "cnot 10 -> 11")]
    #[test_case(State::one().product(State::one()), State::one().product(State::zero()) ; "cnot 11 -> 10")]
    fn cnot_works(mut state: State<4>, expected: State<4>) {
        cnot(&mut state);
        assert_eq!(state, expected);
    }
}
