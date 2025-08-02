#[test]
fn test_func1() {
    assert_eq!(2 + 2, 4);
}

fn foo(User { name, age, student }: User) {
    println!("{}", name);
    println!("{}", age);
    println!("{}", student);
}

#[test]
fn test_func15() {
    let a = User {
        name: String::from("mike"),
        age: 20,
        student: false,
    };
    foo(a);
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    student: bool,
}

// #[test]
// fn test_func14() {
//     let a = (1,2,'a');
//     foo(a)
// }
//
// fn foo((a, b, c): (u32, u32, char)) {
//     println!("a = {}, b = {}, c = {}", a, b, c);
// }

// #[test]
// fn test_func13() {
//     let a = User {
//         name: String::from("mike"),
//         age: 20,
//         student: false,
//     };
//     let User {
//         ref name,
//         age,
//         student,
//     } = a;
//
//     println!("{}", name);
//     println!("{}", age);
//     println!("{}", student);
//     println!("{:?}", a);
// }
//
// #[derive(Debug)]
// struct User {
//     name: String,
//     age: u32,
//     student: bool,
// }

// #[test]
// fn test_func12() {
//     let a_rec = Rectangle {
//         width: 10,
//         height: 20,
//     };
//     // 请打开下面这一行进行实验
//     // let shape_a = Shape::Rectangle(a_rec);
//     // 请打开下面这一行进行实验
//     // let shape_a = Shape::Triangle((0, 1), (3,4), (3, 0));
//     let shape_a = Shape::Circle {
//         origin: (0, 0),
//         radius: 5,
//     };
//
//     // 这里演示了在模式匹配中将枚举的负载解出来的各种形式
//     match shape_a {
//         Shape::Rectangle(a_rec) => {
//             // 解出一个结构体
//             println!("Rectangle {}, {}", a_rec.width, a_rec.height);
//         }
//         Shape::Triangle(x, y, z) => {
//             // 解出一个元组
//             println!("Triangle {:?}, {:?}, {:?}", x, y, z);
//         }
//         Shape::Circle { origin, radius } => {
//             // 解出一个结构体的字段
//             println!("Circle {:?}, {:?}", origin, radius);
//         }
//     }
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// enum Shape {
//     Rectangle(Rectangle),
//     Triangle((u32, u32), (u32, u32), (u32, u32)),
//     Circle { origin: (u32, u32), radius: u32 },
// }
// #[test]
// fn test_func11() {
//     let (b,c,d) = foo();
//
//     println!("{}", b);
//     println!("{}", c);
//     println!("{}", d);
// }
//
//
//
// fn foo() -> (u32, u32, char) {
//     (1,2,'a')
// }
// #[test]
// fn test_func10() {
//     let a = (1,2,'a');
//     let (b,c,d) = a;
//     println!("{:?}", a);
//     println!("{}", b);
//     println!("{}", c);
//     println!("{}", d);
// }
// #[test]
// fn test_func9() {
//     // 创建实例
//     let shape_a = Shape::Rectangle {
//         width: 10,
//         height: 20,
//     };
//     // 模式匹配出负载内容
//     let Shape::Rectangle { width, height } = shape_a else {
//         panic!("Can't extract rectangle.");
//     };
//     println!("width: {}, height: {}", width, height);
// }
//
// #[derive(Debug)]
// enum Shape {
//     Rectangle { width: u32, height: u32 },
//     Triangle,
//     Circle,
// }

// #[test]
// fn test_func8() {
//     let mut shape_a = Shape::Rectangle;
//     let mut i = 0;
//     while let Shape::Rectangle = shape_a {
//         if i > 9 {
//             println!("Grater than 9,quit!");
//             shape_a = Shape::Circle;
//         } else {
//             println!("`i` is `{:?}`. Try again.", i);
//             i += 1;
//         }
//     }
// }
//
// #[test]
// fn test_func7() {
//     let number = 13;
//     // 你可以试着修改上面的数字值，看看下面走哪个分支
//
//     println!("Tell me about {}", number);
//     match number {
//         // 匹配单个数字
//         1 => println!("One!"),
//         // 匹配几个数字
//         2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
//         // 匹配一个范围，左闭右闭区间
//         13..=19 => println!("A teen"),
//         // 处理剩下的情况
//         _ => println!("Ain't special"),
//     }
// }
//
// #[test]
// fn test_func6() {
//     let shape_a = Shape::Rectangle; // 创建实例
//     let ret = match shape_a {
//         Shape::Rectangle => 1,
//         _ => 10,
//     };
//     println!("{}", ret);
// }
// #[test]
// fn test_func5() {
//     let shape_a = Shape::Rectangle; // 创建实例
//     let ret = match shape_a {
//         // 匹配实例，并返回结果给ret
//         Shape::Rectangle => 1,
//         Shape::Triangle => 2,
//         Shape::Circle => 3,
//     };
//     println!("{}", ret);
// }
//
// #[test]
// fn test_func4() {
//     let shape_a = Shape::Rectangle;
//     match shape_a {
//         Shape::Rectangle => {
//             println!("{:?}", Shape::Rectangle); // 进了这个分支
//         }
//         Shape::Triangle => {
//             println!("{:?}", Shape::Triangle);
//         }
//         Shape::Circle => {
//             println!("{:?}", Shape::Circle);
//         }
//     }
// }
//
// #[derive(Debug)]
// enum Shape {
//     Rectangle,
//     Triangle,
//     Circle,
// }
// #[test]
// fn test_func3() {
//     // 实例化枚举
//     let add = MyEnum::Add;
//     // 实例化后执行枚举的方法
//     add.run(100, 200);
// }
//
// impl MyEnum {
//     fn run(&self, x: i32, y: i32) -> i32 {
//         match self {
//             Self::Add => x + y,
//             Self::Subtract => x - y,
//         }
//     }
// }
// enum MyEnum {
//     Add,
//     Subtract,
// }

// #[test]
// fn test_func2() {
//     // 使用 as 进行类型的转化
//     println!("zero is {}", Number::Zero as i32);
//     println!("one is {}", Number::One as i32);
//
//     println!("roses are #{:06x}", Color::Red as i32);
//     println!("violets are #{:06x}", Color::Blue as i32);
// }
//
// enum Number {
//     Zero = 0,
//     One,
//     Two,
// }

// 给枚举每个变体赋予不同的值
// enum Color {
//     Red = 0xff0000,
//     Green = 0x00ff00,
//     Blue = 0x0000ff,
// }
