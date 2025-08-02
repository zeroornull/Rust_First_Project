#[test]
fn test_func1() {
    assert_eq!(2 + 2, 4);
}

use std::collections::HashMap;

struct AAA(Vec<u8>);
struct BBB {
    hashmap: HashMap<String, AAA>
}
struct CCC(BBB);
type DDD = Vec<CCC>;
type EEE = HashMap<String, Vec<DDD>>;

// use std::collections::HashMap;
//
// type AAA = HashMap<String, Vec<u8>>;
// type BBB = Vec<AAA>;
// type CCC = HashMap<String, BBB>;
// type DDD = Vec<CCC>;
// type EEE = HashMap<String, DDD>;

struct List<T>(Vec<T>);
#[test]
fn test_func10() {
    assert_eq!(2 + 2, 4);
}

struct Point(u32, u32);

struct Rectangle {
    p1: Point,
    p2: Point,
}

struct Triangle(Point, Point, Point);

struct Circle(Point, u32);

enum Shape {
    Rectangle(Rectangle),
    Triangle(Triangle),
    Circle(Circle),
}

struct Axes;

struct Geometry {
    shape: Shape,
    axes: Axes,
}

struct Algebra;

enum Level {
    // 定义学校的级别
    Elementary, // 小学
    Secondary,  // 初中
    High,       // 高中
}

enum Course {
    Geometry(Geometry),
    Algebra(Algebra),
}

struct MathLesson {
    // 定义数学课程，包括数学的科目和级别
    math: Course,
    level: Level,
}

// struct Point<X1, Y1> {
//     x: X1,
//     y: Y1,
// }
//
// // 这里定义了impl block中可以使用的类型参数X3, Y3，
// impl<X3, Y3> Point<X3, Y3> {
//     // 这里单独为mixup方法定义了两个新的类型参数 X2, Y2
//     // 于是在mixup方法中，可以使用4个类型参数：X3, Y3, X2, Y2
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X3, Y2> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
//
// #[test]
// fn test_func9() {
//     let p1 = Point{x:5,y:10.4};
//     let p2 = Point { x: "Hello", y: 'c' };
//     let p3 = p1.mixup(p2);
//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }
// #[test]
// fn test_func8() {
//     let p = Point { x: 5, y: 10 };
//     println!("p.x = {}", p.x());
// }
//
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {          // 在impl后定义impl block中要用到的类型参数
//     fn x(&self) -> &T {     // 这里，在方法的返回值上使用了这个类型参数
//         &self.x
//     }
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// fn print<T: std::fmt::Display>(p: Point<T>) {
//     println!("Point {}, {}", p.x, p.y);
// }
// #[test]
// fn test_func8() {
//     let p = Point { x: 10, y: 20 };
//     print(p);
//
//     let p = Point { x: 10.2, y: 20.4 };
//     print(p);
// }

// #[test]
// fn test_func7() {
//     let p = PointU32 {x: 10, y: 20};
//     print_u32(p);
//
//     let p = PointF32 {x: 10.2, y: 20.4};
//     print_f32(p);
// }
//
//
// struct PointU32 {
//     x: u32,
//     y: u32,
// }
//
// struct PointF32 {
//     x: f32,
//     y: f32,
// }
//
// fn print_u32(p: PointU32) {
//     println!("Point {}, {}", p.x, p.y);
// }
//
// fn print_f32(p: PointF32) {
//     println!("Point {}, {}", p.x, p.y);
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// enum Aaa<T, U> {
//     V1(Point<T>),
//     V2(Vec<U>),
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn play(n: T) {}
// }
//
// impl Point<u32> {      // 这里，对具化类型 Point<u32> 继续做 impl
//     fn doit() {}
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {        // 注意这一行
//     fn play(n: T) {}      // 注意这一行
// }

// #[allow(dead_code)]
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
//
// #[test]
// fn test_func7() {
//     let _both_integer = Point::<u32, u32> { x: 5, y: 10 };
//     let _both_float = Point::<f32, f32> { x: 1.0, y: 4.0 };
//     let _integer_and_float = Point::<u32, f32> { x: 5, y: 4.0 };
// }

// #[allow(dead_code)]
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// #[test]
// fn test_func6() {
//     let _integer = Point::<u32> { x: 5, y: 10 };
//     let _float = Point::<f32> { x: 1.0, y: 4.0 };
// }
// #[allow(dead_code)]
// struct Point<T,U> {
//     x: T,
//     y: U,
// }
//
// #[test]
// fn test_func5() {
//     let _both_integer = Point { x: 5, y: 10 };
//     let _both_float = Point { x: 1.0, y: 4.0 };
//     let _integer_and_float = Point { x: 5, y: 4.0 };
// }

// #[allow(dead_code)]
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// #[test]
// fn test_func5() {
//     let _integer = Point { x: 5, y: 10 }; // 一个整数point
//
//     let _float = Point { x: 1.0, y: 4.0 }; // 一个浮点数point
// }

#[test]
fn test_func4() {
    let _a = 9 + '1' as u8;
    let _b = 9.to_string() + "1";
}
#[test]
fn test_func3() {
    let a = 1.0f32;
    let b = 10 as f32;

    let _c = a * b;
}
#[test]
fn test_func2() {
    // let a = 1.0f32;
    // let b = 10;

    // let c = a * b;
}
