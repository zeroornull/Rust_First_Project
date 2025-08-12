#[test]
fn sample_func1() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}


#[test]
fn test_func1() {
    let x = 5;
    // 填写空白处
    let p = &x;

    println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
}


#[test]
fn test_func2() {
    let x = 5;
    let y = &x;

    // 只能修改以下行
    assert_eq!(5, *y);
}


// 修复错误
#[test]
fn test_func3() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    fn borrow_object(s: &String) {}
}



// 修复错误
#[test]
fn test_func4() {
    let mut s = String::from("hello, ");

    push_str(&mut s);
    fn push_str(s: &mut String) {
        s.push_str("world")
    }
}


#[test]
fn test_func5() {
    let mut s = String::from("hello, ");

    // 填写空白处，让代码工作
    let p = &mut s;

    p.push_str("world");
}


#[test]
fn test_func6() {
    let c = '中';

    let r1 = &c;
    // 填写空白处，但是不要修改其它行的代码
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // 判断两个内存地址的字符串是否相等
    assert_eq!(get_addr(r1),get_addr(r2));

    // 获取传入引用的内存地址的字符串形式
    fn get_addr(r: &char) -> String {
        format!("{:p}", r)
    }
}



// 移除代码某个部分，让它工作
// 你不能移除整行的代码！
#[test]
fn test_func7() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);
}


#[test]
fn test_func8() {
    // 通过修改下面一行代码来修复错误
    let  mut s = String::from("hello, ");

    borrow_object(&mut s);

    fn borrow_object(s: &mut String) {}
}


// 下面的代码没有任何错误
#[test]
fn test_func9() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    fn borrow_object(s: &String) {}
}


// 注释掉一行代码让它工作
#[test]
fn test_func10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    // println!("{}",r1);
}


#[test]
fn test_func11() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
    // 你不能同时使用 r1 和 r2
    // println!("{}, {}", r1, r2);
}
