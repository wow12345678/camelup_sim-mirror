use std::collections::HashMap;
use std::fs::{self, OpenOptions, write};
use std::hash::Hash;
use std::io::Write;

#[derive(Debug, Clone)]
struct Camel {
    color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Blue,
    Green,
    Orange,
    White,
    Yellow,
}

#[derive(Debug, Clone, Copy)]
struct Dice {
    color: Color,
    value: u32,
}

#[derive(Debug, Clone)]
struct Configuration {
    pos_color_map: HashMap<u32, Vec<Color>>,
    dice_queue: Vec<Dice>,
    available_colours: Vec<Color>,
}

fn main() {
    const COUNT_ALL: u32 = 5 * 4 * 3 * 2 * 3_u32.pow(5);
    let init_pos_color_map: HashMap<u32, Vec<Color>> = HashMap::from([
        (1, vec![Color::Blue, Color::Green]),
        (2, vec![Color::Yellow, Color::White]),
        (3, vec![Color::Orange]),
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

    for (i, conf) in configs.iter().enumerate() {
        let mut file = OpenOptions::new().append(true).open("test.txt").unwrap();
        let _ = writeln!(file, "Config {i}:\n{conf:?}");
    }

    //TODO:calc camel leaderboard position
    //TODO:calc probabilities
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
