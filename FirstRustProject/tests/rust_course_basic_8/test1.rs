#[test]
fn sample_func1() {
    // fn add<T>(a:T, b:T) -> T {
    //     a + b
    // }
    // println!("add i8: {}", add(2i8, 3i8));
    // println!("add i32: {}", add(20, 30));
    // println!("add f64: {}", add(1.23, 1.23));
}

#[test]
fn test_func1() {
    // 填空
    struct A; // 具体的类型 `A`.
    struct S(A); // 具体的类型 `S`.
    struct SGen<T>(T); // 泛型 `SGen`.

    fn reg_fn(_s: S) {}

    fn gen_spec_t(_s: SGen<A>) {}

    fn gen_spec_i32(_s: SGen<i32>) {}

    fn generic<T>(_s: SGen<T>) {}
    // 使用非泛型函数
    reg_fn(S(A)); // 具体的类型
    gen_spec_t(SGen(A)); // 隐式地指定类型参数  `A`.
    gen_spec_i32(SGen(6)); // 隐式地指定类型参数`i32`.

    // 显式地指定类型参数 `char`
    generic::<char>(SGen('a'));

    // 隐式地指定类型参数 `char`.
    generic(SGen('c'));
}

// 实现下面的泛型函数 sum

#[test]
fn test_func2() {
    fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
        x + y
    }
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
}

// 实现一个结构体 Point 让代码工作

#[test]
fn test_func3() {
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

// 修改以下结构体让代码工作

#[test]
fn test_func4() {
    struct Point<T, K> {
        x: T,
        y: K,
    }

    // 不要修改这行代码！
    let p = Point {
        x: 5,
        y: "hello".to_string(),
    };
}

#[test]
fn test_func5() {
    // 为 Val 增加泛型参数，不要修改 `main` 中的代码
    struct Val<T> {
        val: T,
    }

    impl<T> Val<T> {
        fn value(&self) -> &T {
            &self.val
        }
    }
    let x = Val { val: 3.0 };
    let y = Val {
        val: "hello".to_string(),
    };
    println!("{}, {}", x.value(), y.value());
}

#[test]
fn test_func6() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        // 实现 mixup，不要修改其它代码！
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point {
        x: "Hello",
        y: '中',
    };

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
}

// 修复错误，让代码工作

#[test]
fn test_func7() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    let p = Point {
        x: 5.0_f32,
        y: 10.0_f32,
    };
    println!("{}", p.distance_from_origin())
}
