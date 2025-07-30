#[test]
fn test_func1() {
    assert_eq!(2 + 2, 4);
}
#[test]
fn test_func30() {
    let s1 = 1u32;
    let s2 = 2u32;
    let s3 =  3u32;
    let s4 = 4u32;

    let v = vec![s1,s2,s3,s4];
    let a = v[0];
    // let a = &v[0];

}
#[test]
fn test_func29() {
    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
    let s3 = String::from("ccc");
    let s4 = String::from("ddd");

    let v = vec![s1,s2,s3,s4];
    // let a = v[0];
    // let a = &v[0];
    for s in v {
        println!("{}", s);
    }

}
#[test]
fn test_func28() {
    let mut a = ["1".to_string(), "2".to_string(), "3".to_string()];

    for item in &a {
        println!("{}", item);
    }

    for item in &mut a {
        println!("{}", item);
    }

    for item in a {
        println!("{}", item);
    }

    // println!("{:?}", a);  // 你可以试试把这一句打开
    // 因为 `into_iter()` 会消耗集合所有权，因此在上面示例中我们把它放在最后去展示。
}

#[test]
fn test_func27() {
    let mut a = ["1".to_string(), "2".to_string(), "3".to_string()];
    let mut an_iter = a.iter();
    assert_eq!(Some(&"1".to_string()), an_iter.next());
    assert_eq!(Some(&"2".to_string()), an_iter.next());
    assert_eq!(Some(&"3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.iter_mut();

    assert_eq!(Some(&mut "1".to_string()), an_iter.next());
    assert_eq!(Some(&mut "2".to_string()), an_iter.next());
    assert_eq!(Some(&mut "3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.into_iter();

    assert_eq!(Some("1".to_string()), an_iter.next());
    assert_eq!(Some("2".to_string()), an_iter.next());
    assert_eq!(Some("3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());

    // println!("{:?}", a);    // 请你试试这一行有没有问题？
}
#[test]
fn test_func26() {
    let mut a = [1, 2, 3]; // 一个整数数组

    let mut an_iter = a.iter(); // 转换成第一种迭代器

    assert_eq!(Some(&1), an_iter.next());
    assert_eq!(Some(&2), an_iter.next());
    assert_eq!(Some(&3), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.iter_mut(); // 转换成第二种迭代器

    assert_eq!(Some(&mut 1), an_iter.next());
    assert_eq!(Some(&mut 2), an_iter.next());
    assert_eq!(Some(&mut 3), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.into_iter(); // 转换成第三种迭代器，并消耗掉a

    assert_eq!(Some(1), an_iter.next());
    assert_eq!(Some(2), an_iter.next());
    assert_eq!(Some(3), an_iter.next());
    assert_eq!(None, an_iter.next());

    println!("{:?}", a);
}
#[test]
fn test_func25() {
    let a: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut an_iter = a.into_iter();

    while let Some(i) = an_iter.next() {
        println!("{i}")
    }
}
#[test]
fn test_func24() {
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.err(), None);

    let x: Result<u32, &str> = Err("Nothing here");
    assert_eq!(x.err(), Some("Nothing here"));
}
#[test]
fn test_func23() {
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.ok(), Some(2));

    let x: Result<u32, &str> = Err("Nothing here");
    assert_eq!(x.ok(), None);
}
#[test]
fn test_func22() {
    let x = Some("foo");
    assert_eq!(x.ok_or(0), Ok("foo"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or(0), Err(0));
}
#[test]
fn test_func21() {
    fn stringify(x: u32) -> String {
        format!("error code: {x}")
    }
    let x: Result<u32, u32> = Ok(2);
    assert_eq!(x.map_err(stringify), Ok(2));
    let x: Result<u32, u32> = Err(13);
    assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));
}
#[test]
fn test_func20() {
    fn sq_then_to_string(x: u32) -> Result<String, &'static str> {
        x.checked_mul(x)
            .map(|sq| sq.to_string())
            .ok_or("overflowed")
    }

    assert_eq!(Ok(2).and_then(sq_then_to_string), Ok(4.to_string()));
    assert_eq!(Ok(1_000_000).and_then(sq_then_to_string), Err("overflowed"));
    assert_eq!(
        Err("not a number").and_then(sq_then_to_string),
        Err("not a number")
    );
}
#[test]
fn test_func19() {
    fn mutate(r: &mut Result<i32, i32>) {
        match r.as_mut() {
            Ok(v) => *v = 42,
            Err(e) => *e = 0,
        }
    }
    let mut x: Result<i32, i32> = Ok(2);
    mutate(&mut x);
    assert_eq!(x.unwrap(), 42);
    let mut x: Result<i32, i32> = Err(13);
    mutate(&mut x);
    assert_eq!(x.unwrap(), 0);
}

#[test]
fn test_func18() {
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.as_ref(), Ok(&2));

    let x: Result<u32, &str> = Err("Error");
    assert_eq!(x.as_ref(), Err(&"Error"));
}
#[test]
fn test_func17() {
    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_err(), false);

    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.is_err(), true);
}
#[test]
fn test_func16() {
    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_ok(), true);
    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.is_ok(), false);
}
#[test]
fn test_func15() {
    let line = "1\n2\n3\n4\n";
    for num in line.lines() {
        match num.parse::<i32>().map(|i| i * 2) {
            Ok(n) => println!("{n}"),
            Err(..) => {}
        }
    }
}
#[test]
fn test_func14() {
    fn sq_then_to_string(x: u32) -> Option<String> {
        x.checked_mul(x).map(|sq| sq.to_string())
    }
    assert_eq!(Some(2).and_then(sq_then_to_string), Some(4.to_string()));
    assert_eq!(Some(1_000_000).and_then(sq_then_to_string), None);
    assert_eq!(None.and_then(sq_then_to_string), None);
}

