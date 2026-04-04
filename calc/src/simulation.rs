#[cfg(debug_assertions)]
use crate::Dice;
use crate::{color::Color, configuration::Configuration};
use dashmap::DashMap;
use rayon::prelude::*;
use std::convert::Into;
use std::rc::Rc;

use hashbrown::{DefaultHashBuilder, HashMap};

#[derive(Debug, Default)]
#[cfg(debug_assertions)]
struct CacheStatistics {
    cache_hits: u32,
    cache_misses: u32,
    total_function_calls: u32,
}

#[cfg(debug_assertions)]
impl CacheStatistics {
    fn new() -> Self {
        Self {
            cache_hits: 0,
            cache_misses: 0,
            total_function_calls: 0,
        }
    }

    fn record_hit(&mut self) {
        self.cache_hits += 1;
        self.total_function_calls += 1;
    }

    fn record_miss(&mut self) {
        self.cache_misses += 1;
        self.total_function_calls += 1;
    }

    pub(crate) fn print_stats(&self) {
        println!("=== Cache Statistics ===");
        println!("Total function calls: {}", self.total_function_calls);
        println!("Cache hits: {}", self.cache_hits);
        println!("Cache misses: {}", self.cache_misses);
    }
}

pub struct SimulationResult {
    leaderboard: [[u128; 5]; 5],
    #[cfg(debug_assertions)]
    stats: CacheStatistics,
}

impl SimulationResult {
    pub fn print_stats(&self) {
        if cfg!(debug_assertions) {
            #[cfg(debug_assertions)]
            self.stats.print_stats();
        }
    }

    /// Weighted aggregated leaderboard.
    ///
    /// Returns a 2D array where `[camel_color][place]` = weighted count as u128.
    pub fn weighted_leaderboard(&self) -> [[u128; 5]; 5] {
        self.leaderboard
    }
}

/// Simulates a complete Camel Up game from the given configuration until a camel wins.
///
/// Exhaustively explores all possible dice outcomes across multiple rounds using
/// parallel breadth-first expansion. Each round, every non-finished configuration is
/// expanded into all `5! × 3^5 = 29,160` possible dice permutations. Equivalent
/// configurations are compressed via a `HashMap` to keep the state space manageable.
///
/// Configurations where a camel has already won are marked `done` and carried forward
/// with a scaling factor so that all branches are weighted equally in the final result.
///
/// Returns a [`SimulationResult`] containing a weighted leaderboard where
/// `[camel_color][place]` holds the number of branches in which that camel finished
/// in that position. Divide by the row sum to get probabilities.
///
/// <div class="warning">
///
/// Do not try to simulate for Configurations with many camels on fields earlier than 7,
/// it takes very long and u128 overflows and thus the results are wrong.
///
/// </div>
///
pub fn simulate_rounds(init_config: Configuration) -> SimulationResult {
    let mut compressed: HashMap<Configuration, u128> = HashMap::new();
    const BRANCH_COUNT: u128 = 2 * 3 * 4 * 5_u128 * 3_u128.pow(5);
    compressed.insert(init_config, 1);

    loop {
        let d_hasher = DefaultHashBuilder::default();
        let next_compressed: DashMap<Configuration, u128, DefaultHashBuilder> =
            DashMap::with_hasher(d_hasher);
        let old_compressed: Vec<(Configuration, u128)> = compressed.drain().collect();

        old_compressed.into_par_iter().for_each(|(conf, count)| {
            if conf.done {
                // scale by the full round factor because of early exit
                *next_compressed.entry(conf).or_insert(0) += count * BRANCH_COUNT;
            } else {
                simulate_rounds_rec(conf, count, &next_compressed);
            }
        });

        compressed = next_compressed.into_iter().collect();

        if compressed.iter().all(|(conf, _)| conf.done) {
            break;
        }
    }

    let configs: Vec<(Configuration, u128)> = compressed.drain().collect();

    // aggregated weighted placements
    let mut placements: [[u128; 5]; 5] = [[0; 5]; 5];
    for (conf, count) in configs {
        for (i, &color_index) in conf.leaderboard().iter().enumerate() {
            placements[color_index as usize][i] += count;
        }
    }

    SimulationResult {
        leaderboard: placements,
        #[cfg(debug_assertions)]
        stats: CacheStatistics::new(),
    }
}

