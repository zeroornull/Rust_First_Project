#[cfg(test)]
mod tests {
    #[test]
    fn sample_test1(){
        const MAX_ID:usize = usize::MAX/2;
        fn main(){
            println!("用户ID允许的最大值是{}",MAX_ID);
        }
        
        main();
    }

    #[test]
    fn sample_test2(){
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
    fn sample_test3(){
        use std::sync::atomic::{AtomicUsize,Ordering};
        static REQUEST_RECV:AtomicUsize = AtomicUsize::new(0);
        fn main(){
            for _ in 0..100{
                REQUEST_RECV.fetch_add(1,Ordering::Relaxed);
            }
            println!("当前用户请求数{:?}",REQUEST_RECV);
        }
        
        main();
    }

    #[test]
    fn sample_test4(){
        use std::sync::atomic::{Ordering,AtomicUsize};
        
        struct Factory{
            factory_id: usize,
        }
        
        static GLOBAL_ID_COUNTER:AtomicUsize=  AtomicUsize::new(0);
        const MAX_ID:usize = usize::MAX/2;
        
    }
}