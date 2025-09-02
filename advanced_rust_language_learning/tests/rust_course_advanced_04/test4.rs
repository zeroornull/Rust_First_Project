#[cfg(test)]
mod tests {

    #[test]
    fn sample_test1() {
        // let s = String::from("hello, world");
        // // s在这里被转移给a
        // let a = Box::new(s);
        // // 报错！此处继续尝试将 s 转移给 b
        // let b = Box::new(s);
    }

    #[test]
    fn sample_test2() {
        use std::rc::Rc;

        let a = Rc::new(String::from("hello,world"));
        let b = Rc::clone(&a);

        assert_eq!(2, Rc::strong_count(&a));
        assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));
    }

    #[test]
    fn sample_test3() {
        use std::rc::Rc;

        let a = Rc::new(String::from("test ref counting"));
        println!("count after creating a = {}", Rc::strong_count(&a));
        #[allow(unused_variables)]
        let b = Rc::clone(&a);
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Rc::clone(&a);
            println!("count after creating c = {}", Rc::strong_count(&c));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    #[test]
    fn sample_test4() {
        use std::rc::Rc;
        struct Owner {
            name: String,
            // ...其它字段
        }
        struct Gadget {
            id: i32,
            owner: Rc<Owner>,
            // ...其它字段
        }

        fn main() {
            // 创建一个基于引用计数的 `Owner`.
            let gadget_owner: Rc<Owner> = Rc::new(Owner {
                name: "Gadget Man".to_string(),
            });

            // 创建两个不同的工具，它们属于同一个主人
            let gadget1 = Gadget {
                id: 1,
                owner: Rc::clone(&gadget_owner),
            };
            let gadget2 = Gadget {
                id: 2,
                owner: Rc::clone(&gadget_owner),
            };

            // 释放掉第一个 `Rc<Owner>`
            drop(gadget_owner);

            // 尽管在上面我们释放了 gadget_owner，但是依然可以在这里使用 owner 的信息
            // 原因是在 drop 之前，存在三个指向 Gadget Man 的智能指针引用，上面仅仅
            // drop 掉其中一个智能指针引用，而不是 drop 掉 owner 数据，外面还有两个
            // 引用指向底层的 owner 数据，引用计数尚未清零
            // 因此 owner 数据依然可以被使用
            println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
            println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

            // 在函数最后，`gadget1` 和 `gadget2` 也被释放，最终引用计数归零，随后底层
            // 数据也被清理释放
        }

        main();
    }

    #[test]
    fn sample_test5() {
        // use std::rc::Rc;
        // use std::thread;
        //
        // fn main(){
        //     let s = Rc::new(String::from("多线程漫游者"));
        //     for _ in 0..10{
        //         let s = Rc::clone(&s);
        //         let handle = thread::spawn(move || {
        //             println!("{}",s)
        //         });
        //     }
        // }
    }

    #[test]
    fn sample_test6() {
        use std::sync::Arc;
        use std::thread;

        fn main() {
            let s = Arc::new(String::from("多线程漫游者"));
            for _ in 0..10 {
                let s = Arc::clone(&s);
                #[allow(unused_variables)]
                let handle = thread::spawn(move || {
                    println!("{}", s);
                });
            }
        }
        
        main();
    }
}
