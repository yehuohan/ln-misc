//! blog示例
//! 用于展示状态模式（state pattern，面向对象的一个设计模式）。


pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// 博文的内容，根据状态而定
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    /// 审核博文
    pub fn request_review(&mut self) {
        // 这里先使用take()从state字段中Some值取出，并留下None（rust不允许结构体在存空字段）；
        // 在操作完成后，将新状态放回state中。
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    /// 审核通过
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    /// 只有实现了State trait，且放在Box中的类型，才有request_review、approve方法；
    /// 这里需要获取Box<Self>的所有权，同时返回Box<dyn State>的所有权；
    /// 即从 旧状态 转到 新状态；
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    /// 在不同状态，blog的内容不同
    /// 例如，未通过审核前，blog的内容应当为空
    #[allow(unused_variables)]
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    /// 请求审核后，从Draft转到PendingReview
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    /// approve不会改变Draft状态
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    /// request_review不会改变PendingReview状态
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    /// 通过审核后，从PendingReview转到Published
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    /// 通过审核后，才能看到博文的内容
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
