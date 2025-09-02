#[cfg(test)]
mod tests {

    #[test]
    fn sample_test1() {
        use std::cell::Cell;
        fn main() {
            let c = Cell::new("asdf");
            let one = c.get();
            c.set("qwer");
            let two = c.get();
            println!("{}，{}", one, two);
        }

        main();
    }
    #[test]
    fn sample_test2() {
        use std::cell::RefCell;

        fn main() {
            let s = RefCell::new(String::from("hello, world"));
            let s1 = s.borrow();
            let s2 = s.borrow_mut();

            println!("{},{}", s1, s2);
        }

        main();
    }

    #[test]
    fn sample_test3() {
        // let x = Cell::new(1);
        // let y = &x;
        // let z = &x;
        // x.set(2);
        // y.set(3);
        // y.set(4);
        // println!("{}", x.get());
        //
        // // code snippet 2
        // let mut x = 1;
        // let y = &mut x;
        // let z = &mut x;
        // x = 2;
        // *y = 3;
        // *z = 4;
        // println!("{}", x);
    }

    #[test]
    fn sample_test4() {
        // fn main() {
        //     let x = 5;
        //     let y = &mut x;
        // }
    }

    #[test]
    fn sample_test5() {
        use std::cell::RefCell;
        pub trait Messenger {
            fn send(&self, msg: String);
        }

        pub struct MsgQueue {
            msg_cache: RefCell<Vec<String>>,
        }

        impl Messenger for MsgQueue {
            fn send(&self, msg: String) {
                self.msg_cache.borrow_mut().push(msg)
            }
        }

        fn main() {
            let mq = MsgQueue {
                msg_cache: RefCell::new(Vec::new()),
            };
            mq.send("hello,world".to_string());
        }
        main();
    }

    #[test]
    fn sample_test6() {
        use std::cell::RefCell;
        use std::rc::Rc;
        fn main() {
            let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

            let s1 = s.clone();
            let s2 = s.clone();
            //
            s2.borrow_mut().push_str(", oh yeah!");
            println!("{:?}\n{:?}\n{:?}", s, s1, s2);
        }

        main();
    }

    #[test]
    fn sample_test7() {
        use std::cell::Cell;

        fn is_even(i: i32) -> bool {
            i % 2 == 0
        }

        // fn retain_even(nums: &mut Vec<i32>) {
        //     let mut i = 0;
        //     // for num in nums.iter().filter(|&num| is_even(*num)) {
        //     //     nums[i] = *num;
        //     //     i += 1;
        //     // }
        //     for j in 0..nums.len() {
        //         if is_even(nums[j]) {
        //             nums[i] = nums[j];
        //             i += 1;
        //         }
        //     }
        //     nums.truncate(i);
        // }

        #[allow(dead_code)]
        fn retain_even(nums: &mut Vec<i32>) {
            let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..]).as_slice_of_cells();
            let mut i = 0;
            for num in slice.iter().filter(|num| is_even(num.get())) {
                slice[i].set(num.get());
                i += 1;
            }
            nums.truncate(i);
        }
    }
}
