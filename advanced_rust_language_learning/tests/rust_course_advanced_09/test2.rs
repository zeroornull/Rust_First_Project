#[cfg(test)]
mod tests {
    use std::arch::asm;

    #[test]
    fn sample_test1(){
        use std::arch::asm;
        
        unsafe{
            asm!("nop")
        }
    }

    #[test]
    fn sample_test2(){
        use std::arch::asm;
        
        let x:u64;
        unsafe {
            asm!("mov {},5",out(reg)x);
        }
        assert_eq!(x,5);
    }

    #[test]
    fn sample_test3(){
        use std::arch::asm;
        
        let i:u64 = 3;
        let o:u64;
        unsafe {
            asm!(
            "mov {0}, {1}",
            "add {0}, 5",
            out(reg) o,
            in(reg) i,
            );
        }
        assert_eq!(o,8);
    }

    #[test]
    fn sample_test4(){
        use std::arch::asm;
        
        let mut x:u64 = 3;
        unsafe {
            asm!("add {0}, 5",inout(reg) x);
        }
        assert_eq!(x,8);
    }

    #[test]
    fn sample_test5(){
        use std::arch::asm;
        
        let x:u64 = 3;
        let y:u64;
        
        unsafe {
            asm!("add {0}, 5",inout(reg) x => y);
        }
        assert_eq!(y,8);
    }

    #[test]
    fn sample_test6(){
        use std::arch::asm;

        let mut a: u64 = 4;
        let b: u64 = 4;
        let c: u64 = 4;
        unsafe {
            asm!(
            "add {0}, {1}",
            "add {0}, {2}",
            inout(reg) a,
            in(reg) b,
            in(reg) c,
            );
        }
        assert_eq!(a, 12);

    }

    #[test]
    fn sample_test7(){
        use std::arch::asm;
        
        let mut a:u64 = 4;
        let b:u64 =4;
        unsafe {
            asm!("add {0}, {1}",inlateout(reg) a, in(reg) b);
        }
        assert_eq!(a,8);
    }

    #[test]
    fn sample_test8(){
        use std::arch::asm;
        
        let cmd = 0xd1;
        unsafe {
            asm!("out 0x64, eax", in("eax") cmd);
        }
    }

    #[test]
    fn sample_test9(){
        use std::arch::asm;
        
        #[allow(dead_code)]
        fn mul(a:u64,b:u64)->u128{
            let lo:u64;
            let hi:u64;
            
            unsafe {
                asm!(
                // The x86 mul instruction takes rax as an implicit input and writes
                // the 128-bit result of the multiplication to rax:rdx.
                "mul {}",
                in(reg) a,
                inlateout("rax") b => lo,
                lateout("rdx") hi
                );
            }
            ((hi as u128) << 64) + lo as u128
            
        }
    }

    #[test]
    fn sample_test10(){
        // three entries of four bytes each
        let mut name_buf = [0_u8; 12];
        // String is stored as ascii in ebx, edx, ecx in order
        // Because ebx is reserved, the asm needs to preserve the value of it.
        // So we push and pop it around the main asm.
        // (in 64 bit mode for 64 bit processors, 32 bit processors would use ebx)
        
        unsafe {
            asm!(
            "push rbx",
            "cpuid",
            "mov [rdi], ebx",
            "mov [rdi + 4], edx",
            "mov [rdi + 8], ecx",
            "pop rbx",
            // We use a pointer to an array for storing the values to simplify
            // the Rust code at the cost of a couple more asm instructions
            // This is more explicit with how the asm works however, as opposed
            // to explicit register outputs such as `out("ecx") val`
            // The *pointer itself* is only an input even though it's written behind
            in("rdi") name_buf.as_mut_ptr(),
            // select cpuid 0, also specify eax as clobbered
            inout("eax") 0 => _,
            // cpuid clobbers these registers too
            out("ecx") _,
            out("edx") _,
            );
        }
        let name = core::str::from_utf8(&name_buf).unwrap();
        println!("CPU Manufacturer ID: {}",name);
    }

    #[test]
    fn sample_test11(){
        use std::arch::asm;

        // Multiply x by 6 using shifts and adds
        let mut x:u64 = 4;
        unsafe {
            asm!(
            "mov {tmp}, {x}",
            "shl {tmp}, 1",
            "shl {x},2",
            "add {x}, {tmp}",
            x = inout(reg) x,
            tmp = out(reg) _,
            );
        }
        assert_eq!(x,4*6);
    }
}