use std::net::IpAddr::V4;

#[test]
fn sample_func1() {
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("第三个元素是 {}", third);
    match v.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("去你的第三个元素，根本没有！"),
    }
}

#[test]
fn sample_func2() {
    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}

#[test]
fn sample_func3() {
    // let mut v = vec![1, 2, 3, 4, 5];
    //
    // let first = &v[0];
    //
    // v.push(6);
    //
    // println!("The first element is: {first}");
}

#[test]
fn sample_func4() {
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 10
    }
    for i in &v {
        println!("{i}");
    }
}

#[test]
fn sample_func5() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }
    fn main() {
        let v = vec![
            IpAddr::V4("127.0.0.1".to_string()),
            IpAddr::V6("::1".to_string()),
        ];
        for ip in v {
            show_addr(ip)
        }
    }
    fn show_addr(ip: IpAddr) {
        println!("{:?}", ip);
    }

    main();
}

#[test]
fn sample_func6() {
    trait IpAddr {
        fn display(&self);
    }
    struct V4(String);
    impl IpAddr for V4 {
        fn display(&self) {
            println!("ipv4 {:?}", self.0);
        }
    }

    struct V6(String);
    impl IpAddr for V6 {
        fn display(&self) {
            println!("ipv6: {:?}", self.0)
        }
    }
    fn main() {
        let v: Vec<Box<dyn IpAddr>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::1".to_string())),
        ];

        for ip in v {
            ip.display();
        }
    }
    main();
}

#[test]
fn sample_func7() {
    let v = vec![0; 3]; // 默认值为 0，初始长度为 3
    let v_from = Vec::from([0, 0, 0]);
    assert_eq!(v, v_from);
}

#[test]
fn sample_func8() {
    let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]);
    println!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());
    v.reserve(100);
    println!(
        "Vector（reserve） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );
    v.shrink_to_fit();
    println!(
        "Vector（shrink_to_fit） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );
}

#[test]
fn sample_func9() {
    let mut v = vec![1, 2];
    assert!(!v.is_empty());

    v.insert(2, 3);
    assert_eq!(v.remove(1), 2);
    assert_eq!(v.pop(), Some(3));
    assert_eq!(v.pop(), Some(1));
    assert_eq!(v.pop(), None);
    v.clear();

    let mut v1 = [11, 22].to_vec();
    v.append(&mut v1);
    v.truncate(1);
    v.retain(|x| *x > 10);
    let mut v = vec![11, 22, 33, 44, 55];
    let mut m: Vec<_> = v.drain(1..=3).collect();

    let v2 = m.split_off(1);
}
#[test]
fn sample_func10() {
    let v = vec![1, 2, 3, 4, 5];
    let slice = &v[1..=3];
    assert_eq!(slice, &[2, 3, 4]);
}

#[test]
fn sample_func11() {
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort_unstable();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}

#[test]
fn sample_func12() {
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}

#[test]
fn sample_func13() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn new(name: String, age: u32) -> Person {
            Person { name, age }
        }
    }

    fn main() {
        let mut people = vec![
            Person::new("Zoe".to_string(), 25),
            Person::new("Stev".to_string(), 60),
            Person::new("John".to_string(), 1),
        ];
        people.sort_unstable_by(|a, b| a.name.cmp(&b.name));

        println!("{:?}", people)
    }
    main();
}

#[test]
fn sample_func14() {
    #[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn new(name: String, age: u32) -> Person {
            Person { name, age }
        }
    }

    fn main() {
        let mut people = vec![
            Person::new("Zoe".to_string(), 25),
            Person::new("Al".to_string(), 60),
            Person::new("Al".to_string(), 30),
            Person::new("John".to_string(), 1),
            Person::new("John".to_string(), 25),
        ];

        people.sort_unstable();
        println!("{:?}", people);
    }
    main();
}

#[test]
fn test_func1() {
    fn main() {
        let arr: [u8; 3] = [1, 2, 3];

        let v = Vec::from(arr);
        is_vec(&v);

        let v = vec![1, 2, 3];
        is_vec(&v);

        // vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式，因此...
        let v = vec![1, 2, 3];
        is_vec(&v);

        // ...在下面的代码中, v 是 Vec<[u8; 3]> , 而不是 Vec<u8>
        // 使用 Vec::new 和 `for` 来重写下面这段代码
        let mut v1 = Vec::new();
        for i in &v {
            v1.push(*i)
        }

        is_vec(&v1);

        assert_eq!(format!("{:?}", v), format!("{:?}", v1));

        println!("Success!")
    }

    fn is_vec(v: &Vec<u8>) {}

    fn main2() {
        let arr: [u8; 3] = [1, 2, 3];

        let v = Vec::from(arr);
        is_vec(&v);

        let v = vec![1, 2, 3];
        is_vec(&v);

        // vec!(..) and vec![..] are same macros, so
        let v = vec![1, 2, 3];
        is_vec(&v);

        // in code below, v is Vec<[u8; 3]> , not Vec<u8>
        // USE Vec::new and `for` to rewrite the below code
        let mut v1 = vec![];
        for i in &v {
            v1.push(*i);
        }
        is_vec(&v1);

        assert_eq!(v, v1);

        println!("Success!")
    }

    main();
    main2();
}

