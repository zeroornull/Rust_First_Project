#[test]
fn test_func1() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_func18() {
    use std::sync::Arc;

    struct Atype;
    struct Btype;
    struct Ctype;

    trait TraitA {}

    impl TraitA for Atype {}
    impl TraitA for Btype {}
    impl TraitA for Ctype {}

    struct MyStruct {
        x: Arc<dyn TraitA>
    }

    let a = Atype;
    let t1 = MyStruct {x: Arc::new(a)};
    let b = Btype;
    let t2 = MyStruct {x: Arc::new(b)};
    let c = Ctype;
    let t3 = MyStruct {x: Arc::new(c)};
}

#[test]
fn test_func17() {
    use std::sync::Arc;

    #[derive(Debug)]
    struct Point {
        x: u32,
        y: u32,
    }
    impl Point{
        fn play_ref(&self) {
            println!("I'am play_ref of Point.");
        }
        fn play_mutref(&mut self) {
            println!("I'am play_mutref of Point.");
        }
        fn play_own(self) {
            println!("I'am play_own of Point.");
        }
        fn play_boxown(self: Box<Self>) {    // 注意这里
            println!("I'am play_boxown of Point.");
        }
        fn play_arcown(self: Arc<Self>) {    // 注意这里
            println!("I'am play_arcown of Point.");
        }
    }

    let mut boxed: Box<Point> = Box::new(Point{x: 10, y: 20});
    boxed.play_ref();
    boxed.play_mutref();
    boxed.play_boxown();
    // boxed.play_own();  // play_boxown()和 play_own() 只能同时打开一个

    let arced: Arc<Point> = Arc::new(Point{x: 10, y: 20});
    arced.play_ref();
    // arced.play_mutref();  // 不能用
    // arced.play_own();     // 不能用，Arc<T> 中的T无法被移出
    arced.play_arcown();
}

#[test]
fn test_func16() {
    use std::sync::Arc;

    #[derive(Debug)] // 这里不需要目标type实现Clone trait
    struct Point {
        x: u32,
        y: u32,
    }
    impl Point {
        fn play(&self) {
            println!("I'am a method of Point.");
        }
    }

    let arced: Arc<Point> = Arc::new(Point { x: 10, y: 20 });
    let another_arced = arced.clone();
    println!("{:?}", arced);
    println!("{:?}", another_arced);
    arced.play();
    another_arced.play();
    let arc3_ref = &another_arced;
    arc3_ref.play();
}

#[test]
fn test_func15() {
    struct Atype;
    struct Btype;
    struct Ctype;

    trait TraitA {}

    impl TraitA for Atype {}
    impl TraitA for Btype {}
    impl TraitA for Ctype {}

    struct MyStruct {
        x: Box<dyn TraitA>, // 结构体的字段类型是 Box<dyn TraitA>
    }

    let a = Atype;
    let t1 = MyStruct { x: Box::new(a) };
    let b = Btype;
    let t2 = MyStruct { x: Box::new(b) };
    let c = Ctype;
    let t3 = MyStruct { x: Box::new(c) };
}

#[test]
fn test_func14() {
    struct Atype;
    struct Btype;
    struct Ctype;

    trait TraitA {}

    impl TraitA for Atype {}
    impl TraitA for Btype {}
    impl TraitA for Ctype {}

    fn doit(x: Box<dyn TraitA>) {}

    let a = Atype;
    doit(Box::new(a));
    let b = Btype;
    doit(Box::new(b));
    let c = Ctype;
    doit(Box::new(c));
}

#[test]
fn test_func13() {
    struct Point {
        x: u32,
        y: u32,
    }
    struct Triangle {
        one: Box<Point>, // 三个字段类型都是 Box<Point>
        two: Box<Point>,
        three: Box<Point>,
    }
    let t = Triangle {
        one: Box::new(Point { x: 10, y: 10 }),
        two: Box::new(Point { x: 20, y: 20 }),
        three: Box::new(Point { x: 10, y: 20 }),
    };
}

