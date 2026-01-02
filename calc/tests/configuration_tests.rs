use calc::{Color, Configuration};

#[test]
fn test_configuration_builder_default() {
    let config = Configuration::builder().build();

    // Should use default starting positions
    assert_eq!(config.map.color_pos_map[Color::Blue as usize], 0);
    assert_eq!(config.map.color_pos_map[Color::Green as usize], 0);
    assert_eq!(config.map.color_pos_map[Color::White as usize], 1);
    assert_eq!(config.map.color_pos_map[Color::Yellow as usize], 1);
    assert_eq!(config.map.color_pos_map[Color::Orange as usize], 2);

    // Should have all colors available
    assert_eq!(config.available_colours.len(), 5);

    #[cfg(debug_assertions)]
    assert!(config.dice_queue.is_empty());
}

#[test]
fn test_configuration_builder_with_custom_map() {
    let config = Configuration::builder()
        .with_map(vec![
            (3, Color::Blue),
            (5, Color::Green),
            (7, Color::Orange),
            (9, Color::White),
            (11, Color::Yellow),
        ])
        .build();

    assert_eq!(config.map.color_pos_map[Color::Blue as usize], 3);
    assert_eq!(config.map.color_pos_map[Color::Green as usize], 5);
    assert_eq!(config.map.color_pos_map[Color::Orange as usize], 7);
    assert_eq!(config.map.color_pos_map[Color::White as usize], 9);
    assert_eq!(config.map.color_pos_map[Color::Yellow as usize], 11);
}

#[test]
fn test_configuration_builder_with_available_colors() {
    let config = Configuration::builder()
        .with_available_colors(vec![Color::Blue, Color::Orange])
        .build();

    assert_eq!(config.available_colours.len(), 2);
}

#[test]
#[cfg(debug_assertions)]
fn test_configuration_builder_with_dice_queue() {
    let dice_data = vec![(Color::Blue, 1), (Color::Green, 3), (Color::Orange, 2)];

    let config = Configuration::builder()
        .with_dice_queue(dice_data.clone())
        .build();

    assert_eq!(config.dice_queue.len(), 3);
    assert_eq!(config.dice_queue[0].color, Color::Blue);
    assert_eq!(config.dice_queue[0].value, 1);
    assert_eq!(config.dice_queue[1].color, Color::Green);
    assert_eq!(config.dice_queue[1].value, 3);
    assert_eq!(config.dice_queue[2].color, Color::Orange);
    assert_eq!(config.dice_queue[2].value, 2);
}

#[test]
#[cfg(debug_assertions)]
fn test_configuration_builder_add_dice() {
    let config = Configuration::builder()
        .add_dice(Color::Blue, 1)
        .add_dice(Color::Green, 3)
        .add_dice(Color::Orange, 2)
        .build();

    assert_eq!(config.dice_queue.len(), 3);
    assert_eq!(config.dice_queue[0].color, Color::Blue);
    assert_eq!(config.dice_queue[0].value, 1);
    assert_eq!(config.dice_queue[1].color, Color::Green);
    assert_eq!(config.dice_queue[1].value, 3);
    assert_eq!(config.dice_queue[2].color, Color::Orange);
    assert_eq!(config.dice_queue[2].value, 2);
}

#[test]
fn test_configuration_builder_comprehensive() {
    let config = Configuration::builder()
        .with_map(vec![
            (2, Color::Blue),
            (4, Color::Green),
            (6, Color::Orange),
            (8, Color::White),
            (10, Color::Yellow),
        ])
        .with_available_colors(vec![Color::Blue, Color::Green, Color::Orange])
        .build();

    assert_eq!(config.map.color_pos_map[Color::Blue as usize], 2);
    assert_eq!(config.map.color_pos_map[Color::Green as usize], 4);
    assert_eq!(config.map.color_pos_map[Color::Orange as usize], 6);
    assert_eq!(config.available_colours.len(), 3);

    #[cfg(debug_assertions)]
    assert!(config.dice_queue.is_empty());
}

#[test]
fn test_configuration_builder_method_chaining() {
    let builder = Configuration::builder()
        .with_map(vec![(0, Color::Blue)])
        .with_available_colors(vec![Color::Blue]);

    #[cfg(debug_assertions)]
    let builder = builder.add_dice(Color::Blue, 1);

    let _config = builder.build();
}

#[test]
fn test_configuration_normalization() {
    let mut config = Configuration::builder()
        .with_map(vec![
            (2, Color::Blue),
            (4, Color::Green),
            (6, Color::Orange),
            (8, Color::White),
            (10, Color::Yellow),
        ])
        .with_available_colors(vec![Color::Blue, Color::Green, Color::Orange])
        .build();

    config.normalize();

    assert_eq!(config.map.find_camel(Color::Blue), 0);
    assert_eq!(config.map.find_camel(Color::Green), 2);
    assert_eq!(config.map.find_camel(Color::Orange), 4);
    assert_eq!(config.map.find_camel(Color::White), 6);
    assert_eq!(config.map.find_camel(Color::Yellow), 8);
}

