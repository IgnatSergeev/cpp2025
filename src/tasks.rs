pub fn dummy_task() {
    let mut var = 1;
    var += 1;
    std::hint::black_box(var);
}


