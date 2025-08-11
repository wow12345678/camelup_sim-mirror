use std::collections::HashMap;
use std::f64;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Blue,
    Green,
    Orange,
    White,
    Yellow,
}
impl Color {
    fn map_to_u8(&self) -> u8 {
        match self {
            Color::Blue => 0,
            Color::Green => 1,
            Color::Orange => 2,
            Color::White => 3,
            Color::Yellow => 4,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Dice {
    color: Color,
    value: u8,
}

#[derive(Debug, Clone)]
struct Configuration {
    pos_color_map: HashMap<u8, Vec<Color>>,
    dice_queue: Vec<Dice>,
    available_colours: Vec<Color>,
}

impl Configuration {
    fn leaderboard(&self) -> [Color; 5] {
        let mut poses: Vec<(u8, &Vec<Color>)> =
            self.pos_color_map.iter().map(|(k, v)| (*k, v)).collect();
        poses.sort_by(|a, b| a.0.cmp(&b.0));
        poses.reverse();
        //TODO: maybe better solution for default
        let mut leaderboard: [Color; 5] = [Color::Green; 5];
        //Safety: i is always <= 5
        let mut i = 0;
        for pos in poses {
            for col in pos.1.iter().rev() {
                leaderboard[i] = *col;
                i += 1;
            }
        }
        leaderboard
    }
}

fn main() {
    const COUNT_ALL: u32 = 5 * 4 * 3 * 2 * 3_u32.pow(5);
    let init_pos_color_map: HashMap<u8, Vec<Color>> = HashMap::from([
        (0, vec![Color::Blue, Color::Green]),
        (1, vec![Color::Yellow, Color::White]),
        (2, vec![Color::Orange]),
    ]);

    let init_conf = Configuration {
        pos_color_map: init_pos_color_map,
        dice_queue: Vec::new(),
        available_colours: vec![
            Color::Blue,
            Color::Green,
            Color::Orange,
            Color::White,
            Color::Yellow,
        ],
    };

    let configs = simulate_round(init_conf);

    // let mut file = OpenOptions::new().append(true).open("test.txt").unwrap();
    // for (i, conf) in configs.iter().enumerate() {
    //     let _ = writeln!(file, "Config {i}:\n{conf:?}");
    // }

    let placements = aggragate_placements(&configs);

    println!("{:?}", placements);
    let prob_blue = placements[0][0] as f64 / COUNT_ALL as f64;
    let prob_green = placements[1][0] as f64 / COUNT_ALL as f64;
    let prob_orange = placements[2][0] as f64 / COUNT_ALL as f64;
    let prob_white = placements[3][0] as f64 / COUNT_ALL as f64;
    let prob_yellow = placements[4][0] as f64 / COUNT_ALL as f64;

    println!("Blue: {prob_blue}");
    println!("Green: {prob_green}");
    println!("Orange: {prob_orange}");
    println!("White: {prob_white}");
    println!("Yellow: {prob_yellow}");

    // TODO: this crashes my pc :(
    // let mut new_configs: Vec<Configuration> = Vec::new();
    //
    // for mut conf in configs {
    //     conf.available_colours = vec![
    //         Color::Blue,
    //         Color::Green,
    //         Color::Orange,
    //         Color::White,
    //         Color::Yellow,
    //     ];
    //     new_configs.append(&mut simulate_round(conf));
    // }
    //
    // let new_placements = aggragate_placements(&new_configs);
    //
    // println!("{:?}", new_placements);
    // let new_prob_blue = new_placements[0][0] as f64 / COUNT_ALL as f64;
    // let new_prob_green = new_placements[1][0] as f64 / COUNT_ALL as f64;
    // let new_prob_orange = new_placements[2][0] as f64 / COUNT_ALL as f64;
    // let new_prob_white = new_placements[3][0] as f64 / COUNT_ALL as f64;
    // let new_prob_yellow = new_placements[4][0] as f64 / COUNT_ALL as f64;
    //
    // println!("Blue: {new_prob_blue}");
    // println!("Green: {new_prob_green}");
    // println!("Orange: {new_prob_orange}");
    // println!("White: {new_prob_white}");
    // println!("Yellow: {new_prob_yellow}");
}

// [Color] -> [Placements]
fn aggragate_placements(configs: &Vec<Configuration>) -> [[u32; 5]; 5] {
    let mut placements: [[u32; 5]; 5] = [[0; 5]; 5];

    for conf in configs {
        for (i, col) in conf.leaderboard().iter().enumerate() {
            placements[col.map_to_u8() as usize][i] += 1;
        }
    }

    placements
}

//simulate round (remaining dice throws)
fn simulate_round(init_config: Configuration) -> Vec<Configuration> {
    let amount_throws = init_config.available_colours.len();
    let mut configs: Vec<Configuration> = vec![init_config];

    for _ in 0..amount_throws {
        let mut new_confs: Vec<Configuration> = Vec::new();
        for conf in &configs {
            for color in &conf.available_colours {
                new_confs.append(&mut simulate_dice_throw(conf, color));
            }
        }

        configs = new_confs;
    }

    configs
}

///returns all possible 3 dice values as new Configurations
fn simulate_dice_throw(conf: &Configuration, dice_color: &Color) -> Vec<Configuration> {
    let mut confs: Vec<Configuration> = Vec::new();

    for n in 1..=3 {
        let mut new_conf = conf.clone();
        new_conf.dice_queue.push(Dice {
            color: *dice_color,
            value: n,
        });

        new_conf.available_colours.retain(|c| *c != *dice_color);
        let old_pos = *new_conf
            .pos_color_map
            .iter()
            .filter_map(|(pos, colors)| {
                if colors.contains(dice_color) {
                    Some(pos)
                } else {
                    None
                }
            })
            .next()
            .unwrap();

        let old_pos_camels = new_conf.pos_color_map.get_mut(&old_pos).unwrap();

        let mut moving_camels: Vec<Color> = Vec::new();
        while let Some(last) = old_pos_camels.pop() {
            if last != *dice_color {
                moving_camels.push(last);
            } else {
                moving_camels.push(last);
                break;
            }
        }
        if old_pos_camels.is_empty() {
            new_conf.pos_color_map.remove(&old_pos);
        }

        moving_camels.reverse();
        let new_pos = old_pos + n;

        if let Some(camels_new_pos) = new_conf.pos_color_map.get_mut(&new_pos) {
            camels_new_pos.append(&mut moving_camels);
        } else {
            new_conf.pos_color_map.insert(new_pos, moving_camels);
        }

        confs.push(new_conf);
    }
    confs
}
