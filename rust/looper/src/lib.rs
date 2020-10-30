//! # looper
//!
//! 基于rust实现的消息事件循环系统。
//!
//! - looper模块负责用户消息的插入；
//! - handler模块负责处理用户消息；
//!
//! ## 基本原理
//!
//! looper模块将用户的所有消息，按照UNIX时间顺序，插入到链表中；
//! handler模块从链表中，取出已经到时间的消息进行处理。
//!
//! ## 示例
//!
//! ```
//! use rooper::{Looper, LooperMsg};
//!
//! struct ExMsg {
//!     val: i32,
//! }
//!
//! impl LooperMsg for ExMsg {
//!     fn handle_message(self) {
//!         println!("msg val: {}", self.val);
//!     }
//! }
//!
//! fn main() {
//!     let mut lp = Looper::new();
//!     lp.send_msg_delay(ExMsg {val: 4}, std::time::Duration::from_millis(400));
//!     lp.send_msg_delay(ExMsg {val: 3}, std::time::Duration::from_millis(600));
//!     lp.send_msg_delay(ExMsg {val: 2}, std::time::Duration::from_millis(800));
//!     lp.send_msg_delay(ExMsg {val: 1}, std::time::Duration::from_millis(1000));
//!     lp.send_msg_delay(ExMsg {val: 5}, std::time::Duration::from_millis(200));
//!
//!     std::thread::sleep(std::time::Duration::from_secs(2));
//!     lp.terminate();
//! }
//!
//! ```
//!

#![feature(linked_list_cursors)]

mod looper;
mod handler;
pub use looper::{Looper, LooperMsg};


#[cfg(test)]
mod tests {
    use super::{Looper, LooperMsg};

    static mut CNT: i32 = 0;
    static mut VAL: Vec<i32> = Vec::new();
    struct ExMsg { val: i32 }

    impl LooperMsg for ExMsg {
        fn handle_message(self) {
            println!("msg val: {}", self.val);
            // looper中各个消息不会同时执行，故不用加锁
            unsafe {
                CNT += 1;
                VAL.push(self.val);
            }
        }
    }

    #[test]
    fn it_works() {
        let mut lp = Looper::new();
        lp.send_msg_delay(ExMsg {val: 4}, std::time::Duration::from_millis(400));
        lp.send_msg_delay(ExMsg {val: 2}, std::time::Duration::from_millis(800));
        lp.send_msg_delay(ExMsg {val: 3}, std::time::Duration::from_millis(600));
        lp.send_msg_delay(ExMsg {val: 1}, std::time::Duration::from_millis(1000));
        lp.send_msg_delay(ExMsg {val: 5}, std::time::Duration::from_millis(200));
        std::thread::sleep(std::time::Duration::from_secs(2));
        lp.terminate();

        unsafe {
            assert_eq!(CNT, 5);
            assert_eq!(VAL, vec![5, 4, 3, 2, 1]);
        }
    }

}
