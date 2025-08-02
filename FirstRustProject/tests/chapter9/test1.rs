use std::fmt::{Debug, Formatter};
use std::fmt::Display;
#[test]
fn test_func1() {
    assert_eq!(2 + 2, 4);
}

trait TraitA {}
trait TraitB {}

impl<T: TraitB> TraitA for T {}

impl TraitA for u32 {}

// trait TraitA {}
// trait TraitB {}
//
// impl<T: TraitB> TraitA for T {}
//
// impl TraitB for u32 {}
// impl TraitA for u32 {}

// trait TraitA {}
// trait TraitB {}
//
// impl<T: TraitB> TraitA for T {}  // 这里直接对T进行实现TraitA

// struct MyU32(u32);    // 用 MyU32 代替 u32
//
// impl Display for MyU32 {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
//     // 请实现完整
// }
//
// impl MyU32 {
//     fn get(&self) -> u32 {  // 需要定义一个获取真实数据的方法
//         self.0
//     }
// }

// trait TraitA{}
// impl TraitA for u32 {}
//
// impl Display for u32 {}

// struct A;
// impl Display for A {
//
// }

// struct Pair<T>{
//     x:T,
//     y:T
// }
//
// impl <T> Pair<T>{
//     fn new(x:T,y:T)->Self{
//         Self{x,y}
//     }
// }
//
// impl <T:Display + PartialOrd> Pair<T>{
//     // 第二次 impl
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }

// trait TraitA{}
// trait TraitB{}
// trait TraitC{}
//
// struct A;
// struct B;
// struct C;
//
// impl TraitA for A {}
// impl TraitB for A {}
// impl TraitC for A {}  // 对类型A实现了TraitA, TraitB, TraitC
// impl TraitB for B {}
// impl TraitC for B {}  // 对类型B实现了TraitB, TraitC
// impl TraitC for C {}  // 对类型C实现了TraitC
//
// // 7个版本的doit() 函数
// fn doit1<T: TraitA + TraitB + TraitC>(t: T) {}
// fn doit2<T: TraitA + TraitB>(t: T) {}
// fn doit3<T: TraitA + TraitC>(t: T) {}
// fn doit4<T: TraitB + TraitC>(t: T) {}
// fn doit5<T: TraitA>(t: T) {}
// fn doit6<T: TraitB>(t: T) {}
// fn doit7<T: TraitC>(t: T) {}
//
// fn main() {
//     doit1(A);
//     doit2(A);
//     doit3(A);
//     doit4(A);
//     doit5(A);
//     doit6(A);
//     doit7(A);  // A的实例能用在所有7个函数版本中
//
//     doit4(B);
//     doit6(B);
//     doit7(B);  // B的实例只能用在3个函数版本中
//
//     doit7(C);  // C的实例只能用在1个函数版本中
// }

// mod module_a{
//     pub trait Shape{
//         fn play(&self){
//             println!("1");
//         }
//     }
//     pub struct A;
//     impl Shape for A{}
// }
//
// mod module_b{
//     use super::module_a::Shape;  // 引入这个trait
//     use super::module_a::A;
//
//     fn doit(){
//         let a = A;
//         a.play();
//     }
// }

// #[test]
// fn test_func6() {
//     let a = A;
//     a.play();
//     <A as Circle>::play(&a);
//     <A as Shape>::play(&a);
// }
//
// trait Shape{
//     fn play(&self){
//         println!("1")
//     }
// }
//
// trait Circle:Shape{
//     fn play(&self){
//         println!("2")
//     }
// }
//
// struct A;
// impl Shape for A {
//
// }
//
// impl Circle for A{}
//
// impl A{
//     fn play(&self){
//         println!("3")
//     }
// }

// trait TraitA{
//     const LEN:u32 = 10;
// }
//
// struct A;
// impl TraitA for A{
//     const LEN: u32 = 12;
// }
//
// #[test]
// fn test_func5() {
//     println!("{:?}",A::LEN);
//     println!("{:?}",<A as TraitA>::LEN);
// }

// trait TraitA {
//     type Item: Debug;
// }
//
// #[derive(Debug)]
// struct A;
// struct B;
//
// impl TraitA for B {
//     type Item = A;
// }
//
// fn doit<T>()
// where
//     T: TraitA,
//     T::Item: Debug + PartialEq,
// {
// }

// trait TraitA {
//     type Item;
// }
// struct Foo<T: TraitA<Item = String>> {
//     x: T,
// }
// struct A;
// impl TraitA for A {
//     type Item = String;
// }
//
// #[test]
// fn test_func6() {
//     let a = Foo { x: A };
// }

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
