#[cfg(test)]
mod tests {
    #[test]
    fn sample_func1() {
        #[derive(Debug)]
        struct Foo;

        impl Foo {
            fn mutate_and_share(&mut self) -> &Self {
                &*self
            }
            fn share(&self) {}
        }

        fn main() {
            let mut foo = Foo;
            let loan = foo.mutate_and_share();
            println!("{:?}", loan);
            foo.share();
        }
        main();
    }

    #[test]
    fn sample_func2() {
        // #![allow(unused)]
        // fn main() {
        //     use std::collections::HashMap;
        //     use std::hash::Hash;
        //     fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m V
        //     where
        //         K: Clone + Eq + Hash,
        //         V: Default,
        //     {
        //         match map.get_mut(&key) {
        //             Some(value) => value,
        //             None => {
        //                 map.insert(key.clone(), V::default());
        //                 map.get_mut(&key).unwrap()
        //             }
        //         }
        //     }
        // }
    }

    #[test]
    fn sample_func3() {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        #[allow(dead_code)]
        impl<'a: 'b, 'b> ImportantExcerpt<'a> {
            fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }
    }

    #[test]
    fn sample_func4() {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Point {
            fn move_to(&mut self, x: i32, y: i32) {
                self.x = x;
                self.y = y;
            }
        }

        fn main() {
            let mut p = Point { x: 0, y: 0 };
            let r = &mut p;
            let rr: &Point = &*r;
            println!("{:?}", rr);
            r.move_to(10, 10);
            println!("{:?}", r);
        }
        main();
    }

    #[test]
    fn sample_func5() {
        
        struct Manager<'a>{
            text: &'a str,
        }
        #[allow(dead_code)]
        struct Interface<'b,'a:'b>{
            manager:&'b mut Manager<'a>
        }
        
        impl<'b, 'a: 'b> Interface<'b, 'a>{
            pub fn noop(self){
                println!("interface consumed");
            }
        }
        
        struct List<'a>{
            manager: Manager<'a>
        }
        
        impl<'a> List<'a>{
            pub fn get_interface<'b>(&'b mut self) -> Interface<'b,'a>
            where 'a:'b{
                Interface{
                    manager:&mut self.manager
                }
            }
        }
        
        fn main(){
            let mut list = List{
                manager:Manager{
                    text:"hello"
                }
            };
            
            list.get_interface().noop();
            
            println!("Interface should be dropped here and the borrow released");
            
            //
            //
            use_list(&list);
        }
        
        fn use_list(list:&List){
            println!("{}",list.manager.text)
        }
        
        main();
    }
}
