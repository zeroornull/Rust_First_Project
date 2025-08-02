use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use std::env;
use tokio::net::TcpListener;
use tokio::process::Command;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8888".to_string());
    println!("Listening on: {}", addr);
    let listener = TcpListener::bind(&addr).await?;

    // 注意这里是一个无条件循环，表明始终处于服务状态
    loop {
        // 等待客户端请求连上来
        let (mut stream, _) = listener.accept().await?;
        //
        let mut frame_stream = Framed::new(stream,LengthDelimitedCodec::new());

        // 来一个客户端连接，创建一个对应的新任务
        tokio::spawn(async move {
            //
            while let Some(msg) = frame_stream.next().await{
                match msg {
                    Ok(msg) => {
                        //
                        let directive = String::from_utf8(msg.to_vec())
                            .expect("error when converting to string directive.");
                        println!("{directive}");
                        let output = process(&directive).await;
                        println!("{output}");

                        //
                        _ = frame_stream.send(Bytes::from(output)).await;
                    }
                    Err(e) => {
                        println!("{e:?}")
                    }
                }
            }
        });
    }
}

async fn process(directive: &str) -> String {
    if directive == "gettime" {
        #[cfg(target_os = "windows")]
        {
            let output = Command::new("powershell")
                .arg("-Command")
                .arg("Get-Date -Format 'yyyy-MM-dd HH:mm:ss'")
                .output()
                .await
                .unwrap();
            String::from_utf8(output.stdout).unwrap()
        }
        #[cfg(not(target_os = "windows"))]
        {
            let output = Command::new("date").output().await.unwrap();
            String::from_utf8(output.stdout).unwrap()
        }
    } else {
        "invalid command".to_owned()
    }
}
