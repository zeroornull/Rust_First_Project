
// 修复错误


#[test]
fn test_func1() {
    struct Array<T, const N: usize> {
        data : [T; N]
    }
    let arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 4]
        }
    ];
}


// 填空

#[test]
fn test_func2() {
    fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}