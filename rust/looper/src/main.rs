
use rooper::{Looper, LooperMsg};

struct ExMsg {
    val: i32,
}

impl LooperMsg for ExMsg {
    fn handle_message(self) {
        println!("msg val: {}", self.val);
    }
}

fn main() {
    let mut lp = Looper::new();
    lp.send_msg_delay(ExMsg {val: 4}, std::time::Duration::from_millis(400));
    lp.send_msg_delay(ExMsg {val: 3}, std::time::Duration::from_millis(600));
    lp.send_msg_delay(ExMsg {val: 2}, std::time::Duration::from_millis(800));
    lp.send_msg_delay(ExMsg {val: 1}, std::time::Duration::from_millis(1000));
    lp.send_msg_delay(ExMsg {val: 5}, std::time::Duration::from_millis(200));

    std::thread::sleep(std::time::Duration::from_secs(2));
    lp.terminate();
}
