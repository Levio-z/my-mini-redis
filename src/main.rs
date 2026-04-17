use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
#[tokio::main]
async fn main() {
    // 它一直驻留在那里监听，直到程序退出。
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        // listener 代表的是“监听
        // socket（服务器入口）”，它的职责就是持续接收新的客户端连接。
        // 每调用一次 accept()，只接受一个新连接，并返回一个新的 socket
        // 代表这个连接。我们把它命名为 socket。
        // 原理
        // 握手完成后，内核把连接放进 accept queue（已连接队列）
        // 从等待队列里取出一个已经完成握手的客户端连接，并返回一个新的 socket
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        println!("Accepted");
        tokio::spawn(async move { process(socket, db).await });
    }
}

async fn process(socket: TcpStream, db: Db) -> () {
    use mini_redis::Command::{self, Get, Set};
    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => unimplemented!("{:?}", cmd),
        };

        let _ = conn.write_frame(&response).await;
    }
}
