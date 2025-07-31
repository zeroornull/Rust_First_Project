use std::fmt::{format, write};

#[test]
fn test_func1() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_func15() {
    struct Person {
        name: String,
    }
    impl Person {
        //
        fn new1(name: String) -> Person {
            Person { name }
        }
        // 这个方法可接收
        // - String
        // - &String
        // - &str
        // - Box<str>
        // - char
        // 这几种参数，因为它们都实现了Into<String>
        fn new2<N: Into<String>>(name: N) -> Person {
            Person { name: name.into() } // 调用into()，写起来很简洁
        }
    }
}
#[test]
fn test_func14() {
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    //
    // impl From<(i32, i32)> for Point {
    //     fn from(value: (i32, i32)) -> Self {
    //         Point { x, y }
    //     }
    // }
    //
    // impl From<[i32; 2]> for Point {
    //     // 实现从[i32; 2]到Point的转换
    //     fn from([x, y]: [i32; 2]) -> Self {
    //         Point { x, y }
    //     }
    // }
    //
    // fn example() {
    //     // 使用from()转换不同类型
    //     let origin = Point::from((0, 0));
    //     let origin = Point::from([0, 0]);
    //     // 使用into()转换不同类型
    //     let origin: Point = (0, 0).into();
    //     let origin: Point = [0, 0].into();
    // }
}

#[test]
fn test_func13() {
    trait FnOnce<Args> {
        type Output;
        fn call_once(self, args: Args) -> Self::Output;
    }
    trait FnMut<Args>: FnOnce<Args> {
        fn call_mut(&mut self, args: Args) -> Self::Output;
    }
    trait Fn<Args>: FnMut<Args> {
        fn call(&self, args: Args) -> Self::Output;
    }

    let range = 0..10;
    let get_range_count = || range.count();
    assert_eq!(get_range_count(), 10);
    // get_range_count();

    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    let mut min = i32::MIN;
    let ascending = nums
        .into_iter()
        .filter(|&n| {
            if n <= min {
                false
            } else {
                min = n;
                true
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(vec![0, 4, 8, 10, 15, 18], ascending); // ✅

    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    let min = 9;
    let greater_than_9 = nums.into_iter().filter(|&n| n > min).collect::<Vec<_>>();
    assert_eq!(vec![10, 15, 18, 13], greater_than_9); // ✅

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    let mut fn_ptr: fn(i32) -> i32 = add_one; // 注意这里的类型定义
    assert_eq!(fn_ptr(1), 2); // ✅

    // 如果一个闭包没有捕捉环境变量，它可以通过类型转换转成 fn 类型
    fn_ptr = |x| x + 1; // same as add_one
    assert_eq!(fn_ptr(1), 2); // ✅
}

#[test]
fn test_func12() {
    #[derive(Copy, Clone)]
    struct Point {
        x: u32,
        y: u32,
    }

    let a = Point { x: 10, y: 10 };
    let b = a; // 这里发生了复制，a在后续可以继续使用
    let c = a; // 这里又复制了一份，这下有3份了
    // println!("{}",b)
}

#[test]
fn test_func11() {
    #[derive(Clone)]
    struct Point {
        x: u32,
        y: u32,
    }
    let a = Point { x: 10, y: 10 };
    let b = a; // 这里发生了所有权move，a在后续不能使用了
}

#[test]
fn test_func10() {
    #[derive(Clone, Debug)]
    struct Atype {
        num: u32,
        a_vec: Vec<u32>,
    }

    let a = Atype {
        num: 100,
        a_vec: vec![10, 20, 30],
    };

    let mut b = a.clone();
    b.num = 200;
    b.a_vec[0] = 11;
    b.a_vec[1] = 21;
    b.a_vec[2] = 31;

    println!("{a:?}");
    println!("{b:?}");
}

#[test]
fn test_func9() {
    struct Atype {
        num: u32,
        a_vec: Vec<u32>,
    }
    let a = Atype {
        num: 100,
        a_vec: vec![10, 20, 30],
    };
    let b = a; // 这里发生了移动
}

#[test]
fn test_func8() {
    // trait Add<Rhs = Self> {
    //     type Output;
    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    //
    // impl Add for Point {
    //     type Output = Point;
    //     fn add(self, rhs: Point) -> Point {
    //         Point {
    //             x: self.x + rhs.x,
    //             y: self.y + rhs.y,
    //         }
    //     }
    // }
    //
    // let p1 = Point{x:1,y:2};
    // let p2 = Point { x: 3, y: 4 };
    // let p3 = p1 + p2; // 这里直接用+号作用在两个Point实例上
    // assert_eq!(p3.x, p1.x + p2.x); // ✅
    // assert_eq!(p3.y, p1.y + p2.y); // ✅
}

#[test]
fn test_func7() {
    use std::collections::BTreeSet;

    #[derive(Ord, PartialOrd, PartialEq, Eq)]
    struct Point {
        x: i32,
        y: i32,
    }

    fn example_btreeset() {
        let mut points = BTreeSet::new();
        points.insert(Point { x: 0, y: 0 });
    }

    fn example_sort<T: Ord>(mut sortable: Vec<T>) -> Vec<T> {
        sortable.sort();
        sortable
    }
}
#[test]
fn test_func6() {
    #[derive(PartialEq, PartialOrd)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(PartialEq, PartialOrd)]
    enum StopLight {
        Red,
        Yellow,
        Green,
    }
}

#[test]
fn test_func5() {
    #[derive(PartialEq, Debug)] // 注意这一句
    struct Point {
        x: i32,
        y: i32,
    }

    fn example_assert(p1: Point, p2: Point) {
        assert_eq!(p1, p2)
    }

    fn example_compare_collections<T: PartialEq>(vec1: Vec<T>, vec2: Vec<T>) {
        if vec1 == vec2 {
            //
        } else {
        }
    }
}

#[test]
fn test_func4() {
    use std::fmt;
    #[derive(Default)]
    struct Point {
        x: i32,
        y: i32,
    }
    //
    impl fmt::Display for Point {
        // 实现唯一的fmt方法，这里定义用户自定义的格式
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y) // write!宏向stdout写入
        }
    }
    println!("origin: {}", Point::default());
    // 打印出 "origin: (0, 0)"
    // 在 format! 中用 "{}" 将类型表示/转换为 String
    let stringified = format!("{}", Point::default());
    assert_eq!("(0, 0)", stringified); // ✅
}

#[test]
fn test_func3() {
    #[derive(Default)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }
    impl Color {
        fn new(r: u8, g: u8, b: u8) -> Self {
            Color { r, g, b }
        }
    }

    impl Color {
        fn red(r: u8) -> Self {
            Color {
                r,
                ..Color::default()
            }
        }
        fn green(r: u8) -> Self {
            Color {
                r,
                ..Color::default()
            }
        }
        fn blue(r: u8) -> Self {
            Color {
                r,
                ..Color::default()
            }
        }
    }
}

#[test]
fn test_func2() {
    trait Default {
        fn default() -> Self;
    }

    struct Color(u8, u8, u8);

    impl std::default::Default for Color {
        //
        fn default() -> Self {
            Color(0, 0, 0)
        }
    }

    let color = Color::default();
    // let color: Color = Default::default();
}
