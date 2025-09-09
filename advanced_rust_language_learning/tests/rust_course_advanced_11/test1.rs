#[cfg(test)]
mod tests {

    #[test]
    fn sample_test1(){
        
        use futures::executor::block_on;
        
        async fn do_something(){
            println!("go go go !");
        }
        
        fn main(){
            let future = do_something();
            block_on(future);
        }
        
        main();
    }
    
    #[test]
    fn sample_test2(){
        use futures::executor::block_on;

        async fn hello_world() {
            hello_cat().await;
            println!("hello, world!");
        }

        async fn hello_cat() {
            println!("hello, kitty!");
        }
        fn main() {
            let future = hello_world();
            block_on(future);
        }
        main();
    }

    #[test]
    fn sample_test3(){
        use futures::executor::block_on;
        
        struct Song{
            author:String,
            name:String,
        }
        
        async fn learn_song()->Song{
            Song{
                author:"周杰伦".to_string(),
                name: String::from("《菊花台》")
            }
        }
        
        async fn sing_song(song:Song){
            println!(
                "给大家献上一首{}的{} ~ {}",
                song.author,song.name,"菊花残，满地伤~ ~"
            );
        }
        
        async fn dance(){
            println!("唱到情深处，身体不由自主的动了起来~ ~");
        }
        
        fn main(){
            let song = block_on(learn_song());
            block_on(sing_song(song));
            block_on(dance());
        }
        
        main();
    }

    #[test]
    fn sample_test4(){
        use futures::executor::block_on;

        struct Song{
            author:String,
            name:String,
        }

        async fn learn_song() -> Song {
            Song {
                author: "曲婉婷".to_string(),
                name: String::from("《我的歌声里》"),
            }
        }

        async fn sing_song(song: Song) {
            println!(
                "给大家献上一首{}的{} ~ {}",
                song.author, song.name, "你存在我深深的脑海里~ ~"
            );
        }

        async fn dance() {
            println!("唱到情深处，身体不由自主的动了起来~ ~");
        }
        
        async fn learn_and_sing(){
            // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
            let song = learn_song().await;

            // 唱歌必须要在学歌之后
            sing_song(song).await;
        }

        fn main(){
            let song = block_on(learn_song());
            block_on(sing_song(song));
            block_on(dance());
        }

        main();
    }
    
}