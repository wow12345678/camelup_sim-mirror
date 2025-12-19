use calc::{CamelMap, Color, ColorState, Configuration, simulate_rounds};

fn simple_test_config() -> Configuration {
    Configuration {
        map: CamelMap::new(vec![
            (0, Color::Blue),
            (0, Color::Green),
            (1, Color::Yellow),
            (1, Color::White),
            (2, Color::Orange),
        ]),
        #[cfg(debug_assertions)]
        dice_queue: Vec::new(),
        available_colours: ColorState::new(vec![
            Color::Blue,
            Color::Green,
            Color::Orange,
            Color::White,
            Color::Yellow,
        ]),
    }
}

#[test]
fn test_simulate_round_new() {
    let mut small_config = simple_test_config();
    small_config.available_colours = ColorState::new(vec![Color::Blue]);

    let res = simulate_rounds(small_config);
    assert_eq!(res.placements().len(), 3);

    for placement in res.placements() {
        for &pos in placement {
            assert!(pos <= 4, "Position should be 0-4, got {}", pos);
        }
    }
}