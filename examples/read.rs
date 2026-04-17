use tokio::{
    fs::File,
    io::{self, AsyncReadExt},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 20];

    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;

    println!("The bytes: {:?}", String::from_utf8_lossy(&buffer[..n]));

    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).await?;

    println!("The bytes: {:?}", String::from_utf8_lossy(&buffer));
    Ok(())
}
