#[test]
fn sample_func1() {
    use std::fs::File;

    fn main() {
        let f = File::open("tests/sample_func1.rs").unwrap();
    }
}

#[test]
fn sample_func2() {
    // use std::fs::File;
    //
    // fn main() {
    //     let f = File::open("tests/sample_func2.rs").unwrap();
    //     let f = match f {
    //         Ok(file) => file,
    //         Err(error) => {
    //             panic!("Problem opening the file: {:?}", error)
    //         },
    //     };
    // }
}

#[test]
fn sample_func3() {
    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
        let f = File::open("hello.txt");

        // let f = match f {
        //     Ok(file) => file,
        //     Err(error) => match error.kind() {
        //         ErrorKind::NotFound => match File::create("hello.txt") {
        //             Ok(fc) => fc,
        //             Err(e) => panic!("Problem creating the file: {:?}", e),
        //         },
        //         other_error => panic!("Problem opening the file: {:?}", other_error),
        //     },
        // };

        let f = f.unwrap_or_else(|error| match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        });
    }
}

#[test]
fn test_func1() {
    // 填空并修复错误
    use std::num::ParseIntError;

    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>();
        let n2 = n2_str.parse::<i32>();
        Ok(n1.unwrap() * n2.unwrap())
    }

    fn main() {
        let result = multiply("10", "2");
        assert_eq!(result, Ok(20));

        let result = multiply("4", "2");
        assert_eq!(result.unwrap(), 8);

        println!("Success!")
    }

    main();
}

#[test]
fn test_func2() {
    use std::num::ParseIntError;

    // 使用 `?` 来实现 multiply
    // 不要使用 unwrap !
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>()?;
        let n2 = n2_str.parse::<i32>()?;
        Ok(n1 * n2)
    }

    fn main() {
        assert_eq!(multiply("3", "4").unwrap(), 12);
        println!("Success!")
    }

    main();
}

#[test]
fn test_func3() {
    use std::fs::File;
    use std::io::{self, Read};

    fn read_file1() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    // 填空
    // 不要修改其它代码
    fn read_file2() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }

    fn main() {
        assert_eq!(
            read_file1().unwrap_err().to_string(),
            read_file2().unwrap_err().to_string()
        );
        println!("Success!")
    }

    main();
}

#[test]
fn test_func4_1() {
    use std::num::ParseIntError;

    // 使用两种方式填空: map, and then
    fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
        n_str.parse::<i32>().map(|n| n + 2)
    }

    fn main() {
        assert_eq!(add_two("4").unwrap(), 6);

        println!("Success!")
    }
    main();
}

#[test]
fn test_func4_2() {
    use std::num::ParseIntError;

    // 使用两种方式填空: map, and then
    fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
        n_str.parse::<i32>().and_then(|n| Ok(n + 2))
    }

    fn main() {
        assert_eq!(add_two("4").unwrap(), 6);

        println!("Success!")
    }
    main();
}

#[test]
fn test_func5() {
    use std::num::ParseIntError;

    // 使用 Result 重写后，我们使用模式匹配的方式来处理，而无需使用 `unwrap`
    // 但是这种写法实在过于啰嗦..
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        match n1_str.parse::<i32>() {
            Ok(n1)  => {
                match n2_str.parse::<i32>() {
                    Ok(n2)  => {
                        Ok(n1 * n2)
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }

    // 重写上面的 `multiply` ，让它尽量简洁
    // 提示：使用 `and_then` 和 `map`
    fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        n1_str.parse::<i32>().and_then(|n|{
            n2_str.parse::<i32>().map(|n2| n * n2)
        })
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n)  => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn main() {
        let twenty = multiply1("10", "2");
        print(twenty);

        // 下面的调用会提供更有帮助的错误信息
        let tt = multiply("t", "2");
        print(tt);

        println!("Success!")
    }
    
    main();
}

#[test]
fn test_func6() {
    use std::num::ParseIntError;

    // 填空
    type Res<T> = Result<T, ParseIntError>;

    // 使用上面的别名来引用原来的 `Result` 类型
    fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        })
    }

    // 同样, 这里也使用了类型别名来简化代码
    fn print(result: Res<i32>) {
        match result {
            Ok(n)  => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn main() {
        print(multiply("10", "2"));
        print(multiply("t", "2"));

        println!("Success!")
    }
    
    main();
}

