#[cfg(test)]
mod tests {

    #[test]
    fn sample_test1() {
        fn main() {
            let s1 = Some("some1");
            let s2 = Some("some2");
            let n: Option<&str> = None;

            let o1: Result<&str, &str> = Ok("ok1");
            let o2: Result<&str, &str> = Ok("ok2");
            let e1: Result<&str, &str> = Err("error1");
            let e2: Result<&str, &str> = Err("error2");

            assert_eq!(s1.or(s2), s1); // Some1 or Some2 = Some1
            assert_eq!(s1.or(n), s1); // Some or None = Some
            assert_eq!(n.or(s1), s1); // None or Some = Some
            assert_eq!(n.or(n), n); // None1 or None2 = None2

            assert_eq!(o1.or(o2), o1); // Ok1 or Ok2 = Ok1
            assert_eq!(o1.or(e1), o1); // Ok or Err = Ok
            assert_eq!(e1.or(o1), o1); // Err or Ok = Ok
            assert_eq!(e1.or(e2), e2); // Err1 or Err2 = Err2

            assert_eq!(s1.and(s2), s2); // Some1 and Some2 = Some2
            assert_eq!(s1.and(n), n); // Some and None = None
            assert_eq!(n.and(s1), n); // None and Some = None
            assert_eq!(n.and(n), n); // None1 and None2 = None1

            assert_eq!(o1.and(o2), o2); // Ok1 and Ok2 = Ok2
            assert_eq!(o1.and(e1), e1); // Ok and Err = Err
            assert_eq!(e1.and(o1), e1); // Err and Ok = Err
            assert_eq!(e1.and(e2), e1); // Err1 and Err2 = Err1
        }
        main();
    }

    #[test]
    fn sample_test2() {
        fn main() {
            // or_else with Option
            let s1 = Some("some1");
            let s2 = Some("some2");
            let fn_some = || Some("some2"); // 类似于: let fn_some = || -> Option<&str> { Some("some2") };

            let n: Option<&str> = None;
            let fn_none = || None;

            assert_eq!(s1.or_else(fn_some), s1); // Some1 or_else Some2 = Some1
            assert_eq!(s1.or_else(fn_none), s1); // Some or_else None = Some
            assert_eq!(n.or_else(fn_some), s2); // None or_else Some = Some
            assert_eq!(n.or_else(fn_none), None); // None1 or_else None2 = None2

            // or_else with Result
            let o1: Result<&str, &str> = Ok("ok1");
            let o2: Result<&str, &str> = Ok("ok2");
            let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

            let e1: Result<&str, &str> = Err("error1");
            let e2: Result<&str, &str> = Err("error2");
            let fn_err = |_| Err("error2");

            assert_eq!(o1.or_else(fn_ok), o1); // Ok1 or_else Ok2 = Ok1
            assert_eq!(o1.or_else(fn_err), o1); // Ok or_else Err = Ok
            assert_eq!(e1.or_else(fn_ok), o2); // Err or_else Ok = Ok
            assert_eq!(e1.or_else(fn_err), e2); // Err1 or_else Err2 = Err2
        }
        main();
    }

    #[test]
    fn sample_test3() {
        fn main() {
            // and_then with Option
            let s1 = Some("some1");
            let s2 = Some("some2");
            let fn_some = |_| Some("some2"); // 类似于: let fn_some = |_| -> Option<&str> { Some("some2") };

            let n: Option<&str> = None;
            let fn_none = |_| None;

            assert_eq!(s1.and_then(fn_some), s2); // Some1 and_then Some2 = Some2
            assert_eq!(s1.and_then(fn_none), n); // Some and_then None = None
            assert_eq!(n.and_then(fn_some), n); // None and_then Some = None
            assert_eq!(n.and_then(fn_none), n); // None1 and_then None2 = None1

            // and_then with Result
            let o1: Result<&str, &str> = Ok("ok1");
            let o2: Result<&str, &str> = Ok("ok2");
            let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

            let e1: Result<&str, &str> = Err("error1");
            let e2: Result<&str, &str> = Err("error2");
            let fn_err = |_| Err("error2");

            assert_eq!(o1.and_then(fn_ok), o2); // Ok1 and_then Ok2 = Ok2
            assert_eq!(o1.and_then(fn_err), e2); // Ok and_then Err = Err
            assert_eq!(e1.and_then(fn_ok), e1); // Err and_then Ok = Err
            assert_eq!(e1.and_then(fn_err), e1); // Err1 and_then Err2 = Err1
        }

        main();
    }

