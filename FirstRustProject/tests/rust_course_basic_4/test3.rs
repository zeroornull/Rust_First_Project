
// 填空并修复错误
// 1. 不要使用 `to_string()`
// 2. 不要添加/删除任何代码行
#[test]
fn test_func1() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership(s.clone());

    assert_eq!(s, "hello, world!");

    println!("Success!");
    fn move_ownership(s: String) {
        println!("ownership of \"{}\" is moved here!", s)
    }
}

// 填空
#[test]
fn test_func2() {
    let mut s = String::from("hello, world");

    let slice1: &str = &s; // 使用两种方法
    assert_eq!(slice1, "hello, world");

    let slice2 = &s[0..5];
    assert_eq!(slice2, "hello");

    let slice3: &mut String = & mut s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("Success!")
}


// 问题:  我们的代码中发生了多少次堆内存分配？
// 你的回答: 
#[test]
fn test_func3() {
    // 基于 `&str` 类型创建一个 String,
    // 字符串字面量的类型是 `&str`
    let s: String = String::from("hello, world!");

    // 创建一个切片引用指向 String `s`
    let slice: &str = &s;

    // 基于刚创建的切片来创建一个 String
    let s: String = slice.to_string();

    assert_eq!(s, "hello, world!");

    println!("Success!")
}


// 修改下面的代码以打印如下内容: 
// 25
// 25
// 25
// 循环中不会发生任何内存分配
#[test]
fn test_func6() {
    let mut s = String::with_capacity(25);


    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!")
}


// 填空


#[test]
fn test_func7() {
    use std::mem;
    let story = String::from("Rust By Practice");

    // 阻止 String 的数据被自动 drop
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(16, len);

    // 我们可以基于 ptr 指针、长度和容量来重新构建 String. 
    // 这种操作必须标记为 unsafe，因为我们需要自己来确保这里的操作是安全的
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!")
}