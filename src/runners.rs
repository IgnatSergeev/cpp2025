pub fn serial(threads: usize, tasks: u32, task: fn())
{
    let mut handles = Vec::with_capacity(tasks as usize);
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(threads.into())
        .build()
        .unwrap();
    for _ in 0..tasks {
        handles.push(rt.spawn(async move { task() }));
    }

    rt.block_on(async {
        for handle in handles {
            handle.await.unwrap();
        }
    });
}

pub fn parallel(threads: usize, tasks: u32, task: fn()) {
    let tasks_per_thread: usize = (tasks as f32 / threads as f32) as usize;
    let remaining_tasks: usize = (tasks as f32 % threads as f32) as usize;

    let mut threads_handle = Vec::with_capacity(threads as usize);
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(threads.into())
        .build()
        .unwrap();
    for i in 0..threads {
        let mut current_tasks = tasks_per_thread;
        if i < remaining_tasks {
            current_tasks += 1;
        }
        threads_handle.push(rt.spawn(async move {
            let mut tasks_handle = Vec::with_capacity(current_tasks);
            for _ in 0..current_tasks {
                tasks_handle.push(tokio::task::spawn(async move { task() }));
            }

            for handle in tasks_handle {
                handle.await.unwrap();
            }
        }));
    }

    rt.block_on(async {
        for handle in threads_handle {
            handle.await.unwrap();
        }
    });
}
