#[cfg(test)]
mod tests {
    use std::io::Error;
    use std::num::ParseIntError;

    #[test]
    fn test_func1() {
        let my_str = "hello";

        //
        #[allow(unused_variables)]
        let string1 = String::from(my_str);
        #[allow(unused_variables)]
        let string2 = my_str.to_string();
        //
        #[allow(unused_variables)]
        let string3: String = my_str.into();
    }

    #[test]
    fn test_func2() {
        fn main() {
            // impl From<bool> for i32
            let i1: i32 = false.into();
            let i2: i32 = i32::from(false);
            assert_eq!(i1, i2);
            assert_eq!(i1, 0);

            // 使用两种方式修复错误
            // 1. 哪个类型实现 From 特征 : impl From<char> for ? , 你可以查看一下之前提到的文档，来找到合适的类型
            // 2. 上一章节中介绍过的某个关键字
            #[allow(unused_variables)]
            let i3: u32 = 'a'.into();

            // 使用两种方法来解决错误
            #[allow(unused_variables)]
            let s: String = 'a'.into();

            println!("Success!")
        }

        main();
    }

    #[test]
    fn test_func3() {
        use std::fs;
        use std::io;
        use std::num;

        #[allow(dead_code)]
        enum CliError {
            IoError(io::Error),
            ParseError(num::ParseIntError),
        }

        #[allow(dead_code)]
        impl From<io::Error> for CliError {
            fn from(error: Error) -> Self {
                CliError::IoError(error)
            }
            // 实现 from 方法
        }
        #[allow(dead_code)]
        impl From<num::ParseIntError> for CliError {
            fn from(error: ParseIntError) -> Self {
                CliError::ParseError(error)
            }
            // 实现 from 方法
        }
        #[allow(dead_code)]
        fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
            // ? 自动将 io::Error 转换成 CliError
            let contents = fs::read_to_string(&file_name)?;
            // num::ParseIntError -> CliError
            let num: i32 = contents.trim().parse()?;
            Ok(num)
        }

        fn main() {
            println!("Success!")
        }

        main();
    }

    #[test]
    fn test_func4() {
        // TryFrom 和 TryInto 也被包含在 `std::prelude` 中, 因此以下引入是没必要的
        // use std::convert::TryInto;

        fn main() {
            let n: i16 = 256;

            // Into 特征拥有一个方法`into`,
            // 因此 TryInto 有一个方法是 ?
            let n: u8 = match n.try_into() {
                Ok(n) => n,
                Err(e) => {
                    println!(
                        "there is an error when converting: {:?}, but we catch it",
                        e.to_string()
                    );
                    0
                }
            };

            assert_eq!(n, 0);

            println!("Success!")
        }
        main();
    }

    #[test]
    fn test_func5() {
        #[derive(Debug, PartialEq)]
        struct EvenNum(i32);

        impl TryFrom<i32> for EvenNum {
            type Error = ();

            // 实现 `try_from`
            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value % 2 == 0 {
                    Ok(EvenNum(value))
                } else {
                    Err(())
                }
            }
        }

        fn main() {
            assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
            assert_eq!(EvenNum::try_from(5), Err(()));

            // 填空
            let result: Result<EvenNum, ()> = 8i32.try_into();
            assert_eq!(result, Ok(EvenNum(8)));
            let result: Result<EvenNum, ()> = 5i32.try_into();
            assert_eq!(result, Err(()));

            println!("Success!")
        }
        
        main();
    }
}
