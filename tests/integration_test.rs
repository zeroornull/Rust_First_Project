mod chapter1;
mod multi_file;
mod chapter3;
mod chapter4;
mod chapter5;
mod chapter6;
mod chapter7;
mod chapter8;
mod chapter9;
mod chapter10;
mod chapter11;
mod chapter12;
mod chapter13;
mod chapter14;

#[cfg(test)]
mod integration_tests {
    // use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test5() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    }
    #[test]
    fn test4() {
        // let s1 = String::from("superman 1");
        // let s2 = String::from("superman 2");
        // let s3 = String::from("superman 3");
        // let s4 = String::from("superman 4");
        //
        // let v = vec![s1, s2, s3, s4];
        // // 这里下标访问越界了
        // println!("{:?}", v[4]);
    }
    #[test]
    fn test3() {
        // 字符串字面量前面加r，表示不转义
        let raw_str = r"Escapes don't work here: \x3F \u{211D}";
        println!("{}", raw_str);

        // 这个字面量必须使用r##这种形式，因为我们希望在字符串字面量里面保留""
        let quotes = r#"And then I said: "There is no escape!""#;
        println!("{}", quotes);

        // 如果遇到字面量里面有#号的情况，可以在r后面，加任意多的前后配对的#号，
        // 只要能帮助Rust编译器识别就行
        let longer_delimiter = r###"A string with "# in it. And even "##!"###;
        println!("{}", longer_delimiter);
    }
    #[test]
    fn test2() {
        // 使用 \x 输入等值的ASCII字符（最高7位）
        let byte_escape = "I'm saying hello \x7f";
        println!("{}", byte_escape);

        // 使用 \u{} 输入等值的Unicode字符（最高24位）
        let byte_escape = "I'm saying hello \u{0065}";
        println!("{}", byte_escape);
    }
    #[test]
    fn test1() {
        // 将""号进行转义
        let byte_escape = "I'm saying \"Hello\"";
        println!("{}", byte_escape);

        // 分两行打印
        let byte_escape = "I'm saying \n 你好";
        println!("{}", byte_escape);

        // Windows下的换行符
        let byte_escape = "I'm saying \r\n 你好";
        println!("{}", byte_escape);

        // 打印出 \ 本身
        let byte_escape = "I'm saying \\ Ok";
        println!("{}", byte_escape);

        // 强行在字符串后面加个0，与C语言的字符串一致。
        let byte_escape = "I'm saying hello.\0";
        println!("{}", byte_escape);
    }
}
