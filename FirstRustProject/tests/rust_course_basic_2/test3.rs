// 使用两种方法让代码工作起来
#[test]
fn test_func1() {
    let v = {
        let mut x = 1;
        x += 2
    };

    assert_eq!(v, ());
}
#[test]
fn test_func1_1() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
}

#[test]
fn test_func2() {
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);
}


#[test]
fn test_func3() {
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
    let s = sum(1 , 2);
    assert_eq!(s, 3);
}