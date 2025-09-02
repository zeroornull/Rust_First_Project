#[cfg(test)]
mod tests {
    use std::fmt::Formatter;

    #[test]
    fn sample_test1(){
        use std::fmt;
        
        struct Wrapper(Vec<String>);
        
        impl fmt::Display for Wrapper{
            fn fmt(&self,f:&mut fmt::Formatter)-> fmt::Result{
                write!(f,"[{}]",self.0.join(", "))
            }
        }
        
        fn main(){
            let w = Wrapper(vec![String::from("hello"), String::from("world")]);
            println!("w = {}", w);
        }
        
        main();
    }

    #[test]
    fn sample_test2(){
        use std::ops::Add;
        use std::fmt;
        
        struct Meters(u32);
        impl fmt::Display for Meters{
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "目标地点距离你{}米", self.0)
            }
        }
        
        impl Add for Meters{
            type Output = Self;

            fn add(self, other: Meters) -> Self {
                Self(self.0+other.0)
            }
        }
        
        fn main(){
            #[allow(unused_variables)]
            let d = calculate_distance(Meters(10),Meters(20));
            println!("{}", d);
        }
        
        fn calculate_distance(d1:Meters,d2:Meters)->Meters{
            d1+d2
        }
        
        main();
        
    }

    #[test]
    fn sample_test3(){
        
    }
}