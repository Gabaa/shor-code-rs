mod permutation;

use nalgebra::ArrayStorage;
use nalgebra::Complex;
use nalgebra::Const;
use nalgebra::DMatrix;
use nalgebra::Matrix;
use nalgebra::Matrix2;
use nalgebra::Matrix4;
use nalgebra::SquareMatrix;
use nalgebra::U2;
use num::complex::Complex64;
use num::One;
use num::Zero;

use crate::gates::permutation::make_inverse_permutation;
use crate::gates::permutation::make_permutation;
use crate::gates::permutation::number_of_qubits;
use crate::ComplexMatrix;
use crate::State;

pub trait Gate<const N: usize> {
    /// Get the matrix that represents this gate.
    fn matrix(&self) -> ComplexMatrix<N>;

    /// Apply the gate to a state of the expected size.
    fn apply(&self, state: &mut State<N>) {
        let u = self.matrix();
        state.coefficients = u * state.coefficients;
    }

    /// Apply the gate to a state that is larger than the expected size.
    /// The actual operation is performed on the first log2(N) qubits.
    /// If you want to apply the gate to any other qubits than the first log2(N),
    /// use [Gate::apply_partial_permuted].
    ///
    /// ## Panics
    ///
    /// Panics if M is less than N.
    fn apply_partial<const M: usize>(&self, state: &mut State<M>) {
        assert!(N < M, "N must be strictly less than M");
        let mut d = 1;
        let mut m = M;

        while m != N {
            m >>= 1;
            d <<= 1;
        }

        // not smart enough to convince the compiler that we can do a kronecker product to get a square matrix of dim M.
        // instead, just do it dynamically and then copy over to matrix of correct size.
        let id = DMatrix::identity(d, d);
        let u = self.matrix();
        let u2 = u.kronecker(&id);

        let u3 = ComplexMatrix::<M>::from_iterator(u2.into_iter().map(|&x| x));

        state.coefficients = u3 * state.coefficients;
    }

    /// Apply the gate to a state that is larger than the expected size.
    /// The operation is performed on the `log2(N)` qubits in `indices`.
    ///
    /// ## Panics
    ///
    /// Panics if `M < N`, or if `indices` is not `log2(N)` elements in the range `0..log2(M)`.
    fn apply_partial_permuted<const M: usize>(&self, state: &mut State<M>, indices: &[usize]) {
        // Make the ordering of the qubits where entries from `indices` come first.
        let mut new_ordering = Vec::from_iter(0..number_of_qubits(M));
        for &index in indices {
            new_ordering.remove(index);
        }
        for &index in indices.iter().rev() {
            new_ordering.insert(0, index);
        }

        let perm = make_permutation(&new_ordering);
        state.permute_qubits(&perm);

        self.apply_partial(state);

        let inv_perm = make_inverse_permutation(&perm);
        state.permute_qubits(&inv_perm);
    }
}

pub struct HadamardGate;

impl Gate<2> for HadamardGate {
    fn matrix(&self) -> ComplexMatrix<2> {
        todo!()
    }
}

pub struct BitFlipGate;

impl Gate<2> for BitFlipGate {
    fn matrix(&self) -> ComplexMatrix<2> {
        todo!()
    }
}

pub struct PhaseFlipGate;

impl Gate<2> for PhaseFlipGate {
    fn matrix(&self) -> ComplexMatrix<2> {
        todo!()
    }
}

pub struct ControlledBitFlipGate;

impl Gate<4> for ControlledBitFlipGate {
    fn matrix(&self) -> ComplexMatrix<4> {
        Matrix4::<Complex64>::from_vec(vec![
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
        ])
    }
}

// TODO: Make generic controlled-U gate.
pub struct ControlledGate<const N: usize, G: Gate<N>>(G);

impl<const N: usize, G: Gate<N>> Gate<{ N << 1 }> for ControlledGate<N, G> {
    fn matrix(&self) -> ComplexMatrix<{ N << 1 }> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(State::zero().product(State::zero()) => State::zero().product(State::zero()) ; "cnot 00 -> 00")]
    #[test_case(State::zero().product(State::one()) => State::zero().product(State::one()) ; "cnot 01 -> 01")]
    #[test_case(State::one().product(State::zero()) => State::one().product(State::one()) ; "cnot 10 -> 11")]
    #[test_case(State::one().product(State::one()) => State::one().product(State::zero()) ; "cnot 11 -> 10")]
    fn cnot_works(mut state: State<4>) -> State<4> {
        let cnot = ControlledBitFlipGate;
        cnot.apply(&mut state);
        state
    }

    #[test]
    fn cnot_on_permuted_state_works() {
        let mut state = State::zero().product(State::zero()).product(State::one());
        let expected = State::one().product(State::zero()).product(State::one());

        let cnot = ControlledBitFlipGate;
        cnot.apply_partial_permuted(&mut state, &[2, 0]);
        assert_eq!(state, expected);
    }
}
