#[cfg(test)]
mod tests {

    #[test]
    fn sample_test1() {
        // use std::thread;
        // use std::rc::Rc;
        // fn main(){
        //     let v = Rc::new(5);
        //     let t = thread::spawn(move || {
        //         println!("{}",v);
        //     });
        //     t.join().unwrap();
        //
        // }
    }

    #[test]
    fn sample_test2() {
        // use std::thread;
        // fn main(){
        //     let p = 5 as *mut u8;
        //     let t = thread::spawn(move || {
        //        println!("{:?}",p);
        //     });
        //
        //     t.join().unwrap();
        // }
        //
        // main();
    }

    #[test]
    fn sample_test3() {
        use std::thread;

        #[derive(Debug)]
        #[allow(dead_code)]
        struct MyBox(*mut u8);
        unsafe impl Send for MyBox {}
        fn main() {
            let p = MyBox(5 as *mut u8);
            let t = thread::spawn(move || {
                println!("{:?}", p);
            });
            t.join().unwrap();
        }

        main();
    }

    #[test]
    fn sample_test4() {
        // use std::thread;
        // fn main() {
        //     let v = 5;
        //     let t = thread::spawn(|| {
        //         println!("{:?}",&v);
        //     });
        // 
        //     t.join().unwrap();
        // }
    }

    #[test]
    fn sample_test5() {
        // use std::thread;
        // use std::sync::Arc;
        // use std::sync::Mutex;
        // 
        // #[derive(Debug)]
        // struct MyBox(*const u8);
        // unsafe impl Send for MyBox{}
        // 
        // fn main(){
        //     let b  = &MyBox(5 as *const u8);
        //     let v = Arc::new(Mutex::new(b));
        //     let t = thread::spawn(move || {
        //         let _v1 = v.lock().unwrap();
        //     });
        //     t.join().unwrap();
        // }
    }
    
}
