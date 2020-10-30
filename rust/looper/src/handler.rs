//! handler模块
//!

use std::collections::LinkedList;
use std::sync::{Arc, Mutex, Condvar};
use std::time::{SystemTime, Duration};
use crate::LooperMsg;


/// 消息队列
type Messages<T> = Arc<(Mutex<LinkedList<HandlerMsg<T>>>, Condvar)>;


/// Handler对象
pub struct Handler<T>
where
    T: LooperMsg,
    T: Send + 'static
{
    msgs: Messages<T>,
    wait: Duration,
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
    /// 执行时间（UNIX时间）
    pub when: Duration,
    looper_msg: Option<T>,
}

impl<T: LooperMsg + Send + 'static> Handler<T> {
    /// 创建handler
    pub fn new(msgs: Messages<T>) -> Handler<T> {
        Handler {
            msgs,
            wait: Duration::from_millis(1), // 初始时间很小，防止handler还未处理wait状态，looper就已经notify了
        }
    }

    /// 处理hanlder消息
    pub fn run(&mut self) {
        loop {
            if let Some(hmsg) = self.get_msg() {
                match hmsg.hmtype {
                    HandlerMsgType::Message => {
                        #[cfg(feature = "debug")]
                        println!("handle msg when: {}", hmsg.when.as_millis());

                        hmsg.looper_msg.unwrap().handle_message();
                    }
                    HandlerMsgType::Terminate => break,
                }
            }
        }
    }

    /// 取出消息进行处理
    fn get_msg(&mut self) -> Option<HandlerMsg<T>> {
        let (lock, cvar) = &*self.msgs;
        if let Ok(lst) = lock.lock() {
            #[cfg(feature = "debug")]
            println!("cvar wait");

            // 获取锁，等待下一个消息，或新消息
            if let Ok(lst) = cvar.wait_timeout(lst, self.wait) {
                let mut lst = lst.0;
                if let Some(hmsg) = lst.front() {
                    let now = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap();
                    if now >= hmsg.when {
                        let ret = lst.pop_front();
                        self.wait = if let Some(next) = lst.front() {
                            next.when - now
                        } else {
                            Duration::new(u64::MAX, 0) // 没有消息，则一直等待，直到有消息
                        };
                        return ret; // 函数返回将要处理的消息
                    } else {
                        self.wait = hmsg.when - now;
                    }
                } else {
                    self.wait = Duration::new(u64::MAX, 0); // 没有消息，则一直等待，直到有消息
                }
            }
        }
        None
    }
}

impl<T: LooperMsg + Send + 'static> HandlerMsg<T> {
    pub fn new(looper_msg: Option<T>, delay: Duration) -> HandlerMsg<T> {
        HandlerMsg {
            hmtype: HandlerMsgType::Message,
            when: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap() + delay,
            looper_msg,
        }
    }

    pub fn with_terminate() -> HandlerMsg<T> {
        HandlerMsg {
            hmtype: HandlerMsgType::Terminate,
            when: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap(),
            looper_msg: None,
        }
    }
}