#[test]
fn test_func2() {
    // 填空
    fn main() {
        let mut v1 = Vec::from([1, 2, 4]);
        v1.pop();
        v1.push(3);

        let mut v2 = Vec::new();
        v2.extend([1, 2, 3]);

        assert_eq!(v1, v2);

        println!("Success!")
    }
    main();
}

#[test]
fn test_func3() {
    // 填空
    fn main() {
        // array -> Vec
        // impl From<[T; N]> for Vec
        let arr = [1, 2, 3];
        let v1 = Vec::from(arr);
        let v2: Vec<i32> = arr.into();

        assert_eq!(v1, v2);

        // String -> Vec
        // impl From<String> for Vec
        let s = "hello".to_string();
        let v1: Vec<u8> = s.into();

        let s = "hello".to_string();
        let v2 = s.into_bytes();
        assert_eq!(v1, v2);

        // impl<'_> From<&'_ str> for Vec
        let s = "hello";
        let v3 = Vec::from(s);
        assert_eq!(v2, v3);

        // 迭代器 Iterators 可以通过 collect 变成 Vec
        let v4: Vec<i32> = [0; 10].into_iter().collect();
        assert_eq!(v4, vec![0; 10]);

        println!("Success!")
    }

    main();
}

#[test]
fn test_func4() {
    // 修复错误并实现缺失的代码
    fn main() {
        let mut v = Vec::from([1, 2, 3, 4, 5]);
        for i in 0..5 {
            println!("{:?}", v[i])
        }

        for i in 0..5 {
            // 实现这里的代码...
            v[i] += 1;
        }

        assert_eq!(v, vec![2, 3, 4, 5, 6]);

        println!("Success!")
    }

    main();
}

#[test]
fn test_func5() {
    // 修复错误
    fn main() {
        let mut v = vec![1, 2, 3];

        let slice1 = &v[..];
        // 越界访问将导致 panic.
        // 修改时必须使用 `v.len`
        let slice2 = &v[0..3];

        assert_eq!(slice1, slice2);

        // 切片是只读的
        // 注意：切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
        let vec_ref: &mut Vec<i32> = &mut v;
        (*vec_ref).push(4);
        let slice3 = &mut v[0..4];
        slice3[3] = 42;

        assert_eq!(slice3, &[1, 2, 3, 42]);
        assert_eq!(v, &[1, 2, 3, 42]);

        println!("Success!")
    }
    main();
}

#[test]
fn test_func6() {
    // 修复错误
    fn main() {
        let mut vec = Vec::with_capacity(10);

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 10);

        // 由于提前设置了足够的容量，这里的循环不会造成任何内存分配...
        for i in 0..10 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 10);
        assert_eq!(vec.capacity(), 10);

        // ...但是下面的代码会造成新的内存分配
        vec.push(11);
        assert_eq!(vec.len(), 11);
        assert!(vec.capacity() >= 11);

        // 填写一个合适的值，在 `for` 循环运行的过程中，不会造成任何内存分配
        let mut vec = Vec::with_capacity(100);
        for i in 0..100 {
            vec.push(i);
        }

        assert_eq!(vec.len(), 100);
        assert_eq!(vec.capacity(), 100);

        println!("Success!")
    }
}

#[test]
fn test_func7() {
    #[derive(Debug, PartialEq)]
    enum IpAddr {
        V4(String),
        V6(String),
    }
    fn main() {
        // 填空
        let v: Vec<IpAddr> = vec![
            IpAddr::V4("127.0.0.1".to_string()),
            IpAddr::V6("::1".to_string()),
        ];

        // 枚举的比较需要派生 PartialEq 特征
        assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
        assert_eq!(v[1], IpAddr::V6("::1".to_string()));

        println!("Success!")
    }
}

#[test]
fn test_func8() {
    trait IpAddr {
        fn display(&self);
    }

    struct V4(String);
    impl IpAddr for V4 {
        fn display(&self) {
            println!("ipv4: {:?}", self.0)
        }
    }
    struct V6(String);
    impl IpAddr for V6 {
        fn display(&self) {
            println!("ipv6: {:?}", self.0)
        }
    }

    fn main() {
        // 填空
        let v: Vec<Box<dyn IpAddr>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::1".to_string())),
        ];

        for ip in v {
            ip.display();
        }
    }
    main();
}