#[test]
fn test_func13() {
    let mut x = Some(2);
    let old = x.replace(5);
    assert_eq!(x, Some(5));
    assert_eq!(old, Some(2));

    let mut x = None;
    let old = x.replace(3);
    assert_eq!(x, Some(3));
    assert_eq!(old, None);
}

#[test]
fn test_func12() {
    let mut x = Some(2);
    let y = x.take();

    assert_eq!(x, None);
    assert_eq!(y, Some(2));

    let mut x: Option<u32> = None;
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, None);
}

#[test]
fn test_func11() {
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {}
    }
    assert_eq!(x, Some(42));
}
#[test]
fn test_func10() {
    let text: Option<String> = Some("Hello,world".to_string());
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("still can print text: {text:?}");
}

#[test]
fn test_func9() {
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none(), false);

    let x: Option<u32> = None;
    assert_eq!(x.is_none(), true);
}

#[test]
fn test_func8() {
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);
    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);
}
#[test]
fn test_func7() {
    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x, Some(&12));
    let cloned = opt_x.cloned();
    assert_eq!(cloned, Some(12));
}
#[test]
fn test_func6() {
    let maybe_some_string = Some(String::from("Hello, World!"));
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(13));

    let x: Option<&str> = None;
    assert_eq!(x.map(|s| s.len()), None);
}

#[test]
fn test_func5() {
    // Option
    let x: Option<u32> = None;
    let y: Option<u32> = Some(12);

    assert_eq!(x.unwrap_or_default(), 0);
    assert_eq!(y.unwrap_or_default(), 12);

    // Result
    let good_year_from_input = "1909";
    let bad_year_from_input = "190blarg";
    let good_year = good_year_from_input.parse().unwrap_or_default();
    let bad_year = bad_year_from_input.parse().unwrap_or_default();

    assert_eq!(1909, good_year);
    assert_eq!(0, bad_year);
}
#[test]
fn test_func4() {
    // Option
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");

    // Result
    let default = 2;
    let x: Result<u32, &str> = Ok(9);
    assert_eq!(x.unwrap_or(default), 9);

    let x: Result<u32, &str> = Err("error");
    assert_eq!(x.unwrap_or(default), default);
}

#[test]
fn test_func3() {
    // Option
    let x = Some("air");
    assert_eq!(x.unwrap(), "air");
    // Result
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.unwrap(), 2);
}
#[test]
fn test_func2() {
    // Option
    let x = Some("value");
    assert_eq!(x.expect("fruits are healthy"), "value");
    // Result
    let path = std::env::var("IMPORTANT_PATH")
        .expect("env variable `IMPORTANT_PATH` should be set by `wrapper_script.sh`");
}

// pub enum Option<T>{
//     None,
//     Some(T),
// }
//
// pub enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }
