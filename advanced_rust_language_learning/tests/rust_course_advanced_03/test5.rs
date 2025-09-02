#[cfg(test)]
mod tests {
    #[test]
    fn sample_test1(){
        use num_derive::FromPrimitive;
        use num_traits::FromPrimitive;
        
        #[derive(FromPrimitive)]
        enum MyEnum{
            A=1,
            B,
            C
        }
        
        fn main(){
            let x = 2;
            
            match FromPrimitive::from_i32(x) {
                Some(MyEnum::A) => println!("Got A"),
                Some(MyEnum::B) => println!("Got B"),
                Some(MyEnum::C) => println!("Got C"),
                None            => println!("Couldn't convert {}", x),
            }

            let x = MyEnum::C as i32;

            match x.try_into() {
                Ok(MyEnum::A) => println!("a"),
                Ok(MyEnum::B) => println!("b"),
                Ok(MyEnum::C) => println!("c"),
                Err(_) => eprintln!("unknown number"),
            }
        }

        use std::convert::TryFrom;
        impl TryFrom<i32> for MyEnum{
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    x if x == MyEnum::A as i32 => Ok(MyEnum::A),
                    x if x == MyEnum::B as i32 => Ok(MyEnum::B),
                    x if x == MyEnum::C as i32 => Ok(MyEnum::C),
                    _ => Err(()),
                }
            }
        }
        
        main();
    }

    #[test]
    fn sample_test2(){
        
    }
}