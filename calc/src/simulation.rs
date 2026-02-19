#[cfg(debug_assertions)]
use crate::Dice;
use crate::{color::Color, configuration::Configuration};
use std::convert::Into;
use std::rc::Rc;

use hashbrown::HashMap;

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

pub type Placement = [u8; 5];

pub struct SimulationResult {
    placements: Vec<Placement>,
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

    /// .placements() for Vec of all simulated game results
    pub fn placements(&self) -> &Vec<Placement> {
        &self.placements
    }

    /// Final aggregated placements of all final game states.
    ///
    /// Returns a 2D array where `[camel_color][place]` = number of times `camel_color` finished in `place`.
    pub fn aggregated_leaderboard(&self) -> [[u32; 5]; 5] {
        let mut placements: [[u32; 5]; 5] = [[0; 5]; 5];

        for placement in self.placements() {
            for (i, &color_index) in placement.iter().enumerate() {
                placements[color_index as usize][i] += 1;
            }
        }

        placements
    }
}

pub fn simulate_rounds_full(init_config: Configuration, n: u32) -> [[u64; 5]; 5] {
    let mut compressed: HashMap<Configuration, u64> = HashMap::new();
    compressed.insert(init_config, 1);

    for _ in 0..n {
        let mut next_compressed: HashMap<Configuration, u64> = HashMap::new();

        for (conf, count) in compressed.drain() {
            // if one camel has won, don't continue to simulate

            if conf.done {
                // scale by the full round factor because of early exit
                let round_factor = (1..=5_u64).product::<u64>() * 3_u64.pow(5);
                *next_compressed.entry(conf).or_insert(0) += count * round_factor;
            } else {
                simulate_rounds_full_rec(conf, count, &mut next_compressed);
            }
        }

        compressed = next_compressed;
    }

    let configs: Vec<(Configuration, u64)> = compressed.drain().collect();

    // temporary aggregated weighted placements
    let mut placements: [[u64; 5]; 5] = [[0; 5]; 5];
    for (conf, count) in configs {
        for (i, &color_index) in conf.leaderboard().iter().enumerate() {
            placements[color_index as usize][i] += count;
        }
    }
    placements
}

fn simulate_rounds_full_rec(
    conf: Configuration,
    count: u64,
    output: &mut HashMap<Configuration, u64>,
) {
    // Base case: all dice rolled this round
    if conf.available_colours.is_empty() {
        let mut result = conf;
        result.new_round();
        *output.entry(result).or_insert(0) += count;
        return;
    }

    if conf.map.camel_has_won() {
        let remaining = conf.available_colours.len() as u32;
        let multiplier = (1..=remaining as u64).product::<u64>() * 3_u64.pow(remaining);
        let mut result = conf;
        result.clear_moveable_camels();
        result.done = true;
        *output.entry(result).or_insert(0) += count * multiplier;
        return;
    }

    // For each available color, simulate all possible dice outcomes (1, 2, 3)
    for color_code in &conf.available_colours {
        let dice_color = Color::from_byte(color_code);
        debug_assert_ne!(dice_color, Color::None);

        for dice_value in 1..=3 {
            let mut new_conf = conf.clone();
            new_conf.available_colours.remove_color(dice_color);
            new_conf.map.move_camel(dice_color, dice_value as i8);

            simulate_rounds_full_rec(new_conf, count, output);
        }
    }
}

/// simulates the game from a initial configuration and returns [SimulationResult]
pub fn simulate_rounds(init_config: Configuration) -> SimulationResult {
    let mut cache: HashMap<Configuration, Rc<Vec<Placement>>> = HashMap::new();
    #[cfg(debug_assertions)]
    let mut stats = CacheStatistics::new();
    let placements = simulate_round_rec(
        init_config,
        &mut cache,
        #[cfg(debug_assertions)]
        &mut stats,
    );

    SimulationResult {
        placements: Rc::try_unwrap(placements).unwrap_or_else(|rc| (*rc).clone()),
        #[cfg(debug_assertions)]
        stats,
    }
}

fn simulate_round_rec(
    mut conf: Configuration,
    cache: &mut HashMap<Configuration, Rc<Vec<Placement>>>,
    #[cfg(debug_assertions)] stats: &mut CacheStatistics,
) -> Rc<Vec<Placement>> {
    // Base case
    if conf.available_colours.is_empty() {
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

    let mut all_placements = Vec::with_capacity(3_usize.pow(conf.available_colours.len() as u32));

    // For each available color, simulate all possible dice outcomes (1, 2, 3)
    for color_code in &conf.available_colours {
        let dice_color = Color::from_byte(color_code);
        debug_assert_ne!(dice_color, Color::None);

        for dice_value in 1..=3 {
            let mut new_conf = conf.clone();

            #[cfg(debug_assertions)]
            {
                new_conf.dice_queue.push(Dice {
                    color: dice_color,
                    value: dice_value,
                });
            }

            new_conf.available_colours.remove_color(dice_color);
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
