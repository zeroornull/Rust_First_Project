

#[test]
fn test_func9() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x,y], [3,2]);
}
// 修复下面代码的错误并尽可能少的修改
#[test]
fn test_func8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}
#[test]
fn test_func7() {
    let _x = 1;
}

#[allow(unused_variables)]
#[test]
fn test_func7_2() {
    let x = 1;
}

// compiler warning: unused variable: `x`
#[test]
fn test_func6() {
    let mut x: i32 = 1;
    x = 7;
    // 遮蔽且再次绑定
    let mut x = x;
    x += 3;


    let y = 4;
    // 遮蔽
    let y = "I can also be bound to text!";
}
#[test]
fn test_func5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // 输出 "42".
}

#[test]
fn test_func4() {
    fn define_x() {
        let x = "hello";
        println!("{}, world", x);
    }
    
}

#[test]
fn test_func3() {
    let x: i32 = 10;
    let y: i32 = 5;

    {
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}, y 的值是 {}", x, y);
}
#[test]
fn test_func2() {
    let mut x = 1;
    x += 2;

    println!("x = {}", x);
}
#[test]
fn test_func1() {
    let x: i32 = 1; // 未初始化，但被使用
    let y: i32; // 未初始化，也未被使用
    println!("x is equal to {}", x);
}
