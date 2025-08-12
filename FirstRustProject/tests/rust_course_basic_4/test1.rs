#[test]
fn example_func1() {
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());

    fn say_hello(s: &str) {
        println!("{}", s);
    }
}

#[test]
fn example_func2() {
    let mut s = String::from("Hello ");
    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);
    s.push('!');
    println!("追加字符串 push() -> {}", s);
}

#[test]
fn test_func1() {
    // 修复错误，不要新增代码行
    let s: &str = "hello, world";
}
#[test]
fn test_func2_1() {
    let s: Box<str> = "hello, world".into();
    greetings(&s);

    fn greetings(s: &str) {
        println!("{}", s)
    }
}

#[test]
fn test_func2_2() {
    let s: Box<&str> = "hello, world".into();
    greetings(*s);

    fn greetings(s: &str) {
        println!("{}", s)
    }
}

// 填空
#[test]
fn test_func3() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
}


// 修复所有错误，并且不要新增代码行
#[test]
fn test_func4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s)
}


// 填空
#[test]
fn test_func5() {
    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats")
}


// 修复所有错误，不要删除任何一行代码
#[test]
fn test_func6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}


// 使用至少两种方法来修复错误
#[test]
fn test_func7_1() {
    let s = String::from("hello, world");
    greetings(s);

    fn greetings(s: String) {
        println!("{}",s)
    }
}

#[test]
fn test_func7_2() {
    let s = "hello, world".to_string();
    greetings(s);

    fn greetings(s: String) {
        println!("{}",s)
    }
}


// 使用两种方法来解决错误，不要新增代码行
#[test]
fn test_func8_1() {
    let s = "hello, world".to_string();
    let s1: &str = &s;
}

#[test]
fn test_func8_2() {
    let s = "hello, world";
    let s1: &str = s;
}


#[test]
fn test_func9() {
    // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
    // 填空以输出 "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // 也可以使用 Unicode 形式的转义字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    // 还能使用 \ 来连接多行字符串
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

/* 填空并修复所有错误 */
#[test]
fn test_func10() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    // modify above line to make it work
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"")
}


#[test]
fn test_func11() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
    assert_eq!(h, "h");

    let h1 = &s1[3..6];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
    assert_eq!(h1, "中");
}


#[test]
fn test_func12() {
    // 填空，打印出 "你好，世界" 中的每一个字符
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}


#[test]
fn test_func13() {
    use utf8_slice;
    let s = "The 🚀 goes to the 🌑!";

    let rocket = utf8_slice::slice(s, 4, 5);
    // 结果是 "🚀"
}