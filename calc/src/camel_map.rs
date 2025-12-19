use std::convert::Into;
use crate::color::Color;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CamelMap {
    pub pos_color_map: [Option<Vec<Color>>; 16],
    // colors are encoded by index like the enum
    pub color_pos_map: [u8; 5],
}

impl CamelMap {
    pub fn new(init_positions: Vec<(u8, Color)>) -> Self {
        let mut res = Self {
            pos_color_map: [const { None }; 16],
            color_pos_map: [0; 5],
        };

        for pos in init_positions {
            res.insert_camel(pos);
        }
        res
    }

    //inserts camel at postion
    fn insert_camel(&mut self, (pos, color): (u8, Color)) {
        if let Some(vec) = &mut self.pos_color_map[pos as usize] {
            vec.push(color);
        } else {
            self.pos_color_map[pos as usize] = Some(vec![color]);
        }
        self.color_pos_map[color as usize] = pos;
    }

    // moves camel to position along with all camels on top of it
    pub fn move_camel(&mut self, camel: Color, by: u8) {
        let old_field_pos = self.find_camel(camel);
        let new_pos = old_field_pos + by;

        let old_pos_in_stack = &mut self.pos_color_map[old_field_pos as usize]
            .iter()
            .find_map(|v| v.iter().position(|c| *c == camel))
            .unwrap();

        let mut moving_camels = self.pos_color_map[old_field_pos as usize]
            .as_mut()
            .unwrap()
            .split_off(*old_pos_in_stack);

        // update positions
        for col in &moving_camels {
            self.color_pos_map[Into::<usize>::into(*col)] = new_pos;
        }

        if let Some(vec) = &self.pos_color_map[old_field_pos as usize]
            && vec.is_empty()
        {
            self.pos_color_map[old_field_pos as usize] = None;
        }

        if let Some(vec_new_pos) = self.pos_color_map[new_pos as usize].as_mut() {
            vec_new_pos.append(&mut moving_camels);
        } else {
            self.pos_color_map[new_pos as usize] = Some(moving_camels);
        }
    }

    pub fn find_camel(&self, color: Color) -> u8 {
        self.color_pos_map[Into::<usize>::into(color)]
    }
}
