pub mod blog{
    use std::cell::RefCell;

    pub struct Post{
        state: Option<Box<dyn State>>,
        content: String
    }

    impl Post{
        pub fn new() -> Post{
            Post{
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }
        pub fn add_text(&mut self, text: &str){
            if let Some(s)= self.state.as_mut() {
                if s.append_status() {
                    self.content.push_str(text)git a
                }
            }
        }
        pub fn content(&self) -> &str{
            self.state.as_ref().unwrap().content(self)
        }
        pub fn request_review(&mut self){
            if let Some(s) = self.state.take(){
                self.state = Some(s.request_review());
            }
        }
        pub fn approve(&mut self){
            if let Some(s) = self.state.take(){
                self.state = Some(s.approve());
            }
        }
        pub fn reject(&mut self){
            if let Some(s) = self.state.take(){
                self.state = Some(s.reject());
            }
        }
    }

    trait State{
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, _post: &'a Post) -> &'a str{
            ""
        }
        fn append_status(&self) -> bool{
            false
        }
    }

    struct Draft{}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
           Box::new(PendingReview { num_of_calls: RefCell::new(0)})
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn append_status(&self) -> bool {
            true
        }
    }

    struct PendingReview{
        num_of_calls: RefCell<u8>
    }

    impl State for PendingReview{
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            if *self.num_of_calls.borrow() == 1{
                self.num_of_calls.replace(0);
                Box::new(Published {})
            }else{
                *self.num_of_calls.borrow_mut() += 1;
                self
            }
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft{})
        }
    }
    struct Published {}

    impl State for Published{
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
    

}