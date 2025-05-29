mod runners;
mod tasks;

use clap::{ Parser, ValueEnum };
use std::fmt;
use std::time::Instant;

#[derive(Clone, ValueEnum)]
enum Mode {
    Serial,
    Parallel,
    Fibonacci,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Serial => write!(f, "Serial"),
            Mode::Parallel=> write!(f, "Parallel"),
            Mode::Fibonacci => write!(f, "Fibonacci"),
        }
    }
}

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

    println!("Running in {} mode", args.benchmark);
    println!("Configuration: {} threads, {} tasks", args.threads.unwrap_or(num_cpus::get()), args.tasks);

    match args.benchmark {
        Mode::Serial => {
            std::println!("Starting serial spawn of dummy io");

            let start = Instant::now();
            runners::serial(args.threads.unwrap_or(num_cpus::get()), args.tasks, tasks::dummy_io_task);
            let end = Instant::now();

            let time = end - start;
            std::println!("Throughput: {} tasks/s", args.tasks as f32 / time.as_secs_f32());
            std::println!("Elapsed time {}s", time.as_secs_f32());

        }
        Mode::Parallel => {
            std::println!("Starting parallel spawn of dummy io");

            let start = Instant::now();
            runners::parallel(args.threads.unwrap_or(num_cpus::get()), args.tasks, tasks::dummy_io_task);
            let end = Instant::now();

            let time = end - start;
            std::println!("Throughput: {} tasks/s", args.tasks as f32 / time.as_secs_f32());
            std::println!("Elapsed time {}s", time.as_secs_f32());

        }
        Mode::Fibonacci => {
            println!("Fibonacci");
        }
    }
}
