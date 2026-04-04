use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GameAssetManager {
    game_assets: HashMap<&'static str, GameAsset>,
}

impl GameAssetManager {
    #[allow(dead_code)]
    fn init() -> Self {
        Self {
            game_assets: HashMap::new(),
        }
    }

    pub fn init_with_assets(assets: Vec<(&'static str, GameAsset)>) -> Self {
        let mut map = HashMap::new();

        for asset in assets {
            map.insert(asset.0, asset.1);
        }

        Self { game_assets: map }
    }

    pub fn get_asset(&self, name: &str) -> Option<&GameAsset> {
        self.game_assets.get(name)
    }
}

#[derive(Debug)]
pub struct GameAsset {
    data: AssetData,
}

impl GameAsset {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize, data: Vec<bool>) -> Self {
        Self {
            data: AssetData::new(width, height, data),
        }
    }

    pub fn from_pattern<const W: usize, const H: usize>(pattern: &[[bool; W]; H]) -> Self {
        Self {
            data: AssetData::from_pattern(pattern),
        }
    }

    pub fn view(&self) -> TransformedAsset<'_> {
        TransformedAsset {
            asset: self,
            scale: 1.0,
            flip: false,
        }
    }
}

#[macro_export]
macro_rules! asset_vec {
    ( $( ($name:expr, $pattern:expr) ),* $(,)? ) => {
        vec![
            $(
                (
                    $name,
                    GameAsset::from_pattern::<
                        { $pattern[0].len() },
                        { $pattern.len() },
                    >(&$pattern),
                ),
            )*
        ]
    };
}

// grid structure bitmap data with color
#[derive(Debug, Clone, Default)]
struct AssetData {
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl AssetData {
    fn new(width: usize, height: usize, data: Vec<bool>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    #[inline(always)]
    fn get(&self, x: usize, y: usize) -> bool {
        self.data[y * self.width + x]
    }

    fn from_pattern<const W: usize, const H: usize>(pattern: &[[bool; W]; H]) -> Self {
        let mut grid = Vec::with_capacity(W * H);
        for row in pattern.iter() {
            grid.extend_from_slice(row);
        }
        Self {
            width: W,
            height: H,
            data: grid,
        }
    }
}

pub struct TransformedAsset<'a> {
    asset: &'a GameAsset,
    scale: f32,
    flip: bool,
}

impl<'a> TransformedAsset<'a> {
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn flip(mut self, flip: bool) -> Self {
        self.flip = flip;
        self
    }

    pub fn width(&self) -> usize {
        (self.asset.data.width as f32 * self.scale).round() as usize
    }

    pub fn height(&self) -> usize {
        (self.asset.data.height as f32 * self.scale).round() as usize
    }

    fn get_pixel(&self, x: usize, y: usize) -> bool {
        let mut old_x = (x as f32 / self.scale).round() as usize;
        let old_y = (y as f32 / self.scale).round() as usize;

        let base_width = self.asset.data.width;
        let base_height = self.asset.data.height;

        old_x = old_x.clamp(0, base_width.saturating_sub(1));
        let old_y = old_y.clamp(0, base_height.saturating_sub(1));

        if self.flip {
            old_x = base_width.saturating_sub(1).saturating_sub(old_x);
        }

        self.asset.data.get(old_x, old_y)
    }

    pub fn render(self, area: Rect, buf: &mut Buffer, color: Color) {
        let x_offset = area.x;
        let current_height = self.height();
        let current_width = self.width();

        let y_base = area.y + area.height - (current_height as u16).div_ceil(2);

        let mut y = 0;
        while y < current_height {
            // Process two vertical pixels at once
            if y + 1 < current_height {
                for x in 0..current_width {
                    let char_x = x as u16 + x_offset;
                    let char_y = y_base + (y / 2) as u16;

                    if char_x < buf.area.right()
                        && char_y < buf.area.bottom()
                        && char_y >= buf.area.top()
                    {
                        match (self.get_pixel(x, y), self.get_pixel(x, y + 1)) {
                            (true, true) => {
                                buf[(char_x, char_y)]
                                    .set_char('▀')
                                    .set_fg(color)
                                    .set_bg(color);
                            }
                            (true, false) => {
                                buf[(char_x, char_y)].set_char('▀').set_fg(color);
                            }
                            (false, true) => {
                                buf[(char_x, char_y)].set_char('▄').set_fg(color);
                            }
                            (false, false) => {
                                // Both pixels empty, skip
                            }
                        }
                    }
                }
                y += 2; // Increment by 2 since we processed 2 rows
            } else {
                // Possible odd remainder row
                for x in 0..current_width {
                    if self.get_pixel(x, y) {
                        let char_x = x as u16 + x_offset;
                        let char_y = y_base + (y / 2) as u16;

                        if char_x < buf.area.right()
                            && char_y < buf.area.bottom()
                            && char_y >= buf.area.top()
                        {
                            buf[(char_x, char_y)].set_char('▀').set_fg(color);
                        }
                    }
                }
                y += 1;
            }
        }
    }
}
