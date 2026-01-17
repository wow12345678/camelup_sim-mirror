use calc::{Color, ColorState};

#[test]
fn color_state_assign() {
    let mut test_state = ColorState::default();
    test_state.assign_to_index(0, false);
    test_state.assign_to_index(2, false);
    test_state.assign_to_index(3, false);
    test_state.assign_to_index(4, false);
    assert_eq!(test_state.len(), 1);
    assert_eq!(test_state.state, 0b0100_0000);
    test_state.assign_to_index(3, true);
    test_state.assign_to_index(4, true);
    assert_eq!(test_state.len(), 3);
    assert_eq!(test_state.state, 0b0101_1000);
}

#[test]
fn color_state_iter() {
    let mut test_state = ColorState::default();
    assert_eq!(test_state.next(), Some(Color::Blue.as_byte()));
    assert_eq!(test_state.next(), Some(Color::Green.as_byte()));
    assert_eq!(test_state.next(), Some(Color::Orange.as_byte()));
    assert_eq!(test_state.next(), Some(Color::White.as_byte()));
    assert_eq!(test_state.next(), Some(Color::Yellow.as_byte()));
    assert_eq!(test_state.next(), None);
}
