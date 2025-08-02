use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::task;
use tokio::time;

#[tokio::test]
async fn test_func1() {
    assert_eq!(2 + 2, 4);
}

#[tokio::test]
async fn test_func7() {
    let task_a = task::spawn(async move {
        println!("in task_a");
        time::sleep(Duration::from_secs(3)).await; // 等待3s
        1
    });
    let task_b = task::spawn(async move {
        println!("in task_b");
        2
    });
    let task_c = task::spawn(async move {
        println!("in task_c");
        3
    });

    let ret = tokio::select! {
        r = task_a => r.unwrap(),
        r = task_b => r.unwrap(),
        r = task_c => r.unwrap(),
    };

    println!("{}", ret);
}

#[tokio::test]
async fn test_func6() {
    let task_a = task::spawn(async move{
        println!("in task_a");
        time::sleep(Duration::from_secs(3)).await; // 等待3s
        1
    });
    let task_b = task::spawn(async move {
        println!("in task_b");
        2
    });
    let task_c = task::spawn(async move {
        println!("in task_c");
        3
    });
    let (r1,r2,r3) = tokio::join!(task_a,task_b,task_c);
    println!("{}, {}, {}", r1.unwrap(), r2.unwrap(), r3.unwrap());
}

#[tokio::test]
async fn test_func5() {
    let task_a = task::spawn(async move {
        println!("in task_a");
        time::sleep(Duration::from_secs(3)).await;
        1
    });
    let task_b = task::spawn(async move {
        println!("in task_b");
        2
    });
    let task_c = task::spawn(async move {
        println!("in task_c");
        3
    });

    let mut tasks = Vec::with_capacity(3);
    tasks.push(task_a);
    tasks.push(task_b);
    tasks.push(task_c);

    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        println!("iterate task result..");
        //
        outputs.push(task.await.unwrap());
    }
    println!("{:?}", outputs);
}

#[tokio::test]
async fn test_func4() {
    let ops = vec![1, 2, 3];
    let mut tasks = Vec::with_capacity(ops.len());
    for op in ops {
        //
        tasks.push(tokio::spawn(my_background_op(op)))
    }
    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        //
        outputs.push(task.await.unwrap());
    }
    println!("{:?}", outputs);
}

async fn my_background_op(id: i32) -> String {
    let s = format!("Starting background task {}.", id);
    println!("{}", s);
    s
}

#[tokio::test]
async fn test_func3() {
    let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (tx, mut rx) = mpsc::channel::<(u32, oneshot::Sender<bool>)>(100);

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let task_a = task::spawn(async move {
        time::sleep(Duration::from_secs(3)).await;
        let (resp_tx, resp_rx) = oneshot::channel();
        if let Err(_) = tx1.send((50, resp_tx)).await {
            println!("receiver dropped");
            return;
        }
        if let Ok(ret) = resp_rx.await {
            if ret {
                println!("task_a finished with success.");
            } else {
                println!("task_a finished with failure.");
            }
        } else {
            println!("oneshot sender dropped");
            return;
        }
    });
    let task_b = task::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        if let Err(_) = tx2.send((100, resp_tx)).await {
            println!("receiver dropped");
            return;
        }
        if let Ok(ret) = resp_rx.await {
            if ret {
                println!("task_b finished with success.");
            } else {
                println!("task_b finished with failure.");
            }
        } else {
            println!("oneshot sender dropped");
            return;
        }
    });

    let task_c = task::spawn(async move {
        while let Some((i, resp_tx)) = rx.recv().await {
            println!("got = {}", i);
            db[4] = i;
            println!("{:?}", db);
            resp_tx.send(true).unwrap();
        }
    });

    _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();
    _ = task_c.await.unwrap();
}

#[tokio::test]
async fn test_func2() {
    let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (tx, mut rx) = mpsc::channel::<u32>(100);

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    // drop 原始的 tx，保持只有 tx1 和 tx2 存在
    drop(tx);

    let task_a = tokio::task::spawn(async move {
        if tx1.send(50).await.is_err() {
            println!("receiver dropped");
            return;
        }
    });

    let task_b = tokio::task::spawn(async move {
        if tx2.send(100).await.is_err() {
            println!("receiver dropped");
            return;
        }
    });

    let task_c = tokio::task::spawn(async move {
        while let Some(i) = rx.recv().await {
            println!("got = {}", i);
            // 此处不可变 db 需改写为可变或使用Arc<Mutex<_>>等安全操作
            // 假设这里先注释掉 db 修改，避免错误
            // db[4] = i;
            // println!("{:?}", db);
        }
        println!("Receiver loop ended");
    });

    task_a.await.unwrap();
    task_b.await.unwrap();
    task_c.await.unwrap();
}
