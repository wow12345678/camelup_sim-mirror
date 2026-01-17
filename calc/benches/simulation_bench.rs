use std::hint::black_box;
use std::time::Duration;

use calc::{CamelMap, Color, ColorState, Configuration, simulate_rounds};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

// === Test Configurations ===

/// camels close together
fn config_clustered() -> Configuration {
    Configuration::builder()
        .with_map(vec![
            (0, Color::Blue),
            (0, Color::Green),
            (1, Color::Yellow),
            (1, Color::White),
            (2, Color::Orange),
        ])
        .with_available_colors(vec![
            Color::Blue,
            Color::Green,
            Color::Orange,
            Color::White,
            Color::Yellow,
        ])
        .build()
}

/// camels far apart
fn config_spread() -> Configuration {
    Configuration::builder()
        .with_map(vec![
            (0, Color::Blue),
            (3, Color::Green),
            (6, Color::Yellow),
            (9, Color::White),
            (12, Color::Orange),
        ])
        .with_available_colors(vec![
            Color::Blue,
            Color::Green,
            Color::Orange,
            Color::White,
            Color::Yellow,
        ])
        .build()
}

/// camels staked
fn config_stacked() -> Configuration {
    Configuration::builder()
        .with_map(vec![
            (0, Color::Blue),
            (0, Color::Green),
            (0, Color::Yellow),
            (0, Color::White),
            (0, Color::Orange),
        ])
        .with_available_colors(vec![
            Color::Blue,
            Color::Green,
            Color::Orange,
            Color::White,
            Color::Yellow,
        ])
        .build()
}

/// Partial round: only some colors remaining
fn config_partial(num_colors: usize) -> Configuration {
    let all_colors = [
        Color::Blue,
        Color::Green,
        Color::Orange,
        Color::White,
        Color::Yellow,
    ];
    Configuration::builder()
        .with_map(vec![
            (0, Color::Blue),
            (0, Color::Green),
            (1, Color::Yellow),
            (1, Color::White),
            (2, Color::Orange),
        ])
        .with_available_colors(all_colors[..num_colors].to_vec())
        .build()
}

// === Benchmarks ===

/// Benchmark full simulation with different board configurations
fn bench_full_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_simulation");

    group.bench_function("clustered", |b| {
        b.iter(|| simulate_rounds(black_box(config_clustered())))
    });

    group.bench_function("spread", |b| {
        b.iter(|| simulate_rounds(black_box(config_spread())))
    });

    group.bench_function("stacked", |b| {
        b.iter(|| simulate_rounds(black_box(config_stacked())))
    });

    group.finish();
}

/// Benchmark simulation with varying number of remaining dice
fn bench_partial_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("partial_simulation");

    for num_colors in [1, 2, 3, 4, 5] {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_colors),
            &num_colors,
            |b, &n| b.iter(|| simulate_rounds(black_box(config_partial(n)))),
        );
    }

    group.finish();
}

/// Benchmark the normalize operation in isolation
fn bench_normalize(c: &mut Criterion) {
    let mut group = c.benchmark_group("normalize");

    group.bench_function("spread_config", |b| {
        b.iter_batched(
            config_spread,
            |mut config| {
                config.normalize();
                black_box(config)
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("clustered_config", |b| {
        b.iter_batched(
            config_clustered,
            |mut config| {
                config.normalize();
                black_box(config)
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

/// Benchmark Configuration cloning (hot path in simulation)
fn bench_clone_configuration(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone");

    let clustered = config_clustered();
    group.bench_function("clustered", |b| b.iter(|| black_box(clustered.clone())));

    let spread = config_spread();
    group.bench_function("spread", |b| b.iter(|| black_box(spread.clone())));

    let stacked = config_stacked();
    group.bench_function("stacked", |b| b.iter(|| black_box(stacked.clone())));

    group.finish();
}

/// Benchmark the aggregated leaderboard calculation
fn bench_aggregated_leaderboard(c: &mut Criterion) {
    // Pre-compute simulation result once
    let result = simulate_rounds(config_clustered());

    c.bench_function("aggregated_leaderboard", |b| {
        b.iter(|| black_box(result.aggragated_leaderboard()))
    });
}

/// Benchmark CamelMap operations
fn bench_camel_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("camel_map");

    // Benchmark map creation
    group.bench_function("new", |b| {
        b.iter(|| {
            black_box(CamelMap::new(vec![
                (0, Color::Blue),
                (0, Color::Green),
                (1, Color::Yellow),
                (1, Color::White),
                (2, Color::Orange),
            ]))
        })
    });

    // Benchmark camel movement
    group.bench_function("move_camel", |b| {
        b.iter_batched(
            || {
                CamelMap::new(vec![
                    (0, Color::Blue),
                    (0, Color::Green),
                    (1, Color::Yellow),
                    (1, Color::White),
                    (2, Color::Orange),
                ])
            },
            |mut map| {
                map.move_camel(Color::Blue, 2);
                black_box(map)
            },
            criterion::BatchSize::SmallInput,
        )
    });

    // Benchmark find_camel lookup
    let map = CamelMap::new(vec![
        (0, Color::Blue),
        (3, Color::Green),
        (6, Color::Yellow),
        (9, Color::White),
        (12, Color::Orange),
    ]);
    group.bench_function("find_camel", |b| {
        b.iter(|| black_box(map.find_camel(Color::Orange)))
    });

    group.finish();
}

/// Benchmark ColorState operations
fn bench_color_state(c: &mut Criterion) {
    let mut group = c.benchmark_group("color_state");

    group.bench_function("new", |b| {
        b.iter(|| {
            black_box(ColorState::new(vec![
                Color::Blue,
                Color::Green,
                Color::Orange,
                Color::White,
                Color::Yellow,
            ]))
        })
    });

    let state = ColorState::new(vec![
        Color::Blue,
        Color::Green,
        Color::Orange,
        Color::White,
        Color::Yellow,
    ]);

    group.bench_function("len", |b| b.iter(|| black_box(state.len())));

    group.bench_function("iterate", |b| {
        b.iter(|| {
            let mut sum = 0u8;
            for color_code in &state {
                sum = sum.wrapping_add(color_code);
            }
            black_box(sum)
        })
    });

    group.bench_function("remove_color", |b| {
        b.iter_batched(
            || state.clone(),
            |mut s| {
                s.remove_color(Color::Orange);
                black_box(s)
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
                .measurement_time(Duration::from_secs(20));
    targets = bench_full_simulation,
              bench_partial_simulation,
              bench_normalize,
              bench_clone_configuration,
              bench_aggregated_leaderboard,
              bench_camel_map,
              bench_color_state
}

criterion_main!(benches);
