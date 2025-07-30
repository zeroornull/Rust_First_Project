#[test]
fn test_func1() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_func4() {
    let a = "10".parse::<u32>();
    // let aa: u32 = "10".parse().unwrap(); // 这种写法也很常见
    println!("{:?}", a);

    let a = "10".parse::<f32>();
    println!("{:?}", a);

    let a = "4.2".parse::<f32>();
    println!("{:?}", a);

    let a = "true".parse::<bool>();
    println!("{:?}", a);

    let a = "a".parse::<char>();
    println!("{:?}", a);

    let a = "192.168.1.100".parse::<std::net::IpAddr>();
    println!("{:?}", a);
}

#[test]
fn test_func3() {
    let s = String::from("中国你好");
    let char_vec: Vec<char> = s.chars().collect();
    println!("{:?}", char_vec);

    for ch in s.chars() {
        println!("{:?}", ch);
    }
}

#[test]
fn test_func2() {
    // let s1: &'static str = "I am a superman.";
    // let s2: String = s1.to_string();
    // let s3: &String = &s2;
    // let s4: &str = &s2[..];
    // let s5: &str = &s2[..6];
}
