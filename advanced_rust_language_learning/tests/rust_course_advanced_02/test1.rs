#[cfg(test)]
mod tests {
    #[test]
    fn sample_test1() {
        fn main() {
            let x = 1;
            let sum = |y| x + y;
            assert_eq!(3, sum(2))
        }
        main();
    }

    #[test]
    fn sample_test2_1() {
        use std::thread;
        use std::time::Duration;

        fn muuuuu(intensity: u32) -> u32 {
            println!("muuuuu...");
            thread::sleep(Duration::from_secs(2));
            intensity
        }

        fn workout(intensity: u32, random_number: u32) {
            if intensity < 25 {
                println!("今天活力满满，先做 {} 个俯卧撑!", muuuuu(intensity));
                println!(
                    "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
                    muuuuu(intensity)
                );
            } else if random_number == 3 {
                println!("昨天练过度了，今天还是休息下吧！");
            } else {
                println!(
                    "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
                    muuuuu(intensity)
                );
            }
        }

        fn main() {
            // 强度
            let intensity = 10;
            // 随机值用来决定某个选择
            let random_number = 7;

            // 开始健身
            workout(intensity, random_number);
        }

        main();
    }

    #[test]
    fn sample_test2_2() {
        use std::thread;
        use std::time::Duration;

        // 开始健身，好累，我得发出声音：muuuu...
        fn muuuuu(intensity: u32) -> u32 {
            println!("muuuu.....");
            thread::sleep(Duration::from_secs(2));
            intensity
        }

        fn workout(intensity: u32, random_number: u32) {
            let action = muuuuu;
            if intensity < 25 {
                println!("今天活力满满, 先做 {} 个俯卧撑!", action(intensity));
                println!(
                    "旁边有妹子在看，俯卧撑太low, 再来 {} 组卧推!",
                    action(intensity)
                );
            } else if random_number == 3 {
                println!("昨天练过度了，今天还是休息下吧！");
            } else {
                println!(
                    "昨天练过度了，今天干干有氧, 跑步 {} 分钟!",
                    action(intensity)
                );
            }
        }

        fn main() {
            // 强度
            let intensity = 10;
            // 随机值用来决定某个选择
            let random_number = 7;

            // 开始健身
            workout(intensity, random_number);
        }

        main();
    }

    #[test]
    fn sample_test2_3() {
        use std::thread;
        use std::time::Duration;

        fn workout(intensity: u32, random_number: u32) {
            let action = || {
                println!("muuuu.....");
                thread::sleep(Duration::from_secs(2));
                intensity
            };

            if intensity < 25 {
                println!("今天活力满满，先做 {} 个俯卧撑!", action());
                println!("旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!", action());
            } else if random_number == 3 {
                println!("昨天练过度了，今天还是休息下吧！");
            } else {
                println!("昨天练过度了，今天干干有氧，跑步 {} 分钟!", action());
            }
        }

        fn main() {
            // 动作次数
            let intensity = 10;
            // 随机值用来决定某个选择
            let random_number = 7;

            // 开始健身
            workout(intensity, random_number);
        }

        main();
    }

    #[test]
    fn sample_test3() {
        struct Cacher<T, E>
        where
            T: Fn(E) -> E,
            E: Copy,
        {
            query: T,
            value: Option<E>,
        }

        impl<T, E> Cacher<T, E>
        where
            T: Fn(E) -> E,
            E: Copy,
        {
            fn new(query: T) -> Cacher<T, E> {
                Cacher { query, value: None }
            }
            //
            fn value(&mut self, args: E) -> E {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.query)(args);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }

        let mut c = Cacher::new(|a| a);

        #[allow(unused_variables)]
        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 1);
    }

    #[test]
    fn sample_test4() {
        fn fn_once<F>(func: F)
        where
            F: FnOnce(usize) -> bool + Copy,
        {
            println!("{}", func(3));
            println!("{}", func(4));
        }
        fn main() {
            let x = vec![1, 2, 3];
            fn_once(|z| z == x.len())
        }
        main();
    }

    #[test]
    fn sample_test5() {
        use std::thread;
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });
        handle.join().unwrap();
    }

    #[test]
    fn sample_test6() {
        fn main() {
            let mut s = String::new();
            let mut update_string = |str| s.push_str(str);
            update_string("hello");
            println!("{:?}", s);
        }

        main();
    }

    #[test]
    fn sample_test7() {
        let mut s = String::new();
        let update_string = |str| s.push_str(str);
        exec(update_string);
        println!("{:?}", s);

        fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
            f("hello!")
        }
    }

    #[test]
    fn sample_test8() {
        // let s = String::new();
        // let update_string = move || println!("{}",s);
        //
        // exec(update_string);
        //
        // let mut s = String::new();
        //
        // let mut update_string = || s.push_str("hello!");
        // exec(update_string);
        //
        // fn exec<'a,F:FnMut(&'a str)>(mut f:F){
        //     f("hello!")
        // }
    }

    #[test]
    fn sample_test9() {
        // let mut s = String::new();
        //
        // let update_string = |str| s.push_str(str);
        //
        // exec(update_string);
        //
        // println!("{:?}", s);
        //
        // fn exec<'a, F: Fn(&'a str)>(mut f: F) {
        //     f("hello")
        // }
    }

    #[test]
    fn sample_test10() {
        let s = "hello, ".to_string();

        let update_string = |str| println!("{},{}", s, str);

        exec(update_string);

        println!("{:?}", s);

        fn exec<'a, F: Fn(String) -> ()>(f: F) {
            f("world".to_string())
        }
    }

    #[test]
    fn sample_test11() {
        // fn main(){
        //     let s = String::new();
        //     let update_string = move||println!("{}",s);
        //     exec(update_string);
        // }
        //
        // fn exec(F:Fn())>(f:F){
        //     f()
        // }
    }

    #[test]
    fn sample_test12() {
        let s = String::new();

        let update_string = || println!("{}", s);
        exec(update_string);
        exec1(update_string);
        exec2(update_string);

        fn exec<F: FnOnce()>(f: F) {
            f()
        }

        fn exec1<F: FnMut()>(mut f: F) {
            f()
        }

        fn exec2<F: Fn()>(f: F) {
            f()
        }
    }

    #[test]
    fn sample_test13() {
        // let mut s = String::new();
        // let update_string = |str| -> String{s.push_str(str);s};
        //
        // exec(update_string);
        //
        // fn exec<'a,F:FnMut(&'a str) -> String>(mut f:F){
        //     f("hello")
        // }
    }

    #[test]
    fn sample_test14() {
        // fn factory() -> Fn(i32) -> i32 {
        //     let num = 5;
        //     |x| x + num
        // }
        // let f = factory();
        // let answer = f(1);
        // assert_eq!(6, answer);
    }

    #[test]
    fn sample_test15() {
        #[allow(dead_code)]
        fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
            let num = 5;
            if x > 1 {
                Box::new(move |x| x + num)
            } else {
                Box::new(move |x| x - num)
            }
        }
    }
}