    #[test]
    fn sample_test4() {
        fn main() {
            let s1 = Some(3);
            let s2 = Some(6);

            let n = None;

            let fn_is_even = |x: &i8| x % 2 == 0;

            assert_eq!(s1.filter(fn_is_even), n); // Some(3) -> 3 is not even -> None

            assert_eq!(s2.filter(fn_is_even), s2); // Some(6) -> 6 is even -> Some(6)

            assert_eq!(n.filter(fn_is_even), n); // None -> no value -> None
        }
        
        main();
    }

    #[test]
    fn sample_test5() {
        fn main(){
            let s1 = Some("abcde");
            let s2 = Some(5);

            let n1: Option<&str> = None;
            let n2: Option<usize> = None;

            let o1: Result<&str, &str> = Ok("abcde");
            let o2: Result<usize, &str> = Ok(5);

            let e1: Result<&str, &str> = Err("abcde");
            let e2: Result<usize, &str> = Err("abcde");

            let fn_character_count = |s:&str| s.chars().count();
            
            assert_eq!(s1.map(fn_character_count), s2);
            assert_eq!(n1.map(fn_character_count),n2);
            
            assert_eq!(o1.map(fn_character_count), o2);
            assert_eq!(e1.map(fn_character_count), e2);
        }
        main();
    }

    #[test]
    fn sample_test6() {
        let o1: Result<&str, &str> = Ok("abcde");
        let o2: Result<&str, isize> = Ok("abcde");

        let e1: Result<&str, &str> = Err("404");
        let e2: Result<&str, isize> = Err(404);

        let fn_character_count = |s: &str| -> isize { s.parse().unwrap() }; // 该函数返回一个 isize
        
        assert_eq!(o1.map_err(fn_character_count),o2);
        assert_eq!(e1.map_err(fn_character_count),e2);
    }

    #[test]
    fn sample_test7() {
        const V_DEFAULT:u32 = 1;
        
        let s:Result<u32,()> = Ok(10);
        let n:Option<u32> = None;
        let fn_closure = |v:u32| v+2;
        
        assert_eq!(s.map_or(V_DEFAULT,fn_closure),12);
        assert_eq!(n.map_or(V_DEFAULT,fn_closure),V_DEFAULT);
    }

    #[test]
    fn sample_test8() {
        let s = Some(10);
        let n:Option<i8> = None;
        
        let fn_closure = |v:i8| v+2;
        let fn_default = || 1;
        
        assert_eq!(s.map_or_else(fn_default,fn_closure),12);
        assert_eq!(n.map_or_else(fn_default,fn_closure),1);
        
        let o = Ok(10);
        let e = Err(5);
        let fn_default_for_result = |v:i8| v+1;
        
        assert_eq!(o.map_or_else(fn_default_for_result,fn_closure),12);
        assert_eq!(e.map_or_else(fn_default_for_result,fn_closure),6);
    }

    #[test]
    fn sample_test9() {
        const ERR_DEFAULT:&str = "error message";
        let s = Some("abcde");
        let n:Option<&str>=  None;
        let o:Result<&str,&str> = Ok("abcde");
        let e:Result<&str,&str> = Err(ERR_DEFAULT);
        
        assert_eq!(s.ok_or(ERR_DEFAULT),o);
        assert_eq!(n.ok_or(ERR_DEFAULT),e);
    }

    #[test]
    fn sample_test10() {
        let s = Some("abcde");
        let n: Option<&str> = None;
        let fn_err_message = || "error message";

        let o: Result<&str, &str> = Ok("abcde");
        let e: Result<&str, &str> = Err("error message");

        assert_eq!(s.ok_or_else(fn_err_message), o); // Some(T) -> Ok(T)
        assert_eq!(n.ok_or_else(fn_err_message), e); // None -> Err(default)
    }
    
