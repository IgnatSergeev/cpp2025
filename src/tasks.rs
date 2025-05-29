pub fn dummy_task() {
    let mut var = 1;
    var += 1;
    std::hint::black_box(var);
}

pub fn dummy_io_task() {
    std::println!("Dummy print");
}

