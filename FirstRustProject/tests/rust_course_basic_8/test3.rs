use std::fmt::{Debug, Display, Formatter, format, write};

#[test]
fn sample_func1() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    pub struct Post {
        pub title: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }

    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }

    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "好像微博没Tweet好用".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
}

#[test]
fn sample_func2() {
    pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }
    pub struct Post {
        pub title: String,
        pub author: String,
        pub content: String,
    }
    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    // impl Summary for Post {}

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "好像微博没Tweet好用".to_string(),
    };

    // println!("{}",post.summarize());
    println!("{}", weibo.summarize());
}

#[test]
fn sample_func3() {
    pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // pub fn notify(item: &impl Summary) {
    //     println!("Breaking news! {}", item.summarize());
    // }
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    pub fn notify1(item1: &impl Summary, item2: impl Summary) {}

    pub fn notify2<T: Summary>(item1: &T, item2: &T) {}

    pub fn notify3(item: &(impl Summary + Display)) {}

    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        1
    }

    fn some_function1<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        1
    }
}

#[test]
fn sample_func4() {
    use std::fmt::Display;
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

#[test]
fn sample_func5() {
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

#[test]
fn sample_func6() {
    use std::convert::TryInto;

    let a: i32 = 10;
    let b: u16 = 100;
    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.")
    }
}

#[test]
fn sample_func7() {
    use std::ops::Add;

    #[derive(Debug)]
    struct Point<T: Add<T, Output = T>> {
        x: T,
        y: T,
    }

    impl<T: Add<T, Output = T>> Add for Point<T> {
        type Output = Point<T>;

        fn add(self, p: Point<T>) -> Point<T> {
            Point {
                x: self.x + p.x,
                y: self.y + p.y,
            }
        }
    }

    fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
        a + b
    }

    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    println!("{:?}", add(p1, p2));

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", add(p3, p4));
}

#[test]
fn sample_func8() {
    #![allow(dead_code)]

    use std::fmt;
    use std::fmt::Display;

    #[derive(Debug, PartialEq)]
    enum FileState {
        Open,
        Closed,
    }

    #[derive(Debug)]
    struct File {
        name: String,
        data: Vec<u8>,
        state: FileState,
    }

    impl Display for FileState {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                FileState::Open => write!(f, "OPEN"),
                FileState::Closed => write!(f, "CLOSED"),
            }
        }
    }

    impl Display for File {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "<{} ({})>", self.name, self.state)
        }
    }

    impl File {
        fn new(name: &str) -> File {
            File {
                name: String::from(name),
                data: Vec::new(),
                state: FileState::Closed,
            }
        }
    }

    let f6 = File::new("f6.txt");
    //...
    println!("{:?}", f6);
    println!("{}", f6);
}

#[test]
fn sample_func9() {
    struct Sheep {
        naked: bool,
        name: String,
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }
        fn shear(&mut self) {
            if self.is_naked() {
                // `Sheep` 结构体上定义的方法可以调用 `Sheep` 所实现的特征的方法
                println!("{} is already naked...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }

    trait Animal {
        // 关联函数签名；`Self` 指代实现者的类型
        // 例如我们在为 Pig 类型实现特征时，那 `new` 函数就会返回一个 `Pig` 类型的实例，这里的 `Self` 指代的就是 `Pig` 类型
        fn new(name: String) -> Self;

        // 方法签名
        fn name(&self) -> String;

        fn noise(&self) -> String;

        // 方法还能提供默认的定义实现
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Animal for Sheep {
        fn new(name: String) -> Sheep {
            Sheep {
                name: name,
                naked: false,
            }
        }

        fn name(&self) -> String {
            self.name.clone()
        }

        fn noise(&self) -> String {
            if self.is_naked() {
                "baaaaah?".to_string()
            } else {
                "baaaah!".to_string()
            }
        }

        // 默认的特征方法可以被重写
        fn talk(&self) {
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }

    let mut dolly: Sheep = Animal::new("Dolly".to_string());
    dolly.talk();
    dolly.shear();
    dolly.talk();
}

#[test]
fn test_func1() {
    // 完成两个 `impl` 语句块
    // 不要修改 `main` 中的代码
    trait Hello {
        fn say_hi(&self) -> String {
            String::from("hi")
        }

        fn say_something(&self) -> String;
    }

    struct Student {}
    impl Hello for Student {
        fn say_hi(&self) -> String {
            "hi".to_string()
        }

        fn say_something(&self) -> String {
            "I'm a good student".to_string()
        }
    }
    struct Teacher {}
    impl Hello for Teacher {
        fn say_hi(&self) -> String {
            "Hi, I'm your new teacher".to_string()
        }

        fn say_something(&self) -> String {
            "I'm not a bad teacher".to_string()
        }
    }

    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!")
}

#[test]
fn test_func2() {
    // `Centimeters`, 一个元组结构体，可以被比较大小
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    // `Inches`, 一个元组结构体可以被打印
    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;
            Centimeters(inches as f64 * 2.54)
        }
    }

    // 添加一些属性让代码工作
    // 不要修改其它代码！
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Seconds(i32);

    let _one_second = Seconds(1);

    println!("One second looks like: {:?}", _one_second);
    let _this_is_true = _one_second == _one_second;
    let _this_is_false = _one_second > _one_second;

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} than one meter.", cmp);
}

#[test]
fn test_func3() {
    use std::ops;
    // 实现 fn multiply 方法
    // 如上所述，`+` 需要 `T` 类型实现 `std::ops::Add` 特征
    // 那么, `*` 运算符需要实现什么特征呢? 你可以在这里找到答案: https://doc.rust-lang.org/core/ops/
    fn multiply<T: ops::Mul<Output = T>>(x: T, y: T) -> T {
        x * y
    }
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!")
}

#[test]
fn test_func4() {
    use std::ops;

    struct Foo;
    struct Bar;

    #[derive(PartialEq, Debug)]
    struct FooBar;

    #[derive(PartialEq, Debug)]
    struct BarFoo;

    // 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            FooBar
        }
    }

    impl ops::Sub<Bar> for Foo {
        type Output = BarFoo;

        fn sub(self, _rhs: Bar) -> BarFoo {
            BarFoo
        }
    }

    // 不要修改下面代码
    // 你需要为 FooBar 派生一些特征来让代码工作
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);

    println!("Success!")
}

