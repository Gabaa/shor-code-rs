use crate::gates::ControlledBitFlipGate;
use crate::gates::Gate;
use crate::gates::HadamardGate;
use crate::State;

/// Encode a qubit into the two auxillary qubits. The auxillary qubits must initially be zero.
fn encode_bit_flip<const N: usize>(state: &mut State<N>, control: usize, aux: [usize; 2]) {
    assert!(N >= 8, "must have at least 3 qubits");

    let cnot = ControlledBitFlipGate;
    cnot.apply_partial_permuted(state, &[control, aux[0]]);
    cnot.apply_partial_permuted(state, &[control, aux[1]]);
}

fn decode_bit_flip<const N: usize>(state: &mut State<N>, control: usize, aux: [usize; 2]) {
    assert!(N >= 8, "must have at least 3 qubits");

    todo!()
}

fn encode_phase_flip<const N: usize>(state: &mut State<N>, control: usize, aux: [usize; 2]) {
    assert!(N >= 8, "must have at least 3 qubits");

    let cnot = ControlledBitFlipGate;
    cnot.apply_partial_permuted(state, &[control, aux[0]]);
    cnot.apply_partial_permuted(state, &[control, aux[1]]);

    let h = HadamardGate;
    h.apply_partial_permuted(state, &[control]);
    h.apply_partial_permuted(state, &[aux[0]]);
    h.apply_partial_permuted(state, &[aux[1]]);
}

fn decode_phase_flip<const N: usize>(state: &mut State<N>, control: usize, aux: [usize; 2]) {
    assert!(N >= 8, "must have at least 3 qubits");

    todo!()
}

pub fn encode_shor<const N: usize>(state: &mut State<N>) {
    assert!(N >= 512, "must have at least 9 qubits");

    encode_phase_flip(state, 0, [3, 6]);
    for i in 0..3 {
        encode_bit_flip(state, 3 * i, [3 * i + 1, 3 * i + 2]);
    }
}

pub fn decode_shor<const N: usize>(state: &mut State<N>) {
    assert!(N >= 512, "must have at least 9 qubits");

    todo!()
}

#[cfg(test)]
mod tests {
    use num::One;
    use num::Zero;

    use crate::ComplexVector;

    use super::*;

    #[test]
    fn bit_flip_code_works_zero() {
        let mut state = State::zero().product(State::zero()).product(State::zero());

        encode_bit_flip(&mut state, 0, [1, 2]);
        decode_bit_flip(&mut state, 0, [1, 2]);

        let expected_coefficients = ComplexVector::<8>::from_vec(vec![
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
        ]);
        assert_eq!(state.coefficients, expected_coefficients)
    }

    #[test]
    fn bit_flip_code_works_one() {
        let mut state = State::one().product(State::zero()).product(State::zero());

        encode_bit_flip(&mut state, 0, [1, 2]);
        decode_bit_flip(&mut state, 0, [1, 2]);

        let expected_coefficients = ComplexVector::<8>::from_vec(vec![
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
        ]);
        assert_eq!(state.coefficients, expected_coefficients)
    }
}
