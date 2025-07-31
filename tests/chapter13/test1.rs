use std::fmt::format;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::{task, time};
// 引入AsyncReadExt trait

#[tokio::test]
async fn test_func1() {
    println!("Hello world");
    // 这里可以写更多异步测试代码，使用 .await 调用异步函数
}
#[tokio::test]
async fn test_func7() {
    async fn my_backgroud_op(id: i32) -> String {
        let s = format!("Starting background task {}.", id);
        print!("{}", s);
        s
    }
    let ops = vec![1, 2, 3];
    let mut tasks = Vec::with_capacity(ops.len());
    for op in ops {
        //
        tasks.push(tokio::spawn(my_backgroud_op(op)))
    }
    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        outputs.push(task.await.unwrap())
    }
    println!("{:?}", outputs)
}
#[tokio::test]
async fn test_func6() {
    let task_a = task::spawn(async {
        panic!("something bad happened");
    });
    //
    assert!(task_a.await.is_err());
}

#[tokio::test]
async fn test_func5() {
    let task_a = task::spawn(async { "hello world!" });
    //
    //
    let result = task_a.await.unwrap();
    assert_eq!(result, "hello world!");
}
#[tokio::test]
async fn test_func4() {
    // 测试内部可以手动暂停时间
    tokio::time::pause();
    let mut interval = tokio::time::interval(Duration::from_millis(10));
    interval.tick().await;
    interval.tick().await;
    interval.tick().await;
}

#[tokio::test]
async fn test_func3() {
    async fn doit() -> std::io::Result<()> {
        let mut file = File::open("foo.txt").await?;
        let mut contents = vec![];
        //
        file.read_to_end(&mut contents).await.unwrap();
        println!("len = {}", contents.len());
        Ok(())
    }

    let result = doit().await;
}

#[tokio::test]
async fn test_func2() {
    async fn doit() -> std::io::Result<()> {
        let mut file = File::create("foo.txt").await?; // 创建文件
        file.write_all(b"hello, world!").await?; // 写入内容
        Ok(())
    }

    let result = doit().await; // 注意这里的.await
}
