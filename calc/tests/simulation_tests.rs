use calc::{CamelMap, Color, ColorState, Configuration, simulate_rounds};

fn simple_test_config() -> Configuration {
    Configuration {
        map: CamelMap::builder()
            .with_positions(vec![
                (0, Color::Blue),
                (0, Color::Green),
                (1, Color::Yellow),
                (1, Color::White),
                (2, Color::Orange),
            ])
            .build()
            .unwrap(),
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

#[test]
#[ignore = "only for debug"]
fn test_simulate_round_debug() {
    let color_state = ColorState::new(vec![
        Color::Blue,
        Color::Green,
        Color::White,
        Color::Orange,
        Color::Yellow,
    ]);
    let init_conf = Configuration {
        map: CamelMap::builder()
            .with_positions(vec![
                (1, Color::Blue),
                (1, Color::Green),
                (2, Color::Orange),
                (3, Color::White),
                (3, Color::Yellow),
            ])
            .build().unwrap(),
        #[cfg(debug_assertions)]
        dice_queue: Vec::new(),
        available_colours: color_state,
    };

    let res = simulate_rounds(init_conf);

    res.print_stats();

    let new_placements = res.aggregated_leaderboard();
    println!("{:?}", new_placements);
    let all_game_states_count = res.placements().len();

    let new_prob_blue = new_placements[0][0] as f64 / all_game_states_count as f64;
    let new_prob_green = new_placements[1][0] as f64 / all_game_states_count as f64;
    let new_prob_orange = new_placements[2][0] as f64 / all_game_states_count as f64;
    let new_prob_white = new_placements[3][0] as f64 / all_game_states_count as f64;
    let new_prob_yellow = new_placements[4][0] as f64 / all_game_states_count as f64;

    println!("Blue: {new_prob_blue}");
    println!("Green: {new_prob_green}");
    println!("Orange: {new_prob_orange}");
    println!("White: {new_prob_white}");
    println!("Yellow: {new_prob_yellow}");
}
