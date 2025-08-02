#[test]
fn test_func1() {
    assert_eq!(2 + 2, 4);
}

macro_rules! add {
    ($a:expr,$b:expr) => {
        {
            $a+$b;
        }
    };
    //
    ($a:expr)=>{
        {
            $a
        }
    }
}
#[test]
fn test_func2() {
    let x=0;
    let sum = add!(1,2);  // 调用宏
    let sum = add!(x);
}


