use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use cpp2025::{ runners, tasks, mode::Mode };

fn serial(ctx: &mut Criterion) {
    let mut tasks_map = std::collections::HashMap::new();
    tasks_map.insert(
        "dummy".to_string(),
        tasks::dummy_task as fn()
    );

    measure_mode(Mode::Serial, tasks_map.clone(), ctx);
    measure_mode(Mode::Parallel, tasks_map.clone(), ctx);
}

fn measure_mode(mode: Mode, tasks_map: std::collections::HashMap<std::string::String, fn()>, ctx: &mut criterion::Criterion) {
    let mut mode_group = ctx.benchmark_group(mode.to_string());
    let runner = match mode {
        Mode::Serial => runners::serial,
        Mode::Parallel => runners::parallel,
        Mode::Fibonacci => runners::serial,
    };

    for task in tasks_map {
        for tasks_amount in [10_i32.pow(3), 10_i32.pow(4), 10_i32.pow(5), 5 * 10_i32.pow(5), 10_i32.pow(6)] {
            mode_group.sample_size(15);
            mode_group.measurement_time(std::time::Duration::from_secs(10));
            mode_group.throughput(criterion::Throughput::Elements(tasks_amount as u64));
            mode_group.bench_with_input(
                BenchmarkId::new(task.0.clone(), tasks_amount),
                &tasks_amount,
                |b, amount| b.iter(|| runner(num_cpus::get(), *amount as u32, task.1))
            );
        }
    }

    mode_group.finish();
}

criterion_group!(benches, serial);
criterion_main!(benches);
