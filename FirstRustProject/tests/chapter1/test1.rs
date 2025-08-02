#[test]
fn test_func1() {
    assert_eq!(2 + 2, 4);
}
#[test]
fn test_func18() {
    let s1 = String::from("I am a superman.");
    let s2 = s1;
    //println!("{s1}");
    println!("{s2}");
}
#[test]
fn test_func17() {
    let s1 = String::from("I am a superman.");
    let s2 = s1.clone();
    println!("{s1}");
    println!("{s2}");
}
#[test]
fn test_func16() {
    let a = 10u32;
    let b = a;
    println!("{a}");
    println!("{b}");
}
#[test]
fn test_func15() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
#[test]
fn test_func14() {
    // let a = 10u32;
    let a = 'a';
    println!("{}", a);
}

#[test]
fn test_func13() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;    // 注意这里，重新使用了 let 来定义新变量
    println!("The value of x is: {x}");
}

#[test]
fn test_func12() {
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
}

// #[test]
// fn test_func11() {
//       let a = 10u32;             // 局部变量
//
//       fn add_v1 (x: u32) -> u32 { x + a }    // 定义一个内部函数
//       let add_v2 = |x: u32| -> u32 { x + a };   // 定义一个闭包
//
//       let result1 = add_v1(20);  // 调用函数
//       let result2 = add_v2(20);  // 调用闭包
//       println!("{}", result2);
// }

#[test]
fn test_func10() {
    // // 标准的函数定义
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }

    // // 闭包的定义，请注意形式对比
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };

    // // 闭包的定义2，省略了类型标注
    // let add_one_v3 = |x|             { x + 1 };

    // // 闭包的定义3，花括号也省略了
    // let add_one_v4 = |x|              x + 1  ;
    //
    // let add_one = |x| x + 1;
    // let a_vec: Vec<u32> = vec![1, 2, 3, 4, 5];
    // let vec2: Vec<_> = a_vec.iter().map(add_one).collect();
}

#[test]
fn test_func9() {
    print_a_b(5, 'h');
}

fn print_a_b(a: i32, b: char) {
    println!("The value of a b is: {a}{b}");
}
#[test]
fn test_func8() {
    for ch in 'a'..='z' {
        println!("{ch}");
    }
}

#[test]
fn test_func7() {
    // 左闭右开区间
    for number in 1..4 {
        println!("{number}");
    }
    println!("--");
    // 左闭右闭区间
    for number in 1..=4 {
        println!("{number}");
    }
    println!("--");
    // 反向
    for number in (1..4).rev() {
        println!("{number}");
    }
}

#[test]
fn test_func6() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

#[test]
fn test_func5() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

#[test]
fn test_func4() {
    let mut counter = 0;

    // 这里，接收从循环体中返回的值，对result进行初始化
    let result = loop {
        counter += 1;
        if counter == 10 {
            // 使用break跳出循环，同时带一个返回值回去
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

#[test]
fn test_func3() {
    let x = 1;
    // 在这里，if else 返回了值
    let y = if x == 0 {
        // 代码块结尾最后一句不加分号，表示把值返回回去
        100
    } else {
        // 代码块结尾最后一句不加分号，表示把值返回回去
        101
    };
    println!("y is {}", y);
}

#[test]
fn test_func2() {
    let number = 6;
    // 判断数字number能被4，3，2中的哪一个数字整除
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
