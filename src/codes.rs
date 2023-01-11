use qvnt::prelude::*;

pub trait QuantumErrorCorrectingCode {
    type CodeParams;

    fn encode(state: &mut QReg, params: Self::CodeParams);
    fn decode(state: &mut QReg, params: Self::CodeParams);
}

pub struct BitFlipCode;
impl QuantumErrorCorrectingCode for BitFlipCode {
    type CodeParams = [usize; 3];

    fn encode(state: &mut QReg, params: Self::CodeParams) {
        let v = state.get_vreg();

        state.apply(&op::x(v[params[1]]).c(v[params[0]]).unwrap());
        state.apply(&op::x(v[params[2]]).c(v[params[0]]).unwrap());
    }

    fn decode(state: &mut QReg, params: Self::CodeParams) {
        let v = state.get_vreg();

        state.apply(&op::x(v[params[1]]).c(v[params[0]]).unwrap());
        state.apply(&op::x(v[params[2]]).c(v[params[0]]).unwrap());

        state.apply(&op::x(v[params[0]]).c(v[[params[1], params[2]]]).unwrap());
    }
}

pub struct PhaseFlipCode;
impl QuantumErrorCorrectingCode for PhaseFlipCode {
    type CodeParams = [usize; 3];

    fn encode(state: &mut QReg, params: Self::CodeParams) {
        let v = state.get_vreg();

        BitFlipCode::encode(state, params);
        state.apply(&op::h(v[params]));
    }

    fn decode(state: &mut QReg, params: Self::CodeParams) {
        let v = state.get_vreg();

        state.apply(&op::h(v[params]));
        BitFlipCode::decode(state, params);
    }
}

pub struct ShorCode;
impl QuantumErrorCorrectingCode for ShorCode {
    type CodeParams = [usize; 9];

    fn encode(state: &mut QReg, params: Self::CodeParams) {
        let phase_flip_indices = [params[0], params[3], params[6]];
        PhaseFlipCode::encode(state, phase_flip_indices);

        for i in 0..3 {
            let bit_flip_indices = [params[i * 3], params[i * 3 + 1], params[i * 3 + 2]];
            BitFlipCode::encode(state, bit_flip_indices);
        }
    }

    fn decode(state: &mut QReg, params: Self::CodeParams) {
        for i in 0..3 {
            let bit_flip_indices = [params[i * 3], params[i * 3 + 1], params[i * 3 + 2]];
            BitFlipCode::decode(state, bit_flip_indices);
        }

        let phase_flip_indices = [params[0], params[3], params[6]];
        PhaseFlipCode::decode(state, phase_flip_indices);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_flip_no_error() {
        let mut state = QReg::with_state(3, 0b100);

        BitFlipCode::encode(&mut state, [0, 1, 2]);
        BitFlipCode::decode(&mut state, [0, 1, 2]);

        assert_eq!(state.measure_mask(0b100).get(), 0b100);
    }

    #[test]
    fn phase_flip_no_error() {
        let mut state = QReg::with_state(3, 0b100);

        PhaseFlipCode::encode(&mut state, [0, 1, 2]);
        PhaseFlipCode::decode(&mut state, [0, 1, 2]);

        assert_eq!(state.measure_mask(0b100).get(), 0b100);
    }

    #[test]
    fn shor_no_error() {
        let mut state = QReg::with_state(9, 0b100000000);

        ShorCode::encode(&mut state, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
        ShorCode::decode(&mut state, [0, 1, 2, 3, 4, 5, 6, 7, 8]);

        assert_eq!(state.measure_mask(0b100).get(), 0b100000000);
    }
}
