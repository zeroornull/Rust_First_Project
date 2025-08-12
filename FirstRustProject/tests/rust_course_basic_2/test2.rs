#[test]
fn sample_func1() {
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", size_of_val(&x));
}

// 修改2处 `assert_eq!` 让代码工作

use std::mem::size_of_val;
#[test]
fn test_func1() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!")
}

// 修改一行让代码正常打印
#[test]
fn test_func2() {

    fn print_char(c: char) {
        println!("{}", c);
    }
    let c1 = '中';
    print_char(c1);

}


// 使成功打印
#[test]
fn test_func3() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success!")
    }
}



#[test]
fn test_func4() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    println!("Success!")
}


// 让代码工作，但不要修改 `implicitly_ret_unit` !
#[test]
fn test_func5() {
    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit());

    fn implicitly_ret_unit() {
        println!("I will return a ()")
    }

    // don't use this one
    fn explicitly_ret_unit() -> () {
        println!("I will return a ()")
    }
}


// 让代码工作：修改 `assert!` 中的 `4`
#[test]
fn test_func6() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!")
}