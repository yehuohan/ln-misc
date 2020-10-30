//! looper模块
//!

use std::collections::LinkedList;
use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;
use crate::handler::{Handler, HandlerMsg};


/// 消息队列
type Messages<T> = Arc<(Mutex<LinkedList<HandlerMsg<T>>>, Condvar)>;


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
        //let msgs = Arc::new(Mutex::new(LinkedList::new()));
        let msgs = Arc::new((Mutex::new(LinkedList::new()), Condvar::new()));
        let mut handler = Handler::new(msgs.clone());
        let thread = thread::spawn(move || handler.run());
        Looper {
            msgs: msgs.clone(),
            thread: Some(thread),
        }
    }

    /// 向looper发送消息
    pub fn send_msg(&self, msg: T) {
        self.put_msg(HandlerMsg::new(Option::<T>::Some(msg), Duration::from_millis(0)));
    }

    /// 向looper发送延时消息
    /// - delay: 延时时间
    pub fn send_msg_delay(&self, msg: T, delay: Duration) {
        self.put_msg(HandlerMsg::new(Option::<T>::Some(msg), delay));
    }

    /// 发送停止运行消息
    pub fn terminate(&mut self) {
        self.put_msg(HandlerMsg::with_terminate());
        if let Some(h) = self.thread.take() { // 调用join需要JoinHandle的所有权
            h.join().unwrap();
        }
    }

    /// 根据执行时刻插入消息，使用插入排序算法；
    /// 有新消息插入时，会通知handler更新wait时间。
    fn put_msg(&self, hmsg: HandlerMsg<T>) {
        let (lock, cvar) = &*self.msgs;
        if let Ok(mut lst) = lock.lock() {
            // 获取锁，插入消息
            let mut cur = lst.cursor_front_mut();
            while let Some(msg) = cur.current() {
                if hmsg.when >= msg.when { // 消息是按时间排序的，依次比较即可
                    cur.move_next()
                } else {
                    break;
                }
            }
            cur.insert_before(hmsg);

            #[cfg(feature = "debug")]
            println!("cvar notify");

            cvar.notify_one();
        }
    }
}
