use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use std::sync::atomic::AtomicU32;
#[tokio::test]
async fn test_func1() {
    assert_eq!(2 + 2, 4);
}

#[tokio::test]
async fn test_func9() {
    //
    let atomic_forty_two = AtomicU32::new(42);
    let arc_data = Arc::new(atomic_forty_two);

    let mut some_var = AtomicU32::new(10);
    *some_var.get_mut() = 5;
    assert_eq!(*some_var.get_mut(),5);
}

#[tokio::test]
async fn test_func8() {
    let lock = RwLock::new(5);
    // 多个读锁可以同时存在
    {
        let r1 = lock.read().await;
        let r2 = lock.read().await;
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // 在这一句结束时，两个读锁都释放掉了
    // 同时只能存在一个写锁
    {
        let mut w = lock.write().await;
        *w += 1;
        assert_eq!(*w, 6);
    } //在这一句结束时,写锁释放掉了
}

#[tokio::test]
async fn test_func7() {
    let db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let arc_db = Arc::new(Mutex::new(db)); // 加锁
    let arc_db2 = arc_db.clone();
    let arc_db3 = arc_db.clone();

    let task_a = tokio::task::spawn(async move {
        let mut db = arc_db.lock().await; // 获取锁
        db[4] = 50;
        assert_eq!(db[4], 50); // 校验值
    });
    let task_b = tokio::task::spawn(async move {
        let mut db = arc_db2.lock().await; // 获取锁
        db[4] = 100;
        assert_eq!(db[4], 100); // 校验值
    });
    _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();

    println!("{:?}", arc_db3.lock().await); // 获取锁
}

#[tokio::test]
async fn test_func6() {
    // let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let arc_db = Arc::new(db);
    // let arc_db2 = arc_db.clone();
    //
    // let task_a = tokio::task::spawn(async move {
    //     arc_db[4] = 50;
    // });
    // let task_b = tokio::task::spawn(async move { arc_db2[4] = 100 });
    // _ = task_a.await.unwrap();
    // _ = task_b.await.unwrap();
}

#[tokio::test]
async fn test_func5() {
    // let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //
    // let task_a = tokio::task::spawn(async move {
    //     db[4] = 50;
    // });
    // let task_b = task::spawn(async move {
    //     db[4] = 100;
    // });
    // _ = task_a.await.unwrap();
    // _ = task_b.await.unwrap();

    // println!("{:?}", db);
}

#[tokio::test]
async fn test_func4() {
    let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let task_a = tokio::task::spawn(async move {
        db[4] = 50;
    });
    _ = task_a.await.unwrap();

    // println!("{:?}", db);
}

#[test]
fn test_func3() {
    let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    db[4] = 50;
}

static mut DB: Vec<u32> = Vec::new();
#[test]
fn test_func2() {
    // DB.push(10);
}