fn simulate_rounds_rec(
    conf: Configuration,
    count: u128,
    output: &DashMap<Configuration, u128, DefaultHashBuilder>,
) {
    // Check for game-ending condition first, even if all dice have been rolled
    if conf.map.camel_has_won() {
        let remaining = conf.available_colors.len() as u32;
        let multiplier = (1..=remaining as u128).product::<u128>() * 3_u128.pow(remaining);
        let mut result = conf;
        result.clear_moveable_camels();
        result.done = true;
        *output.entry(result).or_insert(0) += count * multiplier;
        return;
    }

    // Base case: all dice rolled this round, no winner yet
    if conf.available_colors.is_empty() {
        let mut result = conf;
        result.new_round();
        *output.entry(result).or_insert(0) += count;
        return;
    }

    // For each available color, simulate all possible dice outcomes (1, 2, 3)
    for color_code in &conf.available_colors {
        let dice_color = Color::try_from_byte(color_code).unwrap_or_else(|e| panic!("{}", e));

        for dice_value in 1..=3 {
            let mut new_conf = conf.clone();
            new_conf.available_colors.remove_color(dice_color);
            new_conf.map.move_camel(dice_color, dice_value as i8);

            simulate_rounds_rec(new_conf, count, output);
        }
    }
}

/// simulates the game from a initial configuration and returns [SimulationResult]
pub fn simulate_round(init_config: Configuration) -> SimulationResult {
    let mut cache: HashMap<Configuration, Rc<Vec<[u8; 5]>>> = HashMap::new();
    #[cfg(debug_assertions)]
    let mut stats = CacheStatistics::new();
    let placements = simulate_round_rec(
        init_config,
        &mut cache,
        #[cfg(debug_assertions)]
        &mut stats,
    );

    let mut leaderboard: [[u128; 5]; 5] = [[0; 5]; 5];
    for placement in placements.iter() {
        for (i, &color_index) in placement.iter().enumerate() {
            leaderboard[color_index as usize][i] += 1;
        }
    }

    SimulationResult {
        leaderboard,
        #[cfg(debug_assertions)]
        stats,
    }
}

fn simulate_round_rec(
    mut conf: Configuration,
    cache: &mut HashMap<Configuration, Rc<Vec<[u8; 5]>>>,
    #[cfg(debug_assertions)] stats: &mut CacheStatistics,
) -> Rc<Vec<[u8; 5]>> {
    // Base case
    if conf.available_colors.is_empty() {
        #[cfg(debug_assertions)]
        stats.record_miss();
        return Rc::new(vec![conf.leaderboard().map(|color| color.into())]);
    }

    // this is only good for 1 round simulations, since otherwise the progress
    // of the game gets lost
    conf.normalize();

    // check cache
    if let Some(cached_result) = cache.get(&conf) {
        #[cfg(debug_assertions)]
        stats.record_hit();
        return cached_result.clone();
    }

    #[cfg(debug_assertions)]
    stats.record_miss();

    let mut all_placements = Vec::with_capacity(3_usize.pow(conf.available_colors.len() as u32));

    // For each available color, simulate all possible dice outcomes (1, 2, 3)
    for color_code in &conf.available_colors {
        let dice_color = Color::try_from_byte(color_code).unwrap_or_else(|e| panic!("{}", e));

        for dice_value in 1..=3 {
            let mut new_conf = conf.clone();

            #[cfg(debug_assertions)]
            {
                new_conf.dice_queue.push(Dice {
                    color: dice_color,
                    value: dice_value,
                });
            }

            new_conf.available_colors.remove_color(dice_color);
            new_conf.map.move_camel(dice_color, dice_value as i8);

            // recursive call
            let sub_placements = simulate_round_rec(
                new_conf,
                cache,
                #[cfg(debug_assertions)]
                stats,
            );

            all_placements.extend(sub_placements.iter());
        }
    }

    let result = Rc::new(all_placements);
    cache.insert(conf, result.clone());

    result
}
