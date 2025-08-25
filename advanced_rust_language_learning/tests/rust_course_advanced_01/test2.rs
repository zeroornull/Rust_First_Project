#[cfg(test)]
mod tests {
    use std::fmt::Display;

    #[test]
    fn sample_func1() {
        fn print_author(author: &'static str) {
            println!("{}", author);
        }
        fn main() {
            let mark_twain: &str = "Samuel Clemens";
            print_author(mark_twain);
        }

        main();
    }

    #[test]
    fn sample_func2() {
        use std::fmt::Display;
        fn main() {
            let mark_twain = "Samuel Clemens";
            print(&mark_twain);
        }

        fn print<T: Display + 'static>(message: &T) {
            println!("{}", message);
        }

        main();
    }

    #[test]
    fn sample_func3() {
        use std::{slice::from_raw_parts, str::from_utf8_unchecked};

        fn get_memory_location() -> (usize, usize) {
            let string = "Hello World!";
            let pointer = string.as_ptr() as usize;
            let length = string.len();
            (pointer, length)
        }

        fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
            unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
        }

        fn main() {
            let (pointer, length) = get_memory_location();
            let message = get_str_at_location(pointer, length);
            // let message = get_str_at_location(1000, 10);
            println!(
                "The {} bytes at 0x{:X} stored: {}",
                length, pointer, message
            );
        }

        main();
    }

    #[test]
    fn sample_func4() {
        use std::fmt::Debug;
        fn print_it<T: Debug + 'static>(input: &T) {
            println!("'static value passed in is: {:?}", input);
        }

        #[allow(dead_code)]
        fn print_it1(input: impl Debug + 'static) {
            println!("'static value passed in is: {:?}", input);
        }

        fn main() {
            let i = 5;

            print_it(&i);
            // print_it1(&i);
        }

        main();
    }

    #[test]
    fn sample_func5() {
        fn main() {
            let r1;
            let r2;
            {
                static STATIC_EXAMPLE: i32 = 42;
                r1 = &STATIC_EXAMPLE;
                let x = "&'static str";
                r2 = x;
            }

            println!("Static example: {}", r1);
            println!("Static example: {}", r2);

            #[allow(unused_variables)]
            let r3: &str;
            #[allow(unused_assignments)]
            {
                let s1 = "String".to_string();
                //
                //
                static_bound(&s1);

                r3 = &s1;
            }

            fn static_bound<T: Display + 'static>(t: &T) {
                println!("{}", t);
            }
        }
        main();
    }

    #[test]
    fn sample_func6() {
        // fn main(){
        //     let static_string = "I'm in read-only memory";
        //     println!("static_string: {}", static_string);
        // }
        // println!("static_string reference remains alive: {}", static_string);
    }

    #[test]
    fn test_func1_1() {
        /* 使用两种方法填空 */
        fn main() {
            let v: &str = "hello";
            need_static(v);

            println!("Success!")
        }

        fn need_static(r: &'static str) {
            assert_eq!(r, "hello");
        }

        main();
    }

    #[test]
    fn test_func1_2() {
        /* 使用两种方法填空 */
        fn main() {
            const V: &str = "hello";
            need_static(V);

            println!("Success!")
        }

        fn need_static(r: &'static str) {
            assert_eq!(r, "hello");
        }

        main();
    }

    #[test]
    fn test_func2() {
        // #[derive(Debug)]
        // struct Config {
        //     a: String,
        //     b: String,
        // }
        // static mut config: Option<&mut Config> = None;
        //
        // /* 让代码工作，但不要修改函数的签名 */
        // fn init() -> Option<&'static mut Config> {
        //     let c = Box::new(Config {
        //         a: "A".to_string(),
        //         b: "B".to_string(),
        //     });
        //
        //     Some(Box::leak(c))
        // }
        //
        //
        // fn main() {
        //     unsafe {
        //         config = init();
        //
        //         println!("{:?}", config)
        //     }
        // }
        //
        // main();
    }

    #[test]
    fn test_func3() {
        // fn main(){
        //     //
        //     let static_string = "I'm in read-only memory";
        //     println!("static_string: {}", static_string);
        // }
        //
        // println!("static_string reference remains alive: {}", static_string);
    }

    #[test]
    fn test_func4() {
        static NUM: i32 = 18;
        fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
            &NUM
        }

        fn main() {
            {
                let lifetime_num = 9;
                let coerce_static = coerce_static(&lifetime_num);

                println!("coerced_static: {}", coerce_static);
            }

            println!("NUM: {} stays accessible!", NUM);
        }

        main();
    }

    #[test]
    fn test_func5() {
        use std::fmt::Debug;

        fn print_it<T: Debug + 'static>(input: T) {
            println!("'static value passed in is: {:?}", input);
        }

        fn print_it1(input: impl Debug + 'static) {
            println!("'static value passed in is: {:?}", input);
        }

        fn print_it2<T: Debug + 'static>(input: &T) {
            println!("'static value passed in is: {:?}", input);
        }

        fn main() {
            //
            const I: i32 = 5;
            print_it(I);

            //
            print_it(&I);

            print_it1(&I);

            //
            print_it2(&I);
        }

        main();
    }
}
