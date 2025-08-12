use num::Complex;

#[test]
fn test_func1() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}

// 移除某个部分让代码工作
#[test]
fn test_func2() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let z = 10; // 这里 z 的类型是? 
}


// 填空
#[test]
fn test_func3() {
    let v: u16 = 38_u8 as u16;
}


// 修改 `assert_eq!` 让代码工作
#[test]
fn test_func4() {
    // 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
}


// 填空，让代码工作
#[test]
fn test_func5() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
}


// 解决代码中的错误和 `panic`
#[test]
fn test_func6() {
    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{},{}",v1,v2);
}


// 修改 `assert!` 让代码工作
#[test]
fn test_func6_1() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
}


// 将 ? 替换成你的答案
#[test]
fn test_func7() {
    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
}

// 使用两种方法来让下面代码工作
#[test]
fn test_func8() {
    // assert!(0.1+0.2==0.3);
    assert!(0.1_f32+0.2_f32==0.3_f32);
    assert!((0.1_f64+ 0.2 - 0.3).abs() < 0.001);
}

#[test]
fn test_func9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}


// 填空
use std::ops::{Range, RangeInclusive};
#[test]
fn test_func10() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}


// 填空，并解决错误
#[test]
fn test_func11() {
    // 整数加法
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9 / 3 == 3); // error ! make it work // error ! 修改它让代码工作

    assert!(24 % 5 == 4);

    // 逻辑与或非操作
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}