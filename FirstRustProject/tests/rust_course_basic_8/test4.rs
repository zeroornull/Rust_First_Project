#[test]
fn sample_func1() {
    trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }

    // 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
    fn draw1(x: Box<dyn Draw>) {
        // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
        x.draw();
    }

    fn draw2(x: &dyn Draw) {
        x.draw();
    }

    let x = 1.1f64;
    // do_something(&x);
    let y = 8u8;

    // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    draw1(Box::new(x));
    // 基于 y 的值创建一个 Box<u8> 类型的智能指针
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
}

#[test]
fn sample_func2() {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // 绘制按钮的代码
        }
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            // 绘制SelectBox的代码
        }
    }
    pub struct Screen<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> Screen<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    // let screen = Screen {
    //     components: vec![
    //         Box::new(SelectBox {
    //             width: 75,
    //             height: 10,
    //             options: vec![
    //                 String::from("Yes"),
    //                 String::from("Maybe"),
    //                 String::from("No")
    //             ],
    //         }),
    //         Box::new(Button {
    //             width: 50,
    //             height: 10,
    //             label: String::from("OK"),
    //         }),
    //     ],
    // };
    //
    // screen.run();
}

#[test]
fn sample_func3() {
    trait Draw {
        fn draw(&self) -> Self;
    }

    #[derive(Clone)]
    struct Button;
    impl Draw for Button {
        fn draw(&self) -> Self {
            return self.clone();
        }
    }

    let button = Button;
    let newb = button.draw();
}

#[test]
fn test_func1() {
    trait Bird {
        fn quack(&self) -> String;
    }

    struct Duck;
    impl Duck {
        fn swim(&self) {
            println!("Look, the duck is swimming")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }

    impl Bird for Duck {
        fn quack(&self) -> String {
            "duck duck".to_string()
        }
    }

    impl Bird for Swan {
        fn quack(&self) -> String {
            "swan swan".to_string()
        }
    }

    fn main() {
        // 填空
        let duck = Duck;
        duck.swim();

        let bird = hatch_a_bird(2);
        // 变成鸟儿后，它忘记了如何游，因此以下代码会报错
        // bird.swim();
        // 但它依然可以叫唤
        assert_eq!(bird.quack(), "duck duck");

        let bird = hatch_a_bird(1);
        // 这只鸟儿忘了如何飞翔，因此以下代码会报错
        // bird.fly();
        // 但它也可以叫唤
        assert_eq!(bird.quack(), "swan swan");

        println!("Success!")
    }

    // 实现以下函数
    fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
        if species == 1 {
            Box::new(Swan {})
        } else {
            Box::new(Duck {})
        }
    }
}

#[test]
fn test_func2() {
    trait Bird {
        fn quack(&self);
    }

    struct Duck;
    impl Duck {
        fn fly(&self) {
            println!("Look, the duck is flying")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }

    impl Bird for Duck {
        fn quack(&self) {
            println!("{}", "duck duck");
        }
    }

    impl Bird for Swan {
        fn quack(&self) {
            println!("{}", "swan swan");
        }
    }

    // 填空
    let birds: [Box<dyn Bird>; 2] = [Box::new(Duck), Box::new(Swan)];

    for bird in birds {
        bird.quack();
        // 当 duck 和 swan 变成 bird 后，它们都忘了如何翱翔于天际，只记得该怎么叫唤了。。
        // 因此，以下代码会报错
        // bird.fly();
    }
}

#[test]
fn test_func3() {
    // 填空
    trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }

    let x = 1.1f64;
    let y = 8u8;

    // draw x
    draw_with_box(Box::new(x));

    // draw y
    draw_with_ref(&y);

    println!("Success!");

    fn draw_with_box(x: Box<dyn Draw>) {
        x.draw();
    }

    fn draw_with_ref(x: &dyn Draw) {
        x.draw();
    }
}

#[test]
fn test_func4() {
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Foo for String {
        fn method(&self) -> String {
            format!("string: {}", *self)
        }
    }

    // 通过泛型实现以下函数
    fn static_dispatch<T: Foo>(x: T) {
        x.method();
    }

    // 通过特征对象实现以下函数
    fn dynamic_dispatch(x: &dyn Foo) {
        x.method();
    }

    fn main() {
        let x = 5u8;
        let y = "Hello".to_string();

        static_dispatch(x);
        dynamic_dispatch(&y);

        println!("Success!")
    }

    main();
}

#[test]
fn test_func5_1() {
    // 使用至少两种方法让代码工作
    // 不要添加/删除任何代码行
    trait MyTrait {
        fn f(&self) -> Self;
    }

    impl MyTrait for u32 {
        fn f(&self) -> u32 {
            42
        }
    }

    impl MyTrait for String {
        fn f(&self) -> String {
            self.clone()
        }
    }

    fn my_function(x:impl  MyTrait) -> impl MyTrait {
        x.f()
    }

    fn main() {
        my_function(13_u32);
        my_function(String::from("abc"));

        println!("Success!")
    }

    main();
}

#[test]
fn test_func5_2() {

    // 使用至少两种方法让代码工作
    // 不要添加/删除任何代码行
    trait MyTrait {
        fn f(&self) -> Box<dyn MyTrait>;
    }

    impl MyTrait for u32 {
        fn f(&self) -> Box<dyn MyTrait> { Box::new(42) }
    }

    impl MyTrait for String {
        fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }
    }

    fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait>  {
        x.f()
    }

    fn main() {
        my_function(Box::new(13_u32));
        my_function(Box::new(String::from("abc")));

        println!("Success!")
    }

    main();
}
