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

#[test]
// #[ignore = "only for debug"]
fn map_from_vec() {
    let pos_vec = vec![
        (3, Color::White),
        (3, Color::Yellow),
        (3, Color::Blue),
        (3, Color::Green),
        (3, Color::Orange),
    ];
    
    let map = CamelMap::new(pos_vec);

    println!("{map:?}");
}

