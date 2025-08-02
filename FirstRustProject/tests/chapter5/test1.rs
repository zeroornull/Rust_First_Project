#[test]
fn test_func1() {
    assert_eq!(2 + 2, 4);
}
#[test]
fn test_func5() {
    let rect1: Rectangle = Default::default();    // 使用方式1
    let rect2 = Rectangle::default();             // 使用方式2

    println!("{:?}", rect1);
    println!("{:?}", rect2);
}

#[test]
fn test_func4() {
    let rect1 = Rectangle::new(30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area1() // 使用点号操作符调用area方法
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area2() // 使用点号操作符调用area方法
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area3() // 使用点号操作符调用area方法
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        Rectangle::numbers(30, 50) // 使用点号操作符调用area方法
    );
}

#[derive(Debug, Default)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn numbers(rows: u32, cols: u32) -> u32 {
        rows * cols
    }

    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}
impl Rectangle {
    // fn can_hold(&self, other: &Rectangle) -> bool {
    //     self.width > other.width && self.height > other.height
    // }
}

impl Rectangle {
    fn area1(&self) -> u32 {
        self.width * self.height
    }

    fn area2(&self) -> u32 {
        self.width * self.height
    }

    fn area3(&self) -> u32 {
        self.width * self.height
    }
}

#[test]
fn test_func3() {
    // let active = true;
    // let username = String::from("someusername123");
    // let email = String::from("someone@example.com");
    // let user1 = User {
    //     active,
    //     username,
    //     email,
    //     sign_in_count: 1,
    // };
    //
    // // let email = user1.email; // 在这里发生了partially moved
    //
    // println!("{}", user1.username); // 分别打印另外3个字段
    // println!("{}", user1.active);
    // println!("{}", user1.sign_in_count);
    //
    // println!("{:?}", user1) // 这一句无法通过编译
}

// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u32,
// }

#[test]
fn test_func2() {
    // let user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     sign_in_count: 1,
    // };
}

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
