use mini_redis::{Result, client};
#[tokio::main]
async fn main() -> Result<()> {
    // 它异步地与指定的远程地址建立 TCP
    // 连接。连接建立后，返回客户端句柄。尽管操作是异步执行的，我们编写的代码看起来
    // 同步。该操作异步的唯一迹象是 等待操作员。
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result);
    Ok(())
}
