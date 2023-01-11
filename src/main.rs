mod codes;

use codes::*;
use qvnt::prelude::*;

fn main() {
    let mut state = QReg::with_state(9, 0b100000000);

    let before_prob = state.get_probabilities();

    let params = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    ShorCode::encode(&mut state, params);

    state.apply(&op::x(0b000001000));

    ShorCode::decode(&mut state, params);

    let prob_after = state.get_probabilities();
    assert_eq!(before_prob, prob_after);

    println!("Success!");
}
