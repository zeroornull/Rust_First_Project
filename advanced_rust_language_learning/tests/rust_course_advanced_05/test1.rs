#[cfg(test)]
mod tests {

    #[test]
    fn sample_test1() {
        use std::cell::RefCell;
        use std::rc::Rc;

        #[derive(Debug)]
        enum List {
            #[allow(dead_code)]
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    List::Cons(_, item) => Some(item),
                    List::Nil => None,
                }
            }
        }

        // a -> Nil
        let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));
        println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
        println!("a指向的节点 = {:?}", a.tail());

        // b -> a
        let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
        println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
        println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
        println!("b指向的节点 = {:?}", b.tail());

        // 使 a -> b，形成环 a <-> b
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }
        println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
        println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));

        // 注意：如果尝试完整打印结构（递归 Debug），会因环而导致栈溢出
        // println!("a next item = {:?}", a.tail());
    }

    #[test]
    fn sample_test2() {
        use std::rc::Rc;

        fn main() {
            let five = Rc::new(5);

            let weak_five = Rc::downgrade(&five);

            let strong_five: Option<Rc<_>> = weak_five.upgrade();
            assert_eq!(*strong_five.unwrap(), 5);

            drop(five);

            let strong_five: Option<Rc<_>> = weak_five.upgrade();
            assert_eq!(strong_five, None)
        }

        main();
    }

    #[test]
    fn sample_test3() {
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::rc::Weak;

        struct Owner {
            name: String,
            gadgets: RefCell<Vec<Weak<Gadget>>>,
        }

        struct Gadget {
            id: i32,
            owner: Rc<Owner>,
        }

        fn main() {
            //
            //
            let gadget_owner: Rc<Owner> = Rc::new(Owner {
                name: "Gadget Man".to_string(),
                gadgets: RefCell::new(Vec::new()),
            });

            //
            let grade1 = Rc::new(Gadget {
                id: 1,
                owner: gadget_owner.clone(),
            });
            let grade2 = Rc::new(Gadget {
                id: 2,
                owner: gadget_owner.clone(),
            });
            //

            gadget_owner
                .gadgets
                .borrow_mut()
                .push(Rc::downgrade(&grade1));
            gadget_owner
                .gadgets
                .borrow_mut()
                .push(Rc::downgrade(&grade2));

            for gadget_opt in gadget_owner.gadgets.borrow().iter() {
                let gadget = gadget_opt.upgrade().unwrap();
                println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
            }

            //
            //
        }

        main();
    }

    #[test]
    fn sample_test4() {
        use std::cell::RefCell;
        use std::rc::{Rc, Weak};

        #[derive(Debug)]
        struct Node {
            #[allow(dead_code)]
            value: i32,
            parent: RefCell<Weak<Node>>,
            #[allow(dead_code)]
            children: RefCell<Vec<Rc<Node>>>,
        }

        fn main() {
            let leaf = Rc::new(Node {
                value: 3,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            });

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );

            {
                let branch = Rc::new(Node {
                    value: 5,
                    parent: RefCell::new(Weak::new()),
                    children: RefCell::new(vec![Rc::clone(&leaf)]),
                });

                *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

                println!(
                    "branch strong = {}, weak = {}",
                    Rc::strong_count(&branch),
                    Rc::weak_count(&branch),
                );

                println!(
                    "leaf strong = {}, weak = {}",
                    Rc::strong_count(&leaf),
                    Rc::weak_count(&leaf),
                );
            }

            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
            println!(
                "leaf strong = {},weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            )
        }

        main();
    }

    #[test]
    fn sample_test5() {
        // struct SelfRef<'a>{
        //     value:String,
        //
        //
        //     pointer_to_value: &'a str,
        // }
        //
        // fn main(){
        //     let s = "aaa".to_string();
        //     let v = SelfRef{
        //         value:s,
        //         pointer_to_value: &s
        //     };
        // }
    }

    #[test]
    fn sample_test6() {
        // #[derive(Debug)]
        // struct WhatAboutThis<'a>{
        //     name:String,
        //     nickname:Option<&'a str>,
        // }
        //
        // fn main(){
        //     let mut tricky = WhatAboutThis{
        //         name:"Annabelle".to_string(),
        //         nickname:None,
        //     };
        //     tricky.nickname  = Some(&tricky.name[..4]);
        //
        //     tricky
        // }
        //
        // main();
    }

    #[test]
    fn sample_test7() {
        #[derive(Debug)]
        struct WhatAboutThis<'a> {
            name: String,
            nickname: Option<&'a str>,
        }

        impl<'a> WhatAboutThis<'a> {
            fn tie_the_knot(&'a mut self) {
                self.nickname = Some(&self.name[..4]);
            }
        }

        fn main() {
            let mut tricky = WhatAboutThis {
                name: "Annablle".to_string(),
                nickname: None,
            };
            tricky.tie_the_knot();
        }

        main();
    }

    #[test]
    fn sample_test8() {
        #[derive(Debug)]
        struct SelfRef {
            value: String,
            pointer_to_value: *const String,
        }

        impl SelfRef {
            fn new(txt: &str) -> Self {
                SelfRef {
                    value: String::from(txt),
                    pointer_to_value: std::ptr::null(),
                }
            }

            fn init(&mut self) {
                let self_ref: *const String = &self.value;
                self.pointer_to_value = self_ref;
            }

            fn value(&self) -> &str {
                &self.value
            }

            fn pointer_to_value(&self) -> &String {
                assert!(
                    !self.pointer_to_value.is_null(),
                    "Test::b called without Test::init being called first"
                );
                unsafe { &*(self.pointer_to_value) }
            }
        }

        fn main() {
            let mut t = SelfRef::new("hello");
            t.init();
            //
            println!("{}, {:p}", t.value(), t.pointer_to_value());
        }

        main();
    }

    #[test]
    fn sample_test9() {
        #[derive(Debug)]
        struct SelfRef {
            value: String,
            pointer_to_value: *mut String,
        }

        impl SelfRef {
            fn new(txt: &str) -> Self {
                SelfRef {
                    value: String::from(txt),
                    pointer_to_value: std::ptr::null_mut(),
                }
            }

            fn init(&mut self) {
                let self_ref: *mut String = &mut self.value;
                self.pointer_to_value = self_ref;
            }

            fn value(&self) -> &str {
                &self.value
            }

            fn pointer_to_value(&self) -> &String {
                assert!(
                    !self.pointer_to_value.is_null(),
                    "Test::b called without Test::init being called first"
                );
                unsafe { &*(self.pointer_to_value) }
            }
        }

        fn main() {
            let mut t = SelfRef::new("hello");
            t.init();
            println!("{}, {:p}", t.value(), t.pointer_to_value());

            t.value.push_str(", world");
            unsafe {
                (&mut *t.pointer_to_value).push_str("!");
            }

            println!("{}, {:p}", t.value(), t.pointer_to_value());
        }
        
        main();
    }
}
