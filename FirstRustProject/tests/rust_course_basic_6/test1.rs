#[test]
fn sample_func1() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

#[test]
fn sample_func2() {
    enum IpAddr {
        Ipv4,
        Ipv6,
    }
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);
}

#[test]
fn test_func1() {
    // 填空
    enum Direction {
        East,
        West,
        North,
        South,
    }
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => {
            // 在这里匹配 South 或 North
            println!("South or North");
        }
        _ => println!("West"),
    };
}

#[test]
fn test_func2() {
    let boolean = true;

    // 使用 match 表达式填空，并满足以下条件
    //
    // boolean = true => binary = 1
    // boolean = false => binary = 0
    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);
}

// 填空

#[test]
fn test_func3() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    fn show_message(msg: Message) {
        match msg {
            Message::Move { x: a, y: b } => {
                // 这里匹配 Message::Move
                assert_eq!(a, 1);
                assert_eq!(b, 3);
            }
            Message::ChangeColor(_, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            __ => println!("no data in these variants"),
        }
    }

    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg)
    }
}

#[test]
fn test_func4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    // 使用 `matches!` 填空
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }
}

#[test]
fn test_func5() {
    enum MyEnum {
        Foo,
        Bar,
    }
    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) {
            // 修复错误，只能修改本行代码
            count += 1;
        }
    }

    assert_eq!(count, 2);
}

#[test]
fn test_func6() {
    let o = Some(7);

    // 移除整个 `match` 语句块，使用 `if let` 替代

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
    }
}


// 填空


#[test]
fn test_func7() {
    enum Foo {
        Bar(u8)
    }
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar 持有的值是: {}", i);
    }
}




#[test]
fn test_func8() {
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }
    let a = Foo::Qux(10);

    // 移除以下代码，使用 `match` 代替
    // if let Foo::Bar = a {
    //     println!("match foo::bar")
    // } else if let Foo::Baz = a {
    //     println!("match foo::baz")
    // } else {
    //     println!("match others")
    // }
    
    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        _ =>  println!("match others")
    }
}


// 就地修复错误
#[test]
fn test_func9() {
    let age = Some(30);
    if let Some(age) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
        assert_eq!(age, 30);
    } // 新的 `age` 变量在这里超出作用域

    match age {
        // `match` 也能实现变量遮蔽
        Some(age) =>  println!("age 是一个新的变量，它的值是 {}",age),
        _ => ()
    }
}