    #[test]
    fn sample_test11() {
        use std::fmt;
        use std::fmt::Formatter;
        // AppError 是自定义错误类型，它可以是当前包中定义的任何类型，在这里为了简化，我们使用了单元结构体作为例子。
        // 为 AppError 自动派生 Debug 特征
        #[derive(Debug)]
        struct AppError;

        // 为 AppError 实现 std::fmt::Display 特征
        impl fmt::Display for AppError{
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "An Error Occurred, Please Try Again!") // user-facing output
            }
        }

        // 一个示例函数用于产生 AppError 错误
        fn produce_error() -> Result<(),AppError>{
            Err(AppError)
        }
        
        match produce_error() {
            Err(e) => eprintln!("{}", e),
            _ => println!("No error"),
        }
        
        eprintln!("{:?}",produce_error())
        
        
    }

    #[test]
    fn sample_test12() {
        use std::fmt;
        use std::fmt::Formatter;
        struct AppError{
            code:usize,
            message:String,
        }

        // 根据错误码显示不同的错误信息
        impl fmt::Display for AppError{
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                let err_msg = match self.code{
                    404 => "Sorry, Can not find the Page!",
                    _ => "Sorry, something is wrong! Please Try Again!",
                };
                write!(f,"{}",err_msg)
            }
        }
        
        impl fmt::Debug for AppError{
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "AppError {{code: {}, message: {} }}",
                    self.code,self.message
                )
            }
        }
        
        fn produce_error()-> Result<(),AppError>{
            Err(AppError{
                code:404,
                message:String::from("Page not found"),
            })
        }
        
        match produce_error(){
            Err(e) => eprintln!("{}",e),// 抱歉，未找到指定的页面!
            _ => println!("No error"),
        }
        
        eprintln!("{:?}",produce_error());
        eprintln!("{:#?}",produce_error());
        // Err(
        //     AppError { code: 404, message: Page not found }
        // )
            
    }

    #[test]
    fn sample_test13() {
        // use std::fs::File;
        // use std::io;
        // 
        // #[derive(Debug)]
        // struct AppError{
        //     kind: String,    // 错误类型
        //     message: String, // 错误信息
        // }
        // 
        // // 为 AppError 实现 std::convert::From 特征，由于 From 包含在 std::prelude 中，因此可以直接简化引入。
        // // 实现 From<io::Error> 意味着我们可以将 io::Error 错误转换成自定义的 AppError 错误
        // impl From<io::Error> for AppError{
        //     fn from(error: io::Error) -> Self {
        //         AppError{
        //             kind:String::from("io"),
        //             message:error.to_string(),
        //         }
        //     }
        // }
        // 
        // fn main() -> Result<(),AppError>{
        //     let _file = File::open("nonexistent_file.txt")?;
        //     Ok(())
        // }
        // 
        // main().expect("TODO: panic message");
    }

    #[test]
    fn sample_test14() {
        // use std::fs::File;
        // use std::io::{self,Read};
        // use std::num;
        // 
        // #[derive(Debug)]
        // struct AppError{
        //     kind:String,
        //     message:String,
        // }
        // 
        // impl From<io::Error> for AppError{
        //     fn from(error: Error) -> Self {
        //         AppError{
        //             kind:String::from("io"),
        //             message:error.to_string(),
        //         }
        //     }
        // }
        // 
        // impl From<num::ParseIntError> for AppError{
        //     fn from(error: ParseIntError) -> Self {
        //         AppError{
        //             kind:String::from("parse"),
        //             message:error.to_string(),
        //         }
        //     }
        // }
        // 
        // fn main() -> Result<(), AppError> {
        //     let mut file = File::open("hello_world.txt")?;
        // 
        //     let mut content = String::new();
        //     file.read_to_string(&mut content)?;
        // 
        //     let _number: usize;
        //     _number = content.parse()?;
        // 
        //     Ok(())
        // }
        // main().expect("TODO: panic message");

    }

    #[test]
    fn sample_test15() {
        
    }
    
}
