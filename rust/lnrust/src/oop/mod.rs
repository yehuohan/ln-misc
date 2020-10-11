//! rust的面向对象编程
//! 通过trait对象，实现一些面象对像的功能。

pub mod gui;
pub mod blog;

use gui::{Screen, Button};
use blog::Post;

pub fn run() {
    // gui
    let screen = Screen {
        components: vec![
            Box::new(Button{
                width: 50,
                height: 10,
                label: String::from("Ok"),
            }),
        ],
    };
    screen.run();

    // blog
    let mut post = Post::new();
    post.add_text("This is first blog line");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    println!("{}", post.content());
}
