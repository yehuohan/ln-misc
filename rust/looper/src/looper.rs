//! looper模块
//!

use std::collections::LinkedList;
use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;
use crate::handler::{Handler, HandlerMsg};


/// 使用链表存储消息
type Messages<T> = Arc<Mutex<LinkedList<HandlerMsg<T>>>>;


/// Looper对象
pub struct Looper<T>
where
    T: LooperMsg,
    T: Send + 'static
{
    msgs: Messages<T>, // 基于时间顺序的消息列表
    thread: Option<thread::JoinHandle<()>>,
}

/// Looper消息
pub trait LooperMsg {
    /// 处理looper消息
    fn handle_message(self); // looper中的消息所有权转给处理函数
}

impl<T: LooperMsg + Send + 'static> Looper<T> {
    /// 创建looper
    pub fn new() -> Looper<T> {
        let msgs = Arc::new(Mutex::new(LinkedList::new()));
        let handler = Handler::new(msgs.clone());
        let thread = thread::spawn(move || handler.run());
        Looper {
            msgs: msgs.clone(),
            thread: Some(thread),
        }
    }

    /// 向looper发送消息
    pub fn send_msg(&self, msg: T) {
        self.put_msg(HandlerMsg::new(Option::<T>::Some(msg), 0));
    }

    pub fn send_msg_delay(&self, msg: T, delay: Duration) {
        self.put_msg(HandlerMsg::new(Option::<T>::Some(msg), delay.as_millis()));
    }

    /// 发送停止运行消息
    pub fn terminate(&mut self) {
        self.put_msg(HandlerMsg::with_terminate());
        if let Some(h) = self.thread.take() { // 调用join需要JoinHandle的所有权
            h.join().unwrap();
        }
    }

    /// 根据执行时刻插入消息，使用插入排序算法
    fn put_msg(&self, hmsg: HandlerMsg<T>) {
        if let Ok(mut lst) = self.msgs.lock() {
            let mut cur = lst.cursor_front_mut();
            while let Some(msg) = cur.current() {
                if hmsg.when >= msg.when {
                    cur.move_next()
                } else {
                    break;
                }
            }
            cur.insert_before(hmsg);
        }
    }
}
