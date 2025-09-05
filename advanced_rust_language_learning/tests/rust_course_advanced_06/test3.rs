#[cfg(test)]
mod tests {

    #[test]
    fn sample_test1() {
        use std::sync::mpsc;
        use std::thread;

        fn main() {
            // 创建一个消息通道, 返回一个元组：(发送者，接收者)
            let (tx, rx) = mpsc::channel();

            // 创建线程，并发送消息
            thread::spawn(move || {
                // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
                tx.send(1).unwrap();

                // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
                // tx.send(Some(1)).unwrap()
            });

            // 在主线程中接收子线程发送的消息并输出
            println!("receive {}", rx.recv().unwrap());
        }

        main();
    }

    #[test]
    fn sample_test2() {
        use std::sync::mpsc;
        use std::thread;

        fn main() {
            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                tx.send(1).unwrap();
            });

            println!("receive {:?}", rx.try_recv());
            println!("receive {:?}", rx.try_recv());
            println!("receive {:?}", rx.try_recv());
            println!("receive {:?}", rx.try_recv());
            println!("receive {:?}", rx.try_recv());
            println!("receive {:?}", rx.try_recv());
            println!("receive {:?}", rx.try_recv());
            println!("receive {:?}", rx.try_recv());
            println!("receive {:?}", rx.try_recv());
            println!("receive {:?}", rx.try_recv());
        }

        main();
    }

    #[test]
    fn sample_test3() {
        // use std::sync::mpsc;
        // use std::thread;
        //
        // fn main(){
        //     let (tx,rx) = mpsc::channel();
        //
        //     thread::spawn(move || {
        //         let s = String::from("我，飞走咯!");
        //         tx.send(s).unwrap();
        //         println!("val is {}", s);
        //     });
        //
        //     let received = rx.recv().unwrap();
        //     println!("Got: {}", received);
        // }
    }

    #[test]
    fn sample_test4() {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        fn main() {
            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
            });

            for received in rx {
                println!("Got: {}", received);
            }
        }

        main();
    }

    #[test]
    fn sample_test5() {
        use std::sync::mpsc;
        use std::thread;

        fn main() {
            let (tx, rx) = mpsc::channel();
            let tx1 = tx.clone();
            thread::spawn(move || {
                tx.send(String::from("hi from raw tx")).unwrap();
            });

            thread::spawn(move || {
                tx1.send(String::from("hi from cloned tx")).unwrap();
            });

            for received in rx {
                println!("Got:{}", received);
            }
        }
        main();
    }

    #[test]
    fn sample_test6() {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;
        fn main() {
            let (tx, rx) = mpsc::channel();

            let handle = thread::spawn(move || {
                println!("发送之前");
                tx.send(1).unwrap();
                println!("发送之前");
            });

            println!("睡眠之前");
            thread::sleep(Duration::from_secs(3));
            println!("睡眠之后");

            println!("receive {}", rx.recv().unwrap());
            handle.join().unwrap();
        }

        main();
    }

    #[test]
    fn sample_test7() {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        fn main() {
            let (tx, rx) = mpsc::sync_channel(0);

            let handle = thread::spawn(move || {
                println!("发送之前");
                tx.send(1).unwrap();
                println!("发送之后");
            });

            println!("睡眠之前");
            thread::sleep(Duration::from_secs(3));
            println!("睡眠之后");

            println!("receive {}", rx.recv().unwrap());
            handle.join().unwrap();
        }
        
        main();
    }

    #[test]
    fn sample_test8() {
        use std::sync::mpsc::{self,Receiver,Sender};
        enum Fruit{
            Apple(u8),
            Orange(String)
        }
        
        fn main(){
            let (tx,rx):(Sender<Fruit>,Receiver<Fruit>) = mpsc::channel();
            
            tx.send(Fruit::Orange("sweet".to_string())).unwrap();
            tx.send(Fruit::Apple(2)).unwrap();
            
            for _ in 0..2{
                match rx.recv().unwrap() {
                    Fruit::Apple(count) => {
                        println!("received {} apples", count)
                    }
                    Fruit::Orange(flavor) => {
                        println!("received {} oranges", flavor)
                    }
                }
            }
        }
        
        main();
    }

    #[test]
    fn sample_test9() {
        use std::sync::mpsc;
        
        fn main(){
            use std::thread;
            
            let (send,recv) = mpsc::channel();
            let num_threads = 3;
            for i in 0..num_threads{
                let thread_send = send.clone();
                thread::spawn(move ||{
                   thread_send.send(i).unwrap();
                    println!("thread {:?} finished",i);
                });
            }
            
            // 在这里drop send...
            
            for x in recv{
                println!("Got: {}",x);
            }
            
            println!("finished iterating");
        }
        
        main();

    }
    
}
