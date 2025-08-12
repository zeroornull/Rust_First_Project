use num::range;

// 修复代码中的错误，不要新增代码行!
#[test]
fn test_func1_1() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world";
}


#[test]
fn test_func2() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // 修改数字 `8` 让代码工作
    // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
    assert!(std::mem::size_of_val(&slice) == 16);
}


#[test]
fn test_func3() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // 填空让代码工作起来
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
}


#[test]
fn test_func4() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // 填空，不要再使用 0..2
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);
}


#[test]
fn test_func5() {
    let s = "你好，世界";
    // 修改以下代码行，让代码工作起来
    let slice = &s[0..3];

    assert!(slice == "你");
}


// 修复所有错误
#[test]
fn test_func6() {
    let mut s = String::from("hello world");

    // here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // it works because `&String` can be implicitly converted to `&str, If you want know more ,this is called `Deref` 
    let letter = first_letter(&s);

    println!("the first letter is: {}", letter);

    s.clear();

    fn first_letter(s: &str) -> &str {
        &s[..1]
    }
}

