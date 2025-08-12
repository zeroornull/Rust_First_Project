#[test]
fn test_func1() {
    // 修复错误
    enum Number {
        Zero,
        One,
        Two,
    }

    enum Number1 {
        Zero = 0,
        One,
        Two,
    }

    // C语言风格的枚举定义
    // C-like enum
    enum Number2 {
        Zero = 0,
        One = 1,
        Two = 2,
    }
    // 通过 `as` 可以将枚举值强转为整数类型
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
}




// 填空
#[test]
fn test_func2() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg1 = Message::Move{x:1,y:2}; // 使用x = 1, y = 2 来初始化
    let msg2 = Message::Write(String::from("hello, world")); // instantiating with "hello, world!"
} 