use qvnt::prelude::*;

fn encode_shor(state: &mut QReg) {
    state.apply(&op::x(0b000100000).c(0b100000000).unwrap());
    state.apply(&op::x(0b000000100).c(0b100000000).unwrap());

    state.apply(&op::h(0b100100100));

    state.apply(&op::x(0b010000000).c(0b100000000).unwrap());
    state.apply(&op::x(0b001000000).c(0b100000000).unwrap());
    state.apply(&op::x(0b000010000).c(0b000100000).unwrap());
    state.apply(&op::x(0b000001000).c(0b000100000).unwrap());
    state.apply(&op::x(0b000000010).c(0b000000100).unwrap());
    state.apply(&op::x(0b000000001).c(0b000000100).unwrap());
}

fn decode_shor(state: &mut QReg) {
    state.apply(&op::x(0b010000000).c(0b100000000).unwrap());
    state.apply(&op::x(0b001000000).c(0b100000000).unwrap());
    state.apply(&op::x(0b000010000).c(0b000100000).unwrap());
    state.apply(&op::x(0b000001000).c(0b000100000).unwrap());
    state.apply(&op::x(0b000000010).c(0b000000100).unwrap());
    state.apply(&op::x(0b000000001).c(0b000000100).unwrap());

    state.apply(&op::x(0b100000000).c(0b011000000).unwrap());
    state.apply(&op::x(0b000100000).c(0b000011000).unwrap());
    state.apply(&op::x(0b000000100).c(0b000000011).unwrap());

    state.apply(&op::h(0b100100100));

    state.apply(&op::x(0b100000000).c(0b000100000).unwrap());
    state.apply(&op::x(0b100000000).c(0b000000100).unwrap());

    state.apply(&op::x(0b100000000).c(0b000100100).unwrap());
}

fn main() {
    let mut state = QReg::with_state(9, 0b100000000);

    let c = state.measure_mask(0b100000000);
    let before_val = c.get();

    encode_shor(&mut state);

    state.apply(&op::x(0b000001000));

    decode_shor(&mut state);

    let c = state.measure_mask(0b100000000);
    let after_val = c.get();
    assert_eq!(after_val, before_val)
}
