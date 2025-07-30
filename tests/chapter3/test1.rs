#[test]
fn test_func1() {
    assert_eq!(2 + 2, 4);
}
#[test]
fn test_func16() {
    // let mut a: u32 = 10;
    // let b = &a;
    // a = 20;
    //
    // println!("{}", b);
}
#[test]
fn test_func15() {
    let mut s1 = String::from("I am a superman.");
    println!("{s1}");
    foo2(&mut s1); // 注意这里传的是字符串的可变引用 &mut s1
    println!("{s1}");
}

fn foo2(s: &mut String) {
    s.push_str(" You are batman.");
}
#[test]
fn test_func14() {
    let s1 = String::from("I am a superman.");
    foo1(&s1); // 注意这里传的是字符串的引用 &s1
    println!("{s1}"); // 这里可以打印s1的值了
}
fn foo1(s: &String) {
    println!("in fn foo: {s}");
}

#[test]
fn test_func13() {
    // let mut a1 = 10u32;
    // let mut b = &mut a1;
    // *b = 20;
    //
    // let c = &mut b;
    // *c = 30;            // 这里对二级可变引用只使用一级解引用操作
    //
    // println!("{c}");
}
#[test]
fn test_func12() {
    let mut a1 = 10u32;
    let mut b = &mut a1;
    *b = 20;

    let c = &mut b;
    **c = 30; // 多级解引用操作

    println!("{c}");
}
#[test]
fn test_func11() {
    // let mut a1 = 10u32;
    // let mut a2 = 15u32;
    //
    // let mut b = &mut a1;
    // b = &mut a2;
    //
    // let mut c = &a1;
    // c = &a2;
}

#[test]
fn test_func10() {
    let mut a = 10u32;
    let r1 = &mut a;
    let r2 = r1;

    println!("{r2}"); // 打印r2
}
#[test]
fn test_func9() {
    // let mut a = 10u32;
    // let r1 = &mut a;
    // let r2 = r1;
    //
    // println!("{r1}")
}
#[test]
fn test_func8() {
    // let mut a = 10u32;
    // let b = &mut a;
    // *b = 20;
    // let c = &a;      // 在利用b更新了a的值后，c再次借用a
}

#[test]
fn test_func7() {
    // let mut a = 10u32;
    // let b = &mut a;
    // *b = 20;
    //
    // println!("{a}");    // 这里多打印了一行a
    // println!("{b}");
}
#[test]
fn test_func6() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;

    println!("{b}");
    println!("{a}"); // 这里多打印了一行a
}
#[test]
fn test_func5() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;

    println!("{b}");
}

#[test]
fn test_func4() {
    let s1 = String::from("I am a superman.");
    let s2 = &s1;
    let s3 = &&&&&s1;
    let s4 = &s2;
    let s5 = s2;

    println!("{s1}");
    println!("{s2}");
    println!("{s3}");
    println!("{s4}");
    println!("{s5}");
}

#[test]
fn test_func3() {
    let a = 10u32;
    let b = &a; // b是变量a的一级引用
    let c = &&&&&a; // c是变量a的多级引用
    let d = &b; // d是变量a的间接引用
    let e = b; // 引用b再赋值给e

    println!("{a}");
    println!("{b}");
    println!("{c}");
    println!("{d}");
    println!("{e}");
}

fn foo(s: String) -> String {
    println!("{s}");
    s
}
#[test]
fn test_func2() {
    let s1 = String::from("I am a superman.");
    let s1 = foo(s1);
    println!("{s1}");
}
