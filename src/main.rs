use std::collections::HashMap;
use std::fs::{self, OpenOptions, write};
use std::io::Write;

#[derive(Debug, Clone)]
struct Camel {
    color: Color,
    upper_camel: Vec<Color>,
    pos: u32,
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
    dice_queue: Vec<Dice>,
    available_colours: Vec<Color>,
    camel_map: HashMap<Color, Camel>,
}

fn main() {
    const COUNT_ALL: u32 = 5 * 4 * 3 * 2 * 3_u32.pow(5);
    let mut init_camels: HashMap<Color, Camel> = HashMap::new();
    init_camels.insert(
        Color::Blue,
        Camel {
            color: Color::Blue,
            upper_camel: vec![Color::Yellow],
            pos: 1,
        },
    );
    init_camels.insert(
        Color::Green,
        Camel {
            color: Color::Green,
            upper_camel: vec![Color::White],
            pos: 2,
        },
    );
    init_camels.insert(
        Color::Orange,
        Camel {
            color: Color::Orange,
            upper_camel: vec![],
            pos: 3,
        },
    );
    init_camels.insert(
        Color::White,
        Camel {
            color: Color::White,
            upper_camel: vec![],
            pos: 2,
        },
    );
    init_camels.insert(
        Color::Yellow,
        Camel {
            color: Color::Yellow,
            upper_camel: vec![],
            pos: 1,
        },
    );

    let init_conf = Configuration {
        camel_map: init_camels,
        dice_queue: Vec::new(),
        available_colours: vec![
            Color::Blue,
            Color::Green,
            Color::Orange,
            Color::White,
            Color::Yellow,
        ],
    };

    let mut configs: Vec<Configuration> = vec![init_conf];

    for _ in 0..5 {
        let mut new_confs: Vec<Configuration> = Vec::new();
        for conf in &configs {
            for color in &conf.available_colours {
                new_confs.append(&mut simulate_dice_throw(conf, color));
            }
        }

        configs = new_confs;
    }

    for (i, conf) in configs.iter().enumerate() {
        let mut file = OpenOptions::new().append(true).open("test.txt").unwrap();
        let _ = writeln!(file, "Config {i}:\n{:?}", conf);
    }

    //TODO:calc camel leaderboard position
    //TODO:calc probabilities
}

///returns all possible 3 dice values as new Configurations
fn simulate_dice_throw(conf: &Configuration, dice_color: &Color) -> Vec<Configuration> {
    let init_camel_pos = conf
        .camel_map
        .values()
        .filter_map(|camel| {
            if camel.color == *dice_color {
                Some(camel.pos)
            } else {
                None
            }
        })
        .next();
    assert!(init_camel_pos.is_some());
    let mut confs: Vec<Configuration> = Vec::new();

    for n in 1..=3 {
        let mut new_conf = conf.clone();
        new_conf.dice_queue.push(Dice {
            color: *dice_color,
            value: n,
        });

        new_conf.available_colours.retain(|c| *c != *dice_color);

        new_conf
            .camel_map
            .entry(*dice_color)
            .and_modify(|c| c.pos += n);

        //remove from camles on which the camle sits
        for camle in new_conf.camel_map.values_mut() {
            camle.upper_camel.retain(|col| col != dice_color);
        }

        let up_camels = new_conf
            .camel_map
            .get(dice_color)
            .unwrap()
            .upper_camel
            .clone();

        //move up camles
        for up_color in up_camels {
            new_conf
                .camel_map
                .entry(up_color)
                .and_modify(|c| c.pos += n);
        }

        confs.push(new_conf);
    }
    confs
}
