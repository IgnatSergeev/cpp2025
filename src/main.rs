mod runners;
mod tasks;
mod mode;

use clap::Parser;
use std::time::Instant;
use mode::Mode;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum)]
    benchmark: Mode,

    #[arg(long)]
    threads: Option<usize>,

    #[arg(short, long)]
    tasks: u32,

    #[arg(short, long, default_value_t = false)]
    pin: bool,
}


fn main() {
    let args = Args::parse();

    let mut tasks_map = std::collections::HashMap::new();

    tasks_map.insert(
        "dummy".to_string(),
        tasks::dummy_task as fn()
    );

    println!("Starting {} spawn benchmark", args.benchmark);
    println!("Configuration: {} threads, {} tasks", args.threads.unwrap_or(num_cpus::get()), args.tasks);

    match args.benchmark {
        Mode::Serial => {
            for task in tasks_map {
                std::println!("\nRunning {} function benchmark", task.0);
                let start = Instant::now();
                runners::serial(args.threads.unwrap_or(num_cpus::get()), args.tasks, task.1);
                let end = Instant::now();

                let time = end - start;
                std::println!("Throughput: {} tasks/s", args.tasks as f32 / time.as_secs_f32());
                std::println!("Elapsed time {}s", time.as_secs_f32());
            }
        }
        Mode::Parallel => {
            for task in tasks_map {
                std::println!("\nRunning {} function benchmark", task.0);
                let start = Instant::now();
                runners::parallel(args.threads.unwrap_or(num_cpus::get()), args.tasks, task.1);
                let end = Instant::now();

                let time = end - start;
                std::println!("Throughput: {} tasks/s", args.tasks as f32 / time.as_secs_f32());
                std::println!("Elapsed time {}s", time.as_secs_f32());
            }
        }
        Mode::Fibonacci => {
            println!("Fibonacci");
        }
    }
}
