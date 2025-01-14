use oop_in_rust_17::blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());
    
    post.add_text("hi");
    
    
    
    post.approve();
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    assert_eq!("I ate a salad for lunch today", post.content());
}