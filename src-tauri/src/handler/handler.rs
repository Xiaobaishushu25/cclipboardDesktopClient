use crate::app_errors::AppError::{ErrorDescribe, IncompleteError};
use crate::app_errors::AppResult;
use crate::message::message::{DeviceInfo, Message};
use crate::message::message::Message::{ClipboardMessage, DeviceChangeResponseMessage, HeartPackageMessage, NoPairDeviceResponseMessage, PairCodeResponseMessage, PairDeviceInfosResponseMessage, RemovePairResponseMessage, ServerReadyResponseMessage, WorkErrorMessage};
use bytes::{Buf, BytesMut};
use std::io::Cursor;
use std::net::SocketAddr;
use std::time::Duration;
use arboard::{Clipboard};
use tokio::io::{ AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;
use crate::{Btx, DEVICE_INFO};

///该结构体有一个tx，通过tx将消息发送到管道中，在rx端监听收到的消息并处理，主要是写消息的功能
/// 至于为什么不直接用stream.write，因为stream的write和read都需要可变引用，无法同时使用，
/// 至于stream.split分离出一个读一个写，也考虑过，写这个动作需要封装出一个函数方便随时调用，
/// 但是split出的写结构是一个带有生命周期的引用，不好处理
/// 使用mpsc通道就可以做到持续read消息还能随时write消息。
/// 也使用过oneshot因为我看是单生产者单消费者，但是我用了一下不对，查资料才发现是一次性管道
pub struct Context {
    stream: TcpStream,
    // 分配一个缓冲区，用于解析消息
    buffer: BytesMut,
    // tx: Option<Sender<Vec<u8>>>,
    tx: Option<Sender<BytesMut>>,
    ui_tx: Option<Btx>,
    //存服务端发回来的addr
    remote_socket_addr:Option<SocketAddr>,
    // clip_rx: Receiver<String>,
    clipboard:Clipboard
}
impl Context {
    pub(crate) fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(512),
            tx: None,
            ui_tx: None,
            remote_socket_addr:None,
            // clip_rx: rx,
            clipboard:Clipboard::new().expect("创建剪切板失败")
        }
    }
    pub async fn start_work(&mut self,ui_tx:Btx,mut ui_rx:Receiver<Message>, mut clip_rx:Receiver<String>) {
        // 创建一个定时器，每隔45秒发送一次心跳
        let mut heartbeat_interval = time::interval(Duration::from_secs(25));
        // 用于保存心跳任务的句柄
        let mut heartbeat_task: Option<JoinHandle<_>> = None;
        let (tx, mut rx) = tokio::sync::mpsc::channel(20);
        let heart_sender = tx.clone();
        self.tx = Some(tx);
        self.ui_tx = Some(ui_tx);
        loop {
            tokio::select! {
                message = rx.recv() => {
                    match message{
                        Some(msg) => {
                            let _ = &self.stream.write_all(msg.as_ref()).await;
                            let _ = &self.stream.flush().await;
                        }
                        None => {
                            println!("receiver没有消息");
                        }
                    }
                }
                message = clip_rx.recv() => {
                    if let Some(msg) = message {
                        match self.clipboard.get_text(){
                            Ok(text) => {
                                println!("由剪切板管道收到消息{msg}，准备发送{text}");
                                // &self.stream.write_all(ClipboardMessage(text).encode().as_slice()).await;
                                let _ = &self.stream.write_all(ClipboardMessage(text).encode().as_ref()).await;
                                let _ = &self.stream.flush().await;
                            }
                            Err(e) => {
                                println!("获取剪切板内容出错：{}",e);
                            }
                        }
                    }
                }
                message = ui_rx.recv() => {
                    match message{
                        Some(msg) => {
                            println!("由ui管道收到消息{:?}",msg);
                            self.send_message_to_socket(msg).await;
                            // println!("由ui管道收到消息{:?}，准备处理",msg);
                            // self.handle_message(msg).await;
                        }
                        None => {println!("没有收到消息")}
                    }
                }
                message = self.read_message() =>{
                    match message {
                        Ok(om) => {
                            match om {
                                None => { //说明对面在连接到自己的流关了
                                    self.send_message_to_ui(WorkErrorMessage()).await;
                                    //这里要睡眠几秒，我发现不sleep的话这个消息在main的相关rx接不到
                                    //可能break后self被马上清理了，send不出去
                                    //还有另一种更极端的情况：
                                    // 由tcp收到消息msg ServerReadyResponseMessage(127.0.0.1:44126)后马上断开连接
                                    // -此时ui的invoke('get_self_info')可以获取到不会知道服务器错误
                                    //但是start_listen函数刚初始化还没开始监听，而这边的send_message_to_ui(WorkErrorMessage())
                                    //已经发出去了，导致ui无法知道服务器已经断开了
                                    //我也尝试过在self.send_message_to_ui(WorkErrorMessage()).await;上面加睡眠但没用
                                    //先不处理了
                                    tokio::time::sleep(Duration::from_millis(3)).await;
                                    println!("断开连接");
                                    let _ = self.stream.shutdown().await;
                                    // self.stream.poll_read_ready().await.is_ok();
                                    // return;
                                    break;
                                }
                                Some(message) => {
                                    //info!("由tcp收到消息msg {:?}",message);
                                    self.handle_message(message).await;
                                }
                            }
                        }
                        Err(e) => {
                            //对面被强制关闭时会走这里start_work error:io::Error:`远程主机强迫关闭了一个现有的连接。 (os error 10054)`
                            //start_work error:io::Error:`你的主机中的软件中止了一个已建立的连接。 (os error 10053)`
                            self.send_message_to_ui(WorkErrorMessage()).await;
                            let _ = self.stream.shutdown().await;
                            println!("start_work error:{e}");
                            break;
                        }
                    }
                }
                _ =  heartbeat_interval.tick() => {
                    // 取消前一个心跳任务（如果有的话）
                    if let Some(task) = heartbeat_task.take() {
                        task.abort();
                    }
                    let new_heart_sender = heart_sender.clone();
                    // 创建一个新的心跳任务
                    heartbeat_task = Some(tokio::spawn(async move {
                    // 发送心跳消息
                    new_heart_sender.send(HeartPackageMessage().encode()).await.unwrap();
                    }));
                }
            }
        }
    }
    async fn read_message(&mut self) -> AppResult<Option<Message>> {
        loop {
            // println!("buffer.len{}",self.buffer.len());
            if let Some(message) = self.parse_message().await? {
                return Ok(Some(message));
            }
            //when read() returns Ok(0), this signifies that the stream is closed.
            // Any further calls to read() will complete immediately with Ok(0).
            // With TcpStream instances, this signifies that the read half of the socket is closed.
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                println!("即将发生错误 buffer.len{}",self.buffer.len());
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    //对等方重置连接 ，不懂什么意思，mini-redis抄的
                    //意味着在发送数据帧的过程中，远程端关闭了连接。这是一个非干净的关闭
                    Err(ErrorDescribe("connection reset by peer".into()))
                };
            }
        }
    }
    async fn parse_message(&mut self) -> AppResult<Option<Message>> {
        // 创建 Cursor 类型 ，把当前缓冲区包起来
        let mut buf = Cursor::new(&self.buffer[..]);
        match Message::check_entire_message(&mut buf) {
            Ok(_) => {
                //是一个完整的消息，记录当前数据长度
                let len = buf.position();
                //check_entire_message中调了游标位置，要调回来，下面解码要用
                buf.set_position(0);
                // let x = buf.get_ref();
                // let len = x.len();
                // let message = Message::decode(x.to_vec())?;
                let message = Message::decode(&mut buf)?;
                //将游标向后推len，圣经说是清空缓冲区的作用，我不理解
                self.buffer.advance(len as usize);
                //advance完内容长度变为0了，具体为什么还不知道
                Ok(Some(message))
            }
            Err(IncompleteError) => {
                // println!("目前缓冲区有 {}", self.buffer.len());
                // println!("不是完整的消息");
                Ok(None)
                // Err("parse incomplete message".into())
            }
            Err(e) => {
                println!("这是什么error？ {}", e);
                Err(e)
            } // _ => {}
        }
    }
    async fn handle_message(&mut self, message: Message) {
        match message {
            ServerReadyResponseMessage(addr) => {
                DEVICE_INFO.get_or_init(||{
                    DeviceInfo::new(addr,whoami::devicename())
                });
                //info!("初始化设备信息完成{:?}",DEVICE_INFO.get());
                self.remote_socket_addr = Some(addr);
                // DEVICE_INFO.get_or_init(||{
                //     DeviceInfo::new(addr,whoami::devicename())
                // });
                self.send_message_to_ui(ServerReadyResponseMessage(addr)).await;
                // self.send_message(PairRequestMessage("104575".into(),DeviceInfo::new(self.remote_socket_addr.unwrap(),whoami::devicename()))).await;
                // self.send_message(PairCreateMessage(DEVICE_INFO::new(whoami::devicename()))).await;
                // self.send_message(PairRequestMessage("sd".to_string()))
                //     .await;
                // self.send_message(ClipboardMessage("niasd".to_string()))
                //     .await;
            }
            NoPairDeviceResponseMessage() => {
                self.send_message_to_ui(NoPairDeviceResponseMessage()).await;
                println!("没有配对的设备");
            }
            PairDeviceInfosResponseMessage(_) => {
                // println!("当前配对设备{:?}",list);
                self.send_message_to_ui(message).await;
            }
            PairCodeResponseMessage(_) => {
                println!("获得了配对码");
                self.send_message_to_ui(message).await;
            }
            ClipboardMessage(content) => {
                // println!("收到了消息ClipboardMessage{:?}", content);
                if let Ok(text) = self.clipboard.get_text(){
                    if text!=content { //待同步内容和当前剪切板内容一样就不修改剪切板了
                        match self.clipboard.set_text(content){
                            Ok(_) => {}
                            Err(e) => {
                                //Result::unwrap()` on an `Err` value: Unknown { .. } - "Unknown error
                                // while interacting with the clipboard: Could not place the specified text to the clipboard"
                                println!("设置剪切板出错：{}",e.to_string())
                            }
                        }
                    }
                }else {  //获取不到当前剪切板内容就不比较了，直接尝试设置
                    if let Err(e) = self.clipboard.set_text(content){
                        println!("设置剪切板出错：{}",e.to_string())
                    }
                }
            }
            DeviceChangeResponseMessage(_,_) => {
                println!("有设备变化！！");
                self.send_message_to_ui(message).await;
            }
            RemovePairResponseMessage() =>{
                println!("您已被取消配对"); //不需要重新work哦！
                self.send_message_to_ui(message).await;
            }
            _ => {}
        }
    }
    ///通过将消息发送到管道中，在rx端监听收到的消息并再写入socket
    pub async fn send_message_to_socket(&mut self, message: Message) {
        println!("准备发送给服务器{:?}",message);
        self.tx
            .as_ref()
            .unwrap()
            .send(message.encode())
            .await
            .unwrap();
    }
    pub async fn send_message_to_ui(&mut self, message: Message) {
        println!("准备发送给前端{:?}",message);
        self.ui_tx
            .as_ref()
            .unwrap()
            .send(message)
            .unwrap();
    }
}