#[test]
fn test_func5() {
    // 实现 `fn summary`
    // 修复错误且不要移除任何代码行
    trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug)]
    struct Post {
        title: String,
        author: String,
        content: String,
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("The author of post {} is {}", self.title, self.author)
        }
    }

    #[derive(Debug)]
    struct Weibo {
        username: String,
        content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{} published a weibo {}", self.username, self.content)
        }
    }

    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    fn summary(t: &impl Summary) {
        let _ = t.summarize();
    }

    summary(&post);
    summary(&weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}

#[test]
fn test_func6() {
    struct Sheep {}
    struct Cow {}

    trait Animal {
        fn noise(&self) -> String;
    }

    impl Animal for Sheep {
        fn noise(&self) -> String {
            "baaaaah!".to_string()
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> String {
            "moooooo!".to_string()
        }
    }

    // 返回一个类型，该类型实现了 Animal 特征，但是我们并不能在编译期获知具体返回了哪个类型
    // 修复这里的错误，你可以使用虚假的随机，也可以使用特征对象
    fn random_animal(random_number: f64) -> impl Animal {
        if random_number < 0.5 {
            Sheep {}
        } else {
            Sheep {}
        }
    }

    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}

#[test]
fn test_func6_2() {
    struct Sheep {}
    struct Cow {}

    trait Animal {
        fn noise(&self) -> String;
    }

    impl Animal for Sheep {
        fn noise(&self) -> String {
            "baaaaah!".to_string()
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> String {
            "moooooo!".to_string()
        }
    }

    // Returns some struct that implements Animal, but we don't know which one at compile time.
    // FIX the erros here, you can make a fake random, or you can use trait object
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}

#[test]
fn test_func7_1() {
    assert_eq!(sum(1, 2), 3);

    // 通过两种方法使用特征约束来实现 `fn sum`
    fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
        x + y
    }
}

#[test]
fn test_func7_2() {
    assert_eq!(sum(1, 2), 3);
    assert_eq!(sum(1.0, 2.0), 3.0);

    fn sum<T>(x: T, y: T) -> T
    where
        T: std::ops::Add<Output = T>,
    {
        x + y
    }
}



#[test]
fn test_func8() {

    // 修复代码中的错误
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {:?}", self.x);
            } else {
                println!("The largest member is y = {:?}", self.y);
            }
        }
    }

    #[derive(Debug, PartialEq, PartialOrd)]
    struct Unit(i32);

    let pair = Pair{
        x: Unit(1),
        y: Unit(3)
    };

    pair.cmp_display();
}

#[test]
fn test_func9_1() {
    // `T: Trait` 是最常使用的方式
    // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(10), 11);
    assert_eq!(cacher.value(15), 11);
}

#[test]
fn test_func9_2() {
    // 还可以使用 `where` 来约束 T
    struct Cacher<T>
    where T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(20), 21);
    assert_eq!(cacher.value(25), 21);
}

