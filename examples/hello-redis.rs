use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // set key
    client.set("hello", "world".into()).await?;

    // get key
    let result = client.get("hello").await?;

    println!("从服务器端获取的结果={:?}", result);

    Ok(())
}
