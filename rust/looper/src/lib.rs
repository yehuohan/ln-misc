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
//! ## 示例：
//!
//! ```
//! use looper::{Looper, LooperMsg};
//! struct ExMsg { val: i32 }
//! impl LooperMsg for ExMsg {
//!     fn handle_message(self) {
//!         println!("val: {}", self.val);
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
//!     std::thread::sleep(std::time::Duration::from_secs(2));
//!     lp.terminate();
//! }
//!
//! ```
//!
//! ## TODO
//!
//! - 添加test
//!

#![feature(linked_list_cursors)]

mod looper;
mod handler;
pub use looper::{Looper, LooperMsg};
