#[cfg(test)]
mod tests {

    #[test]
    fn sample_test1() {
        let arr = [1, 2, 3];
        // for v in arr{
        //     println!("{}",v);
        // }

        for v in arr.into_iter() {
            println!("{}", v);
        }

        for i in 1..10 {
            println!("{}", i)
        }
    }

    #[test]
    fn sample_test2() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("{}", val);
        }
    }

    #[test]
    fn sample_test3() {
        let arr = [1, 2, 3];
        let mut arr_iter = arr.into_iter();

        assert_eq!(arr_iter.next(), Some(1));
        assert_eq!(arr_iter.next(), Some(2));
        assert_eq!(arr_iter.next(), Some(3));
        assert_eq!(arr_iter.next(), None);
    }

    #[test]
    fn sample_test4() {
        let values = vec![1, 2, 3];

        {
            let result = match IntoIterator::into_iter(values) {
                mut iter => loop {
                    match iter.next() {
                        Some(x) => {
                            println!("{}", x);
                        }
                        None => break,
                    }
                },
            };
            result
        }
    }

    #[test]
    fn sample_test5() {
        fn main() {
            let values = vec![1, 2, 3];
            for v in values.into_iter().into_iter() {
                println!("{}", v)
            }
        }

        main();
    }

    #[test]
    fn sample_test6() {
        let values = vec![1, 2, 3];
        for v in values.into_iter() {
            println!("{}", v);
        }

        //
        //

        let mut values = vec![1, 2, 3];
        let _values_iter = values.iter();

        //
        println!("{:?}", values);

        let mut values_iter_mut = values.iter_mut();

        if let Some(v) = values_iter_mut.next() {
            *v = 0;
        }

        println!("{:?}", values)
    }

    #[test]
    fn sample_test7() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
        println!("{:?}", v1);

        // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
        // println!("{:?}",v1_iter);
    }

    #[test]
    fn sample_test8() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn sample_test9() {
        use std::collections::HashMap;
        fn main() {
            let names = ["sunface", "sunfei"];
            let ages = [18, 18];
            let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

            println!("{:?}", folks);
        }

        main();
    }

    #[test]
    fn sample_test10() {
        #[allow(dead_code)]
        struct Shoe {
            size: u32,
            style: String,
        }

        #[allow(dead_code)]
        fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            shoes.into_iter().filter(|s| s.size == shoe_size).collect()
        }
    }

    #[test]
    fn sample_test11() {
        struct Counter {
            count: u32,
        }
        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count < 5 {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);

        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum)
    }

    #[test]
    fn sample_test12() {
        let v = vec![1u64,2,3,4,5,6];
        let val = v.iter()
            .enumerate()
            .filter(|&(idx,_)|idx%2==0)
            .map(|(_,val)|val)
            .fold(0u64,|sum,acm| sum + acm);
        println!("{}",val);
    }
}
