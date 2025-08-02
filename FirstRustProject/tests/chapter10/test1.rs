use std::fmt::Debug;
use std::sync::TryLockError::Poisoned;
#[test]
fn test_func1() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_func7() {
    struct Atype;
    struct Btype;
    struct Ctype;

    trait TraitA {}

    impl TraitA for Atype {}
    impl TraitA for Btype {}
    impl TraitA for Ctype {}

    fn doit(x:&dyn TraitA) {}

    let a = Atype;
    doit(&a);
    let b = Btype;
    doit(&b);
    let c = Ctype;
    doit(&c);
}

#[test]
fn test_func6() {
    struct Atype;
    struct Btype;
    struct Ctype;

    trait TraitA {}

    impl TraitA for Atype {}
    impl TraitA for Btype {}

    impl TraitA for Ctype {}

    fn doit(x: impl TraitA) {
        // 等价于
        // fn doit<T: TraitA>(x:T) {}
    }
    let a = Atype;
    doit(a);
    let b = Btype;
    doit(b);
    let c = Ctype;
    doit(c);
}

/*struct Atype;
struct Btype;
struct Ctype;

trait TraitA {}

impl TraitA for Atype {}
impl TraitA for Btype {}
impl TraitA for Ctype {}

fn doit(i: u32) -> Box<dyn TraitA> {
    if i == 0 {
        let a = Atype;
        Box::new(a)
    } else if i == 1 {
        let b = Btype;
        Box::new(b)
    } else {
        let c = Ctype;
        Box::new(c)
    }
}*/

// struct Atype;
// struct Btype;
// struct Ctype;
//
// trait TraitA {}
//
// impl TraitA for Atype {}
// impl TraitA for Btype {}
// impl TraitA for Ctype {}
//
// fn doit() -> impl TraitA {  // 注意这一行的函数返回类型
//     let a = Atype;
//     a
//     // 或
//     // let b = Btype;
//     // b
//     // 或
//     // let c = Ctype;
//     // c
// }

// #[test]
// fn test_func5() {
//     let a: Atype = doit::<Atype>();
//     let b: Btype = doit::<Btype>();
//     let c: Ctype = doit::<Ctype>();
// }
//
//
// struct Atype;
// struct Btype;
// struct Ctype;
//
// trait TraitA {
//     fn new() -> Self;    // TraitA中定义了new()函数
// }
//
// impl TraitA for Atype {
//     fn new() -> Atype {
//         Atype
//     }
// }
//
// impl TraitA for Btype {
//     fn new() -> Btype {
//         Btype
//     }
// }
//
// impl TraitA for Ctype {
//     fn new() -> Ctype {
//         Ctype
//     }
// }
//
// fn doit<T: TraitA>() -> T {
//     T::new()
// }

// impl Atype {
//     fn new() -> Atype {
//         Atype
//     }
// }
//
// impl Btype {
//     fn new() -> Btype {
//         Btype
//     }
// }
//
// impl Ctype {
//     fn new() -> Ctype {
//         Ctype
//     }
// }
//
// fn doit<T>() -> T {
//     T::new()
// }

// struct Atype;
// struct Btype;
// struct Ctype;
//
// enum TotalType {
//     A(Atype),
//     B(Btype),
//     C(Ctype),
// }
//
// fn doit<T>()->T{
//     let a = Atype;
//     a
// }

// fn doit(i: u32) -> TotalType {
//     if i == 0 {
//         let a = Atype;
//         TotalType::A(a)
//     } else if i == 1 {
//         let b = Btype;
//         TotalType::B(b)
//     } else {
//         let c = Ctype;
//         TotalType::C(c)
//     }
// }

// #[test]
// fn test_func4() {
//     let p1 = Point { x: 1, y: 1 };
//     let p2 = Point { x: 2, y: 2 };
//     let p3 = p1.add(p2);
//     assert_eq!(p3.x, 3);
//     assert_eq!(p3.y, 3);
//
//     let p1 = Point { x: 1, y: 1 };
//     let delta = 2;
//     let p3 = p1.add(delta); // 这句是错的
//     assert_eq!(p3.x, 3);
//     assert_eq!(p3.y, 3);
// }
//
// trait Add{
//     type ToAdd;
//     type Output;
//     fn add(self,rhs:Self::ToAdd) -> Self::Output;
// }
//
// struct Point{
//     x:i32,
//     y:i32
// }
//
// impl Add for Point{
//     type ToAdd = Point;
//     type Output = Point;
//
//     fn add(self, rhs: Point) -> Point {
//         Point {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }
//
// impl Add for Point{
//     type ToAdd = i32;
//     type Output = Point;
//     fn add(self, rhs: i32) -> Point {
//         Point {
//             x: self.x + rhs,
//             y: self.y + rhs,
//         }
//     }
// }

// #[test]
// fn test_func3() {
//     let a = AType;
//     a.play(10u32)
// }
//
// trait TraitA<T>
// where
//     T: Debug,
// {
//     fn play(&self, _t: T) {}
// }
// struct AType;
//
// impl<T> TraitA<T> for AType where T: Debug + PartialEq {}

// // Self可以用在默认类型位置上
// trait TraitA<T = Self>{
//     fn func(t:T){}
// }
//
// // 这个默认类型为i32
// trait TraitB<T = i32>{
//     fn func2(t:T){}
// }

// struct SomeType;
//
// // 这里省略了类型参数，所以这里的T为Self
// // 进而T就是SomeType本身
// impl TraitA for SomeType{
//     fn func(t:SomeType){}
// }
//
// // 这里省略了类型参数，使用默认类型i32
// impl TraitB for SomeType {
//     fn func2(t: i32) {}
// }
//
// // 这里不省略类型参数，明确指定类型参数为String
// impl TraitA<String> for SomeType {
//     fn func(t: String) {}
// }
//
// // 这里不省略类型参数，明确指定类型参数为String
// impl TraitB<String> for SomeType {
//     fn func2(t: String) {}
// }

// #[test]
// fn test_func2() {
//     let p1 = Point { x: 1, y: 1 };
//     let p2 = Point { x: 2, y: 2 };
//     let p3 = p1.add(p2);
//     assert_eq!(p3.x, 3);
//     assert_eq!(p3.y, 3);
//
//     let p1 = Point { x: 1, y: 1 };
//     let delta = 2;
//     let p3 = p1.add(delta);   // 一个Point实例加一个i32
//     assert_eq!(p3.x, 3);
//     assert_eq!(p3.y, 3);
// }
//
// trait Add<T> {
//     type Output;
//     fn add(self, rhs: T) -> Self::Output;
// }
//
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// // 为 Point 实现 Add<Point> 这个 trait
// impl Add<Point> for Point {
//     type Output = Self;
//
//     fn add(self, rhs: Point) -> Self::Output {
//         Point {
//             x: self.x + rhs.y,
//             y: self.y + rhs.y,
//         }
//     }
// }
//
// impl Add<i32> for Point {
//     type Output = Self;
//     fn add(self, rhs: i32) -> Self::Output {
//         Point {
//             x: self.x + rhs,
//             y: self.y + rhs,
//         }
//     }
// }
