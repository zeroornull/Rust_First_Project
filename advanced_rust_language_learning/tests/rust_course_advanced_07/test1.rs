#[cfg(test)]
mod tests {
    #[test]
    fn sample_test1() {
        const MAX_ID: usize = usize::MAX / 2;
        fn main() {
            println!("用户ID允许的最大值是{}", MAX_ID);
        }

        main();
    }

    #[test]
    fn sample_test2() {
        // static mut REQUEST_RECV: usize = 0;
        // fn main() {
        //     unsafe {
        //         REQUEST_RECV += 1;
        //         assert_eq!(REQUEST_RECV, 1);
        //     }
        // }
        //
        // main();
    }

    #[test]
    fn sample_test3() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);
        fn main() {
            for _ in 0..100 {
                REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
            }
            println!("当前用户请求数{:?}", REQUEST_RECV);
        }

        main();
    }

    #[test]
    fn sample_test4() {
        // use std::sync::atomic::{AtomicUsize, Ordering};

        // struct Factory {
        //     factory_id: usize,
        // }
        //
        // static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
        // const MAX_ID: usize = usize::MAX / 2;
        //
        // fn generate_id() -> usize {
        //     // 检查两次溢出，否则直加一可能导致溢出
        //     let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
        //     if current_val > MAX_ID {
        //         panic!("Factory ids overflowed");
        //     }
        //     GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        //     let next_id = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
        //     if next_id > MAX_ID {
        //         panic!("Factory ids overflowed");
        //     }
        //     next_id
        // }
        //
        // impl Factory {
        //     fn new() -> Self {
        //         Self {
        //             factory_id: generate_id(),
        //         }
        //     }
        // }
    }

    #[test]
    fn sample_test5() {
        use lazy_static::lazy_static;
        use std::sync::Mutex;

        lazy_static! {
            static ref NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
        }

        fn main() {
            let mut v = NAMES.lock().unwrap();
            v.push_str(", Myth");
            println!("{}", v);
        }

        main();
    }

    #[test]
    fn sample_test6() {
        use lazy_static::lazy_static;
        use std::collections::HashMap;

        lazy_static! {
            static ref HASHMAP: HashMap<u32, &'static str> = {
                let mut m = HashMap::new();
                m.insert(0, "foo");
                m.insert(1, "bar");
                m.insert(2, "baz");
                m
            };
        }

        fn main() {
            // 首次访问`HASHMAP`的同时对其进行初始化
            println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

            // 后续的访问仅仅获取值，再不会进行任何初始化操作
            println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());
        }
        main();
    }

    #[test]
    fn sample_test7() {
        // #[derive(Debug)]
        // struct Config {
        //     a: String,
        //     b: String
        // }
        // static mut CONFIG: Option<&mut Config> = None;
        //
        // fn main(){
        //     let c = Box::new(Config {
        //         a: "A".to_string(),
        //         b: "B".to_string(),
        //     });
        //
        //     unsafe {
        //         // 将`c`从内存中泄漏，变成`'static`生命周期
        //         CONFIG = Some(Box::leak(c));
        //         println!("{:?}", CONFIG);
        //     }
        // }
        // main();
    }

    #[test]
    fn sample_test8() {
        // #[derive(Debug)]
        // struct Config {
        //     a: String,
        //     b: String,
        // }
        // static mut CONFIG: Option<&mut Config> = None;
        //
        // fn init()->Option<&'static mut Config>{
        //     let c = Box::new(Config {
        //         a: "A".to_string(),
        //         b: "B".to_string(),
        //     });
        //     Some(Box::leak(c))
        // }
        //
        // fn main() {
        //     unsafe {
        //         CONFIG = init();
        //
        //         println!("{:?}", CONFIG)
        //     }
        // }
        // main();
    }

    #[test]
    fn sample_test9() {
        use std::{sync::OnceLock, thread};

        fn main() {
            // 子线程调用
            let handle = thread::spawn(|| {
                let logger = Logger::global();
                logger.log("thread message".to_string());
            });

            // 主线程调用
            let logger = Logger::global();
            logger.log("some message".to_string());

            let logger2 = Logger::global();
            logger2.log("other message".to_string());

            handle.join().unwrap();
        }

        #[derive(Debug)]
        struct Logger;

        static LOGGER: OnceLock<Logger> = OnceLock::new();

        impl Logger {
            fn global() -> &'static Logger {
                // 获取或初始化 Logger
                LOGGER.get_or_init(|| {
                    println!("Logger is being created..."); // 初始化打印
                    Logger
                })
            }

            fn log(&self, message: String) {
                println!("{}", message)
            }
        }

        main();
    }

    #[test]
    fn sample_test10() {
        use std::{sync::LazyLock, thread};

        fn main() {
            // 子线程中调用
            let handle = thread::spawn(|| {
                let logger = &LOGGER;
                logger.log("thread message".to_string());
            });

            // 主线程调用
            let logger = &LOGGER;
            logger.log("some message".to_string());

            let logger2 = &LOGGER;
            logger2.log("other message".to_string());

            handle.join().unwrap();
        }

        #[derive(Debug)]
        struct Logger;

        static LOGGER: LazyLock<Logger> = LazyLock::new(Logger::new);

        impl Logger {
            fn new() -> Logger {
                println!("Logger is being created...");
                Logger
            }


            fn log(&self, message: String) {
                println!("{}", message)
            }
        }

        main();
    }
}
