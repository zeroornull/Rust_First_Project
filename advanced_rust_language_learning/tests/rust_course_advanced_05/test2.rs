#[cfg(test)]
mod tests {
    #[test]
    fn sample_test1() {
        use std::marker::PhantomPinned;
        use std::pin::Pin;
        use std::ptr::NonNull;

        struct Unmovable {
            data: String,
            slice: NonNull<String>,
            _pin: PhantomPinned,
        }

        impl Unmovable {
            fn new(data: String) -> Pin<Box<Self>> {
                let res = Unmovable {
                    data,
                    slice: NonNull::dangling(),
                    _pin: PhantomPinned,
                };
                let mut boxed = Box::pin(res);

                let slice = NonNull::from(&boxed.data);

                unsafe {
                    let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
                    Pin::get_unchecked_mut(mut_ref).slice = slice;
                }

                boxed
            }
        }

        fn main() {
            let unmoved = Unmovable::new("hello".to_string());
            //
            let still_unmoved = unmoved;
            assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data))
        }

        main();
    }

    #[test]
    fn sample_test2() {
        use ouroboros::self_referencing;

        #[self_referencing]
        struct SelfRef {
            value: String,
            #[borrows(value)]
            pointer_to_value: &'this str,
        }

        fn main() {
            let v = SelfRefBuilder {
                value: "aaa".to_string(),
                pointer_to_value_builder: |value: &String| value,
            }
            .build();

            let s = v.borrow_value();
            let p = v.borrow_pointer_to_value();
            assert_eq!(s, *p);
        }
        
        main();
    }
}
