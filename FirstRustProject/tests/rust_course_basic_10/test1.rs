#[test]
fn sample_func1() {
    // {
    //     let r;
    //
    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //
    //     println!("r: {}", r);
    // }

    {
        let x = 5;

        let r = &x;

        println!("r: {}", r);
    }
}

#[test]
fn sample_func2() {
    fn main() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    main();
}

#[test]
fn sample_func3() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
}

#[test]
fn sample_func4() {
    // struct ImportantExcerpt<'a> {
    //     part: &'a str,
    // }
    // fn main() {
    //     let novel = String::from("Call me Ishmael. Some years ago...");
    //     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //     let i = ImportantExcerpt {
    //         part: first_sentence,
    //     };
    //     print!("{:?}", i);
    // }
    //
    // main()
}

#[test]
fn sample_func5() {
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() { x } else { y }
    }
}
#[test]
fn test_func1() {
    /* 为 `i` 和 `borrow2` 标注合适的生命周期范围 */

    // `i` 拥有最长的生命周期，因为它的作用域完整的包含了 `borrow1` 和 `borrow2` 。
    // 而 `borrow1` 和 `borrow2` 的生命周期并无关联，因为它们的作用域没有重叠
    fn main() {
        let i = 3;
        {
            let borrow1 = &i; // `borrow1` 生命周期开始. ──┐
            //                                                │
            println!("borrow1: {}", borrow1); //              │
        } // `borrow1` 生命周期结束. ──────────────────────────────────┘
        {
            let borrow2 = &i;

            println!("borrow2: {}", borrow2);
        }
    }

    main();
}

#[test]
fn test_func3() {
    /* 添加合适的生命周期标注，让下面的代码工作 */
    fn longest<'a, 'b>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    fn main() {}
}

#[test]
fn test_func4_1() {
    /* 使用三种方法修复下面的错误  */
    fn invalid_output() -> String {
        String::from("foo")
    }

    fn main() {}
}

#[test]
fn test_func4_2() {
    fn invalid_output() -> &'static str {
        "foo"
    }
    fn main() {}
}

#[test]
fn test_func4_3() {
    fn invalid_output<'a>(s: &'a String) -> &'a String {
        s
    }
    fn main() {}
}

#[test]
fn test_func5() {
    // `print_refs` 有两个引用参数，它们的生命周期 `'a` 和 `'b` 至少得跟函数活得一样久
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {} and y is {}", x, y);
    }

    /* 让下面的代码工作 */
    fn failed_borrow<'a>() {
        let _x = 12;

        // ERROR: `_x` 活得不够久does not live long enough
        let y: &i32 = &_x;

        // 在函数内使用 `'a` 将会报错，原因是 `&_x` 的生命周期显然比 `'a` 要小
        // 你不能将一个小的生命周期强转成大的
    }

    fn main() {
        let (four, nine) = (4, 9);

        print_refs(&four, &nine);
        // 这里，four 和 nice 的生命周期必须要比函数 print_refs 长

        failed_borrow();
        // `failed_borrow`  没有传入任何引用去限制生命周期 `'a`，因此，此时的 `'a` 生命周期是没有任何限制的，它默认是 `'static`
    }
}

#[test]
fn test_func6() {
    /* 增加合适的生命周期标注，令代码正常工作 */

    // `i32` 的引用必须比 `Borrowed` 活得更久
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    // 类似的，下面两个引用也必须比结构体 `NamedBorrowed` 活得更久
    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    fn main() {
        let x = 18;
        let y = 15;

        let single = Borrowed(&x);
        let double = NamedBorrowed { x: &x, y: &y };
        let reference = Either::Ref(&x);
        let number = Either::Num(y);

        println!("x is borrowed in {:?}", single);
        println!("x and y are borrowed in {:?}", double);
        println!("x is borrowed in {:?}", reference);
        println!("y is *not* borrowed in {:?}", number);
    }

    main();
}

#[test]
fn test_func7() {
    /* 让代码工作 */

    #[derive(Debug)]
    struct NoCopyType {}

    #[derive(Debug)]
    struct Example<'a, 'b> {
        a: &'a u32,
        b: &'b NoCopyType,
    }

    fn main() {
        let var_a = 35;
        let example: Example;

        // {
        let var_b = NoCopyType {};

        /* 修复错误 */
        example = Example {
            a: &var_a,
            b: &var_b,
        };
        // }

        println!("(Success!) {:?}", example);
    }
    main();
}
#[test]
fn test_func8() {
    #[derive(Debug)]
    struct NoCopyType {}

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Example<'a, 'b> {
        a: &'a u32,
        b: &'b NoCopyType,
    }

    /* 修复函数的签名 */
    fn fix_me<'b>(foo: &Example<'_, 'b>) -> &'b NoCopyType {
        foo.b
    }

    fn main() {
        let no_copy = NoCopyType {};
        let example = Example { a: &1, b: &no_copy };
        fix_me(&example);
        println!("Success!")
    }

    main();
}

#[test]
fn test_func9() {
    /* 添加合适的生命周期让下面代码工作 */
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&'a self) -> i32 {
            3
        }
    }

    fn main() {}
}

#[test]
fn test_func10() {
    /* 移除所有可以消除的生命周期标注 */

    fn nput(x: & i32) {
        println!("`annotated_input`: {}", x);
    }

    fn pass(x: & i32) -> & i32 { x }

    fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
        x
    }

    struct Owner(i32);

    impl Owner {
        // Annotate lifetimes as in a standalone function.
        fn add_one(&mut self) { self.0 += 1; }
        fn print(&self) {
            println!("`print`: {}", self.0);
        }
    }

    struct Person<'a> {
        age: u8,
        name: &'a str,
    }

    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    fn main() {}
}
