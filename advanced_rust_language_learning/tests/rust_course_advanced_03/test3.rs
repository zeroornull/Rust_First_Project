#[cfg(test)]
mod tests {
    // use std::fmt::write;

    #[test]
    fn test_func1() {
        use std::fmt;
        
        struct Point{
            x:i32,
            y:i32,
        }
        
        impl fmt::Display for Point{
            //
            fn fmt(&self,f:&mut fmt::Formatter<'_>)-> fmt::Result{
                write!(f, "The point is ({}, {})", self.x, self.y)
            }
        }
        
        fn main(){
            let origin = Point{x:0,y:0};
            //
            assert_eq!(origin.to_string(), "The point is (0, 0)");
            assert_eq!(format!("{}", origin), "The point is (0, 0)");

            println!("Success!")
        }
        
        main();
    }

    #[test]
    fn test_func2() {
        // 为了使用 `from_str` 方法, 你需要引入该特征到当前作用域中
        use std::str::FromStr;
        fn main() {
            let parsed: i32 = "5".parse().unwrap();
            let turbo_parsed = "10".parse::<i32>().unwrap();
            let from_str = i32::from_str("20").unwrap();
            let sum = parsed + turbo_parsed + from_str;
            assert_eq!(sum, 35);

            println!("Success!")
        }
        
        main();
    }

    #[test]
    fn test_func3() {
        use std::str::FromStr;
        use std::num::ParseIntError;

        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32
        }

        impl FromStr for Point {
            type Err = ParseIntError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                    .split(',')
                    .map(|x| x.trim())
                    .collect();

                let x_fromstr = coords[0].parse::<i32>()?;
                let y_fromstr = coords[1].parse::<i32>()?;

                Ok(Point { x: x_fromstr, y: y_fromstr })
            }
        }
        fn main() {
            // 使用两种方式填空
            // 不要修改其它地方的代码
            // let p = "(3,4)".parse::<Point>();
            let p = Point::from_str("(3,4)");
            assert_eq!(p.unwrap(), Point{ x: 3, y: 4} );

            println!("Success!")
        }
        
        main();
    }
    
}