//! handler模块
//!

use std::collections::LinkedList;
use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use std::time::{SystemTime, Duration};
use crate::LooperMsg;


/// 使用链表存储消息
type Messages<T> = Arc<Mutex<LinkedList<HandlerMsg<T>>>>;


/// Handler对象
pub struct Handler<T>
where
    T: LooperMsg,
    T: Send + 'static
{
    msgs: Messages<T>,
}

/// Handler内置消息类型
///
pub enum HandlerMsgType {
    Message,
    Terminate,
}

/// Handler消息
pub struct HandlerMsg<T>
where
    T: LooperMsg
{
    hmtype: HandlerMsgType,
    pub when: u128,
    looper_msg: Option<T>,
}

impl<T: LooperMsg + Send + 'static> Handler<T> {
    /// 创建handler
    pub fn new(msgs: Messages<T>) -> Handler<T> {
        Handler {
            msgs,
        }
    }

    /// 处理hanlder消息
    pub fn run(&self) {
        loop {
            if let Some(hmsg) = self.get_msg() {
                match hmsg.hmtype {
                    HandlerMsgType::Message => {
                        println!("hanlder msg when: {}", hmsg.when);
                        hmsg.looper_msg.unwrap().handle_message();
                    }
                    HandlerMsgType::Terminate => break,
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// 取出消息进行处理
    fn get_msg(&self) -> Option<HandlerMsg<T>> {
        if let Ok(mut lst) = self.msgs.lock() {
            if let Some(hmsg) = lst.front() {
                if SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() >= hmsg.when {
                    return lst.pop_front();
                }
            }
        }
        None
    }
}

impl<T: LooperMsg + Send + 'static> HandlerMsg<T> {
    pub fn new(looper_msg: Option<T>, delay: u128) -> HandlerMsg<T> {
        HandlerMsg {
            hmtype: HandlerMsgType::Message,
            when: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() + delay,
            looper_msg,
        }
    }

    pub fn with_terminate() -> HandlerMsg<T> {
        HandlerMsg {
            hmtype: HandlerMsgType::Terminate,
            when: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
            looper_msg: None,
        }
    }
}
