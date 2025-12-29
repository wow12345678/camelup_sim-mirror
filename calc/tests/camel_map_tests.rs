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
#[ignore = "only for debug"]
fn map_from_vec() {
    let pos_vec = vec![
        (1, Color::White),
        (1, Color::Yellow),
        (1, Color::Blue),
        (1, Color::Green),
        (2, Color::Orange),
    ];
    
    let map = CamelMap::new(pos_vec);

    println!("{map:?}");
}

