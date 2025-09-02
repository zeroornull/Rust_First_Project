#[cfg(test)]
mod tests {
    #[test]
    fn sample_test1() {
        #[allow(dead_code)]
        struct Person {
            name: String,
            age: u8,
        }

        impl Person {
            #[allow(dead_code)]
            fn new(name: String, age: u8) -> Self {
                Person { name, age }
            }

            #[allow(dead_code)]
            #[allow(unused_variables)]
            fn display(self: &mut Person, age: u8) {
                #[allow(unused_variables)]
                let Person { name, age } = &self;
            }
        }
    }

    #[test]
    fn sample_test2() {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn sample_test3() {
        fn main() {
            let x = Box::new(1);
            #[allow(unused_variables)]
            let sum = *x + 1;
        }

        main();
    }

    #[test]
    fn sample_test4() {
        #[allow(dead_code)]
        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            #[allow(dead_code)]
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        use std::ops::Deref;

        impl<T> Deref for MyBox<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        fn main() {
            let y = MyBox::new(5);

            assert_eq!(5, *y);

            let s = MyBox::new(String::from("hello world"));
            display(&s);

            let m = MyBox::new(String::from("Rust"));
            display(&(*m)[..]);
        }

        fn display(s: &str) {
            println!("{}", s);
        }

        main();
    }

    #[test]
    fn sample_test5() {
        fn main() {
            let s = String::from("hello world");
            display(&s);
        }

        fn display(s: &str) {
            println!("{}", s);
        }

        main();
    }

    #[test]
    fn sample_test6() {
        #[allow(unused_variables)]
        fn foo(s:&str){
            
        }
        
        let owned = "Hello".to_string();
        
        foo(&owned)
    }

    #[test]
    fn sample_test7() {
        use std::rc::Rc;
        
        #[allow(unused_variables)]
        fn foo(s:&str){
            
        }
        
        let owned = "Hello".to_string();
        
        let counted = Rc::new(owned);
        
        foo(&counted);
    }

    #[test]
    fn sample_test8() {
        struct Foo;

        impl Foo {
            fn foo(&self) { println!("Foo"); }
        }

        let f = &&Foo;

        f.foo();
        (&f).foo();
        (&&f).foo();
        (&&&&&&&&f).foo();
    }

    #[test]
    fn sample_test9() {
        struct MyBox<T>{
            v:T,
        }
        
        impl <T> MyBox<T>{
            fn new(x:T)->MyBox<T>{
                MyBox{v:x}
            }
        }
        
        use std::ops::Deref;
        
        impl <T> Deref for MyBox<T>{
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.v
            }
        }
        
        use std::ops::DerefMut;
        
        impl<T> DerefMut for MyBox<T>{
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.v
            }
        }
        
        fn main(){
            let mut s = MyBox::new(String::from("hello, "));
            display(&mut s)
        }
        
        fn display(s:&mut String){
            s.push_str("world");
            println!("{}",s);
        }
        
        main();
    }
    
}
