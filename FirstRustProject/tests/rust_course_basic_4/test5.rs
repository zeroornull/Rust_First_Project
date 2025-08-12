
// fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String
}
#[test]
fn test_func1() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: "".to_string(),
    };
}


struct Unit;
trait SomeTrait {
    // ...定义一些行为
}

// 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
// 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
impl SomeTrait for Unit {  }
#[test]
fn test_func2() {
    let u = Unit;
    do_something_with_unit(u);
}

// 填空，让代码工作
fn do_something_with_unit(u: Unit) {   }


// 填空并修复错误
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
#[test]
fn test_func3() {
    let v = Point(0, 127, 255);
    check_color(v);
    fn check_color(p: Point) {
        let Point(x, _, _) = p;
        assert_eq!(x, 0);
        assert_eq!(p.1, 127);
        assert_eq!(p.2, 255);
    }
}


// 填空并修复错误，不要增加或移除代码行

#[test]
fn test_func4() {
    struct Person {
        name: String,
        age: u8,
    }
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    // how can you believe sunface is only 18? 
    p.age = 30;

    // 填空
    p.name = String::from("sunfei");
}


// 填空

#[test]
fn test_func5()  {
    struct Person {
        name: String,
        age: u8,
    }

    fn build_person(name: String, age: u8) -> Person {
        Person {
            age,
            name
        }
    }
}


// 填空，让代码工作

#[test]
fn test_func6()  {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    fn set_email(u: User) -> User {
        User {
            email: String::from("contact@im.dev"),
            ..u
        }
    }
}


// 填空，让代码工作
#[test]
fn test_func7()  {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // 打印 debug 信息到标准错误输出 stderr,并将 `30 * scale` 的值赋给 `width`
        height: 50,
    };

    dbg!(&rect1); // 打印 debug 信息到标准错误输出 stderr

    println!("{:?}", rect1); // print debug info to stdout
}


// 修复错误

#[test]
fn test_func8()  {
    #[derive(Debug)]
    struct File {
        name: String,
        data: String,
    }
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = f.name;

    println!("{}", f.data);
} 