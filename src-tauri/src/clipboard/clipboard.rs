use std::io::Error;
use clipboard_master::{CallbackResult, ClipboardHandler};
use tokio::sync::mpsc::Sender;

///剪切板监视器，需要传入一个mpsc::Sender构建
/// 每当剪切板发生变化，发出一个msg
pub struct ClipboardMonitor{
    tx:Sender<String>
}
impl ClipboardMonitor{
    pub(crate) fn new(tx:Sender<String>) ->Self{
        Self{
            tx
        }
    }
}
impl ClipboardHandler for ClipboardMonitor {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        //这里要用阻塞发送的方式，因为不是异步函数
        if let Err(e) = self.tx.blocking_send("clipboard change".into()){
            println!("err：在异步上下文中调用同步发送");
        }
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: Error) -> CallbackResult {
        self.tx.blocking_send(format!("clipboard error {}",error.to_string())).unwrap();
        CallbackResult::Next
    }
}
