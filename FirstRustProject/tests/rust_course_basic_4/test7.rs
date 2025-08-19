#[test]
fn sample_func1() {
    let one = [1,2,3];
    let two:[u8;3]=[1,2,3];
    let blank1 = [0;3];
    let blank2: [u8; 3] = [0; 3];
    let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];
    // 借用arrays的元素用作循环中
    for a in &arrays {
        print!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n+10);
        }

        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}


#[test]
fn test_func1() {
    // 使用合适的类型填空
    let arr: [u8;5] = [1, 2, 3, 4, 5];

    // 修改以下代码，让它顺利运行
    assert!(arr.len() == 5);
}


#[test]
fn test_func2() {
    // 很多时候，我们可以忽略数组的部分类型，也可以忽略全部类型，让编译器帮助我们推导
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    // 填空
    // 数组分配在栈上， `std::mem::size_of_val` 函数会返回整个数组占用的内存空间
    // 数组中的每个 char 元素占用 4 字节的内存空间，因为在 Rust 中， char 是 Unicode 字符
    assert!(std::mem::size_of_val(&arr) == 12);
}


#[test]
fn test_func3() {
    // 填空
    let list: [i32; 100] = [1;100] ;

    assert!(list[0] == 1);
    assert!(list.len() == 100);
}


#[test]
fn test_func4() {
    // 修复错误
    let _arr = [1, 2, 3];
}


#[test]
fn test_func5() {
    let arr = ['a', 'b', 'c'];

    let ele = arr[0]; // 只修改此行来让代码工作

    assert!(ele == 'a');
}


// 修复代码中的错误
#[test]
fn test_func6() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
    let name0 = names.get(0).unwrap();

    // 但是下标索引就存在越界的风险了
    let _name1 = &names[1];
}