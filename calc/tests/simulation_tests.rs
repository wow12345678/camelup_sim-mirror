#[allow(unused)]
use calc::{simulate_round, simulate_rounds, Color, ColorState, Configuration};

#[allow(unused)]
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
#[ignore = "only for debug"]
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

    let leaderboard = res.weighted_leaderboard();
    println!("{:?}", leaderboard);
    let all_game_states_count: u128 = leaderboard[0].iter().sum();

    let new_prob_blue = leaderboard[0][0] as f64 / all_game_states_count as f64;
    let new_prob_green = leaderboard[1][0] as f64 / all_game_states_count as f64;
    let new_prob_orange = leaderboard[2][0] as f64 / all_game_states_count as f64;
    let new_prob_white = leaderboard[3][0] as f64 / all_game_states_count as f64;
    let new_prob_yellow = leaderboard[4][0] as f64 / all_game_states_count as f64;

    println!("Blue: {new_prob_blue}");
    println!("Green: {new_prob_green}");
    println!("Orange: {new_prob_orange}");
    println!("White: {new_prob_white}");
    println!("Yellow: {new_prob_yellow}");
}

#[ignore = "only for debug"]
#[test]
fn test_simulate_rounds_debug() {
    let init_conf = Configuration::builder()
        .with_map(vec![
            (7, Color::Blue),
            (7, Color::Green),
            (7, Color::Orange),
            (7, Color::White),
            (7, Color::Yellow),
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
    let leaderboard = res.weighted_leaderboard();
    println!("{leaderboard:?}");
    let all_game_states_count: u128 = leaderboard[0].iter().sum();
    println!("{all_game_states_count}");

    let new_prob_blue = leaderboard[0][0] as f64 / all_game_states_count as f64;
    let new_prob_green = leaderboard[1][0] as f64 / all_game_states_count as f64;
    let new_prob_orange = leaderboard[2][0] as f64 / all_game_states_count as f64;
    let new_prob_white = leaderboard[3][0] as f64 / all_game_states_count as f64;
    let new_prob_yellow = leaderboard[4][0] as f64 / all_game_states_count as f64;

    println!("Blue: {new_prob_blue}");
    println!("Green: {new_prob_green}");
    println!("Orange: {new_prob_orange}");
    println!("White: {new_prob_white}");
    println!("Yellow: {new_prob_yellow}");
}

// #[test]
// #[ignore = "only for debug"]
// fn test_simulate_n_rounds_debug() {
//     let init_conf = Configuration::builder()
//         .with_map(vec![
//             (8, Color::Blue),
//             (8, Color::Green),
//             (8, Color::Orange),
//             (8, Color::White),
//             (8, Color::Yellow),
//         ])
//         .with_dice_queue(Vec::new())
//         .with_available_colors(vec![
//             Color::Blue,
//             Color::Green,
//             Color::White,
//             Color::Orange,
//             Color::Yellow,
//         ])
//         .build();
//
//     const NUM_ROUNDS: u32 = 5;
//     let res = simulate_n_rounds_full(init_conf, NUM_ROUNDS);
//     println!("{res:?}");
//     let all_game_states_count: u128 = res[0].iter().sum();
//     println!("{all_game_states_count}");
//
//     let new_prob_blue = res[0][0] as f64 / all_game_states_count as f64;
//     let new_prob_green = res[1][0] as f64 / all_game_states_count as f64;
//     let new_prob_orange = res[2][0] as f64 / all_game_states_count as f64;
//     let new_prob_white = res[3][0] as f64 / all_game_states_count as f64;
//     let new_prob_yellow = res[4][0] as f64 / all_game_states_count as f64;
//
//     println!("Blue: {new_prob_blue}");
//     println!("Green: {new_prob_green}");
//     println!("Orange: {new_prob_orange}");
//     println!("White: {new_prob_white}");
//     println!("Yellow: {new_prob_yellow}");
// }
