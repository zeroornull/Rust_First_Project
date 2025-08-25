#[cfg(test)]
mod tests {
    #[test]
    fn sample_test1() {
        fn main() {
            let x = 1;
            let sum = |y| x + y;
            assert_eq!(3, sum(2))
        }
        main();
    }

    #[test]
    fn sample_test2_1() {
        use std::thread;
        use std::time::Duration;

        fn muuuuu(intensity: u32) -> u32 {
            println!("muuuuu...");
            thread::sleep(Duration::from_secs(2));
            intensity
        }

        fn workout(intensity: u32, random_number: u32) {
            if intensity < 25 {
                println!("今天活力满满，先做 {} 个俯卧撑!", muuuuu(intensity));
                println!(
                    "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
                    muuuuu(intensity)
                );
            } else if random_number == 3 {
                println!("昨天练过度了，今天还是休息下吧！");
            } else {
                println!(
                    "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
                    muuuuu(intensity)
                );
            }
        }

        fn main() {
            // 强度
            let intensity = 10;
            // 随机值用来决定某个选择
            let random_number = 7;

            // 开始健身
            workout(intensity, random_number);
        }

        main();
    }

    #[test]
    fn sample_test2_2() {
        use std::thread;
        use std::time::Duration;

        // 开始健身，好累，我得发出声音：muuuu...
        fn muuuuu(intensity: u32) -> u32 {
            println!("muuuu.....");
            thread::sleep(Duration::from_secs(2));
            intensity
        }

        fn workout(intensity: u32, random_number: u32) {
            let action = muuuuu;
            if intensity < 25 {
                println!("今天活力满满, 先做 {} 个俯卧撑!", action(intensity));
                println!(
                    "旁边有妹子在看，俯卧撑太low, 再来 {} 组卧推!",
                    action(intensity)
                );
            } else if random_number == 3 {
                println!("昨天练过度了，今天还是休息下吧！");
            } else {
                println!(
                    "昨天练过度了，今天干干有氧, 跑步 {} 分钟!",
                    action(intensity)
                );
            }
        }
        
        fn main() {
            // 强度
            let intensity = 10;
            // 随机值用来决定某个选择
            let random_number = 7;

            // 开始健身
            workout(intensity, random_number);
        }
        
        main();
    }

    #[test]
    fn sample_test2_3() {
        use std::thread;
        use std::time::Duration;
        
        fn workout(intensity:u32,random_number:u32){
            let action = || {
                println!("muuuu.....");
                thread::sleep(Duration::from_secs(2));
                intensity
            };

            if intensity < 25 {
                println!(
                    "今天活力满满，先做 {} 个俯卧撑!",
                    action()
                );
                println!(
                    "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
                    action()
                );
            } else if random_number == 3 {
                println!("昨天练过度了，今天还是休息下吧！");
            } else {
                println!(
                    "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
                    action()
                );
            }
        }


        fn main() {
            // 动作次数
            let intensity = 10;
            // 随机值用来决定某个选择
            let random_number = 7;

            // 开始健身
            workout(intensity, random_number);
        }
        
        main();
        
    }
}
