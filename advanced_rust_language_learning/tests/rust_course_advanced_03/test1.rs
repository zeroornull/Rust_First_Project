#[cfg(test)]
mod tests {
    #[test]
    fn sample_func1() {
        let a: i32 = 10;
        let b: u16 = 100;
        if a < (b as i32) {
            println!("Ten is less than one hundred.");
        }

        let a1 = i8::MAX;
        println!("a1==={}", a1);

        let a2 = 3.1 as i8;
        let b2 = 100_i8 as i32;
        let c2 = 'a' as u8;

        println!("{},{},{}", a2, b2, c2);

        let mut values: [i32; 2] = [1, 2];
        let p1: *mut i32 = values.as_mut_ptr();
        let first_address = p1 as usize;
        let seconds_address = first_address + 4;
        let p2 = seconds_address as *mut i32;
        unsafe {
            *p2 += 1;
        }
        assert_eq!(values[1], 3);
    }

    #[test]
    fn sample_func2() {
        // use std::convert::TryInto;

        // fn main(){
        //     let a:u8 = 10;
        //     let b:u16 = 1500;
        //     let b_:u8 = b.try_into().unwrap();
        //
        //     if a<b_ {
        //         println!("Ten is less than one hundred.");
        //     }
        // }

        // fn main(){
        //     let b:i16 = 1500;
        //     let b_:u8 = match b.try_into(){
        //         Ok(b1) =>b1,
        //         Err(e)=>{
        //             println!("{:?}",e.to_string());
        //             0
        //         }
        //     };
        // }
        //
        // main();
    }

    #[test]
    fn sample_func3() {
        #[allow(dead_code)]
        struct Foo {
            x: u32,
            y: u16,
        }
        #[allow(dead_code)]
        struct Bar {
            a: u32,
            b: u16,
        }
        #[allow(dead_code)]
        fn reinterpret(foo: Foo) -> Bar {
            let Foo { x, y } = foo;
            Bar { a: x, b: y }
        }
    }

    #[test]
    fn sample_func4() {
        // trait Trait {}
        // 
        // fn foo<X: Trait>(t: X) {}
        // 
        // impl<'a> Trait for &'a i32 {}
        // 
        // fn main() {
        //     let t: &mut i32 = &mut 0;
        //     foo(t);
        // }
        // 
        // main();
    }

    #[test]
    fn test_func1() {
        // 修复错误，填空
        // 不要移除任何代码
        fn main() {
            let decimal = 97.123_f32;

            let integer: u8 = decimal as u8;

            #[allow(unused_variables)]
            let c1: char = decimal as u8 as char;
            #[allow(unused_variables)]
            let c2 = integer as char;

            assert_eq!(integer, 'b' as u8 -1);

            println!("Success!")
        }
        
        main();
    }

    #[test]
    fn test_func2() {
        // #![allow(overflowing_literals)]
        // 
        // fn main() {
        //     assert_eq!(u8::MAX, 255);
        //     let v = 1000 as u8;
        // }
        // 
        // main();
    }

    #[test]
    fn test_func3() {
        fn main() {
            assert_eq!(1000u16, 1000);

            // assert_eq!(1000 as u8, 232);

            // 事实上，之前说的规则对于正整数而言，就是如下的取模
            println!("1000 mod 256 is : {}", 1000 % 256);

            assert_eq!(-1_i8 as u8, 255);


            // 从 Rust 1.45 开始，当浮点数超出目标整数的范围时，转化会直接取正整数取值范围的最大或最小值
            assert_eq!(300.1_f32 as u8, 255);
            assert_eq!(-100.1_f32 as u8, 0);


            // 上面的浮点数转换有一点性能损耗，如果大家对于某段代码有极致的性能要求，
            // 可以考虑下面的方法，但是这些方法的结果可能会溢出并且返回一些无意义的值
            // 总之，请小心使用
            unsafe {
                // 300.0 is 44
                println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
                // -100.0 as u8 is 156
                println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
                // nan as u8 is 0
                println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
            }
        }
        
        main();
    }

    #[test]
    fn test_func4() {

        // 填空
        fn main() {
            let mut values: [i32; 2] = [1, 2];
            let p1: *mut i32 = values.as_mut_ptr();
            let first_address: usize = p1 as usize;
            let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
            let p2: *mut i32 = second_address as *mut i32; // p2 指向 values 数组中的第二个元素
            unsafe {
                // 将第二个元素加 1
                *p2+=1;
            }

            assert_eq!(values[1], 3);

            println!("Success!")
        }
        
        main();
    }

    #[test]
    fn test_func5() {
        fn main() {
            let arr :[u64; 13] = [0; 13];
            assert_eq!(size_of_val(&arr), 8 * 13);
            let a: *const [u64] = &arr;
            let b = a as *const [u8];
            unsafe {
                assert_eq!(size_of_val(&*b), 13)
            }
        }
        main();
    }


    
}