#[test]
fn test_func12() {
    #[derive(Debug)]
    struct Point {
        x: u32,
        y: u32,
    }

    impl Point {
        fn play_ref(&self) {
            println!("I'am play_ref of Point.");
        }
        fn play_mutref(&mut self) {
            println!("I'am play_mutref of Point.");
        }
        fn play_own(self) {
            println!("I'am play_own of Point.");
        }
        fn play_boxown(self: Box<Self>) {
            // 注意这里
            println!("I'am play_boxown of Point.");
        }
    }

    let mut boxed: Box<Point> = Box::new(Point { x: 10, y: 20 });
    boxed.play_ref();
    boxed.play_mutref();
    boxed.play_boxown();
    // boxed.play_own();  // play_boxown()和 play_own() 只能同时打开一个
}

#[test]
fn test_func11() {
    #[derive(Debug)]
    struct Point {
        x: u32,
        y: u32,
    }

    impl Point {
        fn play(&self) {
            println!("I'am a method of Point.");
        }
    }

    let mut boxed: Box<Point> = Box::new(Point { x: 10, y: 20 });
    let y = &mut boxed;
    y.play();
    println!("{:?}", y); // 修改前的值
    **y = Point { x: 100, y: 200 }; // 注意这里用了二级解引用
    println!("{:?}", y); // 修改后的值
}

#[test]
fn test_func10() {
    #[derive(Debug)]
    struct Point {
        x: u32,
        y: u32,
    }
    impl Point {
        fn play(&self) {
            println!("I'am a method of Point.");
        }
    }
    let boxed: Box<Point> = Box::new(Point { x: 10, y: 20 });
    boxed.play(); // 调用类型方法
    let y = &boxed; // 取boxed实例的引用
    y.play(); // 调用类型方法
    println!("{:?}", y);
}
#[test]
fn test_func9() {
    #[derive(Debug)]
    struct Point {
        x: u32,
        y: u32,
    }

    fn foo(p: Box<Point>) {
        // 这里参数类型是 Box<Point>
        println!("{:?}", p);
    }

    foo(Box::new(Point { x: 10, y: 20 }));
}
#[test]
fn test_func8() {
    #[derive(Debug, Clone)]
    struct Point {
        x: u32,
        y: u32,
    }
    impl Point {
        fn play(&self) {
            println!("I'am a method of Point.");
        }
    }

    let mut boxed: Box<Point> = Box::new(Point { x: 10, y: 20 });
    let mut another_boxed = boxed.clone();
    *another_boxed = Point { x: 100, y: 200 };
    println!("{:?}", boxed);
    println!("{:?}", another_boxed);
}
#[test]
fn test_func7() {
    #[derive(Debug)]
    struct Point {
        x: u32,
        y: u32,
    }
    let mut boxed: Box<Point> = Box::new(Point { x: 10, y: 20 });
    *boxed = Point { x: 100, y: 200 };
    println!("{:?}", boxed);
}
#[test]
fn test_func6() {
    #[derive(Debug)]
    struct Point {
        x: u32,
        y: u32,
    }
    let p = Point { x: 10, y: 20 };
    let boxed: Box<Point> = Box::new(p);
    let val: Point = *boxed; // 这里做了解引用，Point实例回到栈上

    println!("{:?}", val);
    // println!("{:?}", boxed);  // 解引用后想把boxed再打印出来
}

#[test]
fn test_func5() {
    let boxed: Box<u8> = Box::new(5);
    let val: u8 = *boxed;

    println!("{:?}", val);
    println!("{:?}", boxed);
}

#[test]
fn test_func4() {
    fn foo() -> Box<u8> {
        let i = 5;
        let boxed = Box::new(i); // 创建Box实例
        let q = i; // 这一句用来检查i有没有被move走
        // let boxed2 = boxed;
        boxed
    }

    let _i = foo();
}
#[test]
fn test_func3() {
    struct Point {
        x: u32,
        y: u32,
    }

    fn foo() -> Box<Point> {
        let p = Point { x: 10, y: 20 };
        let boxed = Box::new(p); // 创建Box实例
        // let q = p;                // 这一句用来检查p有没有被move走
        boxed
    }

    let _p = foo();
}

#[test]
fn test_func2() {
    struct Point {
        x: u32,
        y: u32,
    }

    fn foo() -> Box<Point> {
        let p = Point { x: 10, y: 20 };
        Box::new(p)
    }

    let _p = foo();
}
