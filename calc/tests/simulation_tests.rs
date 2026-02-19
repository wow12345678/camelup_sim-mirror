use calc::{Color, ColorState, Configuration, simulate_rounds, simulate_rounds_full};

fn simple_test_config() -> Configuration {
    Configuration::builder()
        .with_map(vec![
            (2, Color::Blue),
            (2, Color::Green),
            (2, Color::Orange),
            (2, Color::White),
            (2, Color::Yellow),
        ])
        .with_dice_queue(Vec::new())
        .with_available_colors(vec![
            Color::Blue,
            Color::Green,
            Color::White,
            Color::Orange,
            Color::Yellow,
        ])
        .build()
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
// #[ignore = "only for debug"]
fn test_simulate_round_debug() {
    let init_conf = Configuration::builder()
        .with_map(vec![
            (8, Color::Blue),
            (8, Color::Green),
            (8, Color::Orange),
            (8, Color::White),
            (8, Color::Yellow),
        ])
        .with_dice_queue(Vec::new())
        .with_available_colors(vec![
            Color::Blue,
            Color::Green,
            Color::White,
            Color::Orange,
            Color::Yellow,
        ])
        .build();

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

#[test]
fn test_simulate_rounds_debug() {
    let init_conf = Configuration::builder()
        .with_map(vec![
            (8, Color::Blue),
            (8, Color::Green),
            (8, Color::Orange),
            (8, Color::White),
            (8, Color::Yellow),
        ])
        .with_dice_queue(Vec::new())
        .with_available_colors(vec![
            Color::Blue,
            Color::Green,
            Color::White,
            Color::Orange,
            Color::Yellow,
        ])
        .build();

    const NUM_ROUNDS: u32 = 4;
    let res = simulate_rounds_full(init_conf, NUM_ROUNDS);
    println!("{res:?}");
    const ALL_GAME_STATES_COUNT: u64 = (5 * 4 * 3 * 2 * 3_u64.pow(5)).pow(NUM_ROUNDS);

    let new_prob_blue = res[0][0] as f64 / ALL_GAME_STATES_COUNT as f64;
    let new_prob_green = res[1][0] as f64 / ALL_GAME_STATES_COUNT as f64;
    let new_prob_orange = res[2][0] as f64 / ALL_GAME_STATES_COUNT as f64;
    let new_prob_white = res[3][0] as f64 / ALL_GAME_STATES_COUNT as f64;
    let new_prob_yellow = res[4][0] as f64 / ALL_GAME_STATES_COUNT as f64;

    println!("Blue: {new_prob_blue}");
    println!("Green: {new_prob_green}");
    println!("Orange: {new_prob_orange}");
    println!("White: {new_prob_white}");
    println!("Yellow: {new_prob_yellow}");
}
