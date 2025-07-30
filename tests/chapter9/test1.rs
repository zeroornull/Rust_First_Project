#[test]
fn test_func1() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_func6() {
    assert_eq!(2 + 2, 4);
}

trait TraitA {
    type Item;
}
struct Foo<T: TraitA<Item = String>> {
    x: T,
}
struct A;
impl TraitA for A {
    type Item = String;
}

#[test]
fn test_func5() {
    let a = Foo { x: A };
}

// #[test]
// fn test_func5() {
//     doit::<TypeA>("abc".to_string())
// }
//
// trait TraitA {
//     type MyType;
// }
//
// fn doit<T: TraitA>(a: T::MyType) {}
//
// struct TypeA;
//
// impl TraitA for TypeA {
//     type MyType = String;
// }
//
// pub trait Iterator {
//     type Item;
//
//     fn next(&mut self) -> Option<Self::Item>;
// }
//
// #[test]
// fn test_func4() {
//     let f = Football;
//     f.play(SportType::Land);
// }
//
// pub trait Sport {
//     type SportType;
//
//     fn play(&self, st: Self::SportType);
// }
//
// struct Football;
// pub enum SportType {
//     Land,
//     Water,
// }
//
// impl Sport for Football {
//     type SportType = SportType;
//     fn play(&self, st: Self::SportType) {}
// }

// #[test]
// fn test_func3() {
//     let mut f = Football;
//     f.play();      // 方法在实例上调用
//     f.play_mut();
//     f.play_own();
//     let _g = Football::play_some();    // 关联函数要在类型上调用
//     let _g = <Football as Sport>::play_some();  // 注意这样也是可以的
// }
//
// struct Football;
//
// trait Sport {
//     fn play(&self) {}    // 注意这里一对花括号，就是trait的关联函数的默认实现
//     fn play_mut(&mut self) {}
//     fn play_own(self);   // 注意这里是以分号结尾，就表示没有默认实现
//     fn play_some() -> Self;
// }
//
// impl Sport for Football{
//     fn play(&self) {}
//     fn play_mut(&mut self) {}
//     fn play_own(self) {}
//     fn play_some() -> Self { Self }
// }
// #[test]
// fn test_func2() {
//     let p = Point {x: 10, y: 20};
//     print(p);
//
//     let p = Point {x: 10.2, y: 20.4};
//     print(p);
//
//     // let p = Point {x: Foo, y: Foo};  // 初始化一个Point<T> 实例
//     // print(p);
// }
//
// struct Foo;  // 新定义了一种类型
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// fn print<T: std::fmt::Display>(p: Point<T>) {
//     println!("Point {}, {}", p.x, p.y);
// }
