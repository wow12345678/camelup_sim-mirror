use calc::{CamelMap, Color};

#[test]
fn test_move_camel() {
    let mut map = CamelMap::new(vec![
        (0, Color::Blue),
        (0, Color::Green),
        (1, Color::Orange),
    ]);

    map.move_camel(Color::Green, 3);
    assert_eq!(map.find_camel(Color::Green), 3);
    assert_eq!(map.find_camel(Color::Blue), 0);
    assert_eq!(map.find_camel(Color::Orange), 1);
    assert_eq!(map.pos_color_map[3], Some(vec![Color::Green]));
}