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

pub fn cnot<const N: usize>(state: &mut State<N>, control: usize, target: usize) {
    let u = Matrix4::<Complex64>::from_vec(vec![
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
    ]);

    // TODO: figure out how to create a larger CNOT matrix that only affects the relevant
    // coefficients in the state.
    // You can do tensor product of matrices (e.g. I otimes X does nothing to first qubit and flips
    // second). Will this require us to reorder some of the entries in the state vector?
    // 16/11: YES! Just need to create "permutation matrix" that reorders the entries in the state
    // vector, and then apply that before and apply the inverse after.
    // (a_1, a_2, a_3, ...)^T
    // a_1 |00000> + a_2 |00001> + a_3 + |00010> + ...
    // a_i |00000> + a_j |
    //
    // Swap b1 and b2.
    // a_1 |00> + a_2 |01> + a_3 |10> + a_4 |11>
    // a_1 |00> + a_3 |01> + a_2 |10> + a_4 |11>
    //
    // Swap b1 and b3.
    // a_1 |000> + a_2 |001> + a_3 |010> + a_4 |011> + a_5 |100> + a_6 |101> + a_7 |110> + a_8 |111>
    // a_1 |000> + a_2 |001> + a_3 |010> + a_4 |011> + a_5 |100> + a_6 |101> + a_7 |110> + a_8 |111>
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
