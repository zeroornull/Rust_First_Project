#[cfg(test)]
mod tests {

    #[test]
    fn sample_test1() {
        use std::ops::Sub;
        use std::sync::atomic::{AtomicU64, Ordering};
        use std::thread::{self, JoinHandle};
        use std::time::Instant;

        const N_TIMES: u64 = 10000000;
        const N_THREADS: usize = 10;

        static R: AtomicU64 = AtomicU64::new(0);

        fn add_n_times(n: u64) -> JoinHandle<()> {
            thread::spawn(move || {
                for _ in 0..n {
                    R.fetch_add(1, Ordering::Relaxed);
                }
            })
        }

        fn main() {
            let s = Instant::now();
            let mut threads = Vec::with_capacity(N_THREADS);

            for _ in 0..N_THREADS {
                threads.push(add_n_times(N_TIMES));
            }

            for thread in threads {
                thread.join().unwrap();
            }

            assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));

            println!("{:?}", Instant::now().sub(s));
        }

        main();
    }

    #[test]
    fn sample_test2() {
        use std::sync::Mutex;
        use std::sync::atomic::{AtomicU64, Ordering};

        struct Counter {
            count: u64,
        }

        fn main() {
            let n = Mutex::new(Counter { count: 0 });
            n.lock().unwrap().count += 1;

            let n = AtomicU64::new(0);

            n.fetch_add(0, Ordering::Relaxed);
        }

        main();
    }

    #[test]
    fn sample_test3() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::thread::{self, JoinHandle};

        static mut DATA: u64 = 0;
        static READY: AtomicBool = AtomicBool::new(false);

        fn reset() {
            unsafe {
                DATA = 0;
            }
            READY.store(false, Ordering::Relaxed);
        }

        fn producer() -> JoinHandle<()> {
            thread::spawn(move || {
                unsafe {
                    DATA = 100;
                }
                READY.store(true, Ordering::Release);
            })
        }

        fn consumer() -> JoinHandle<()> {
            thread::spawn(move || {
                while !READY.load(Ordering::Acquire) {}
                assert_eq!(100, unsafe { DATA });
            })
        }

        fn main() {
            loop {
                reset();
                let t_producer = producer();
                let t_consumer = consumer();
                t_producer.join().unwrap();
                t_consumer.join().unwrap();
            }
        }

        main();
    }


    #[test]
    fn sample_test4() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize,Ordering};
        use std::{hint,thread};
        
        fn main(){
            let spinlock = Arc::new(AtomicUsize::new(1));
            
            let spinlock_clone = Arc::clone(&spinlock);
            let thread = thread::spawn(move||{
                spinlock_clone.store(0,Ordering::SeqCst)
            });
            
            // 等待其他线程释放锁
            while spinlock.load(Ordering::SeqCst)!=0 {
                hint::spin_loop();
            }
            
            if let Err(panic) = thread.join(){
                println!("Thread had an error: {:?}",panic);
            }
        }
        
        main();
    }
}
