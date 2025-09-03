use std::time::Duration;

use bevy::prelude::*;

use crate::system::GamePlay;

#[derive(Resource)]
struct Benchmark {
    timer: Timer,
    cycles: u64,
}

fn print_benchmark(mut benchmark: ResMut<Benchmark>, time: Res<Time>) {
    benchmark.timer.tick(time.delta());
    benchmark.cycles += 1;
    if benchmark.timer.just_finished() {
        println!(
            "循环速度: {:.3}/sec",
            benchmark.cycles as f64 / benchmark.timer.duration().as_secs_f64()
        );
        benchmark.cycles = 0;
    }
}

pub fn benchmark_system(app: &mut App) {
    app.insert_resource(Benchmark {
        timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
        cycles: 0,
    })
    .add_systems(Update, print_benchmark.in_set(GamePlay::AfterBattle));
}
