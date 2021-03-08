// Examples taken from: https://doc.rust-lang.org/book/ch17-02-trait-objects.html

trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!(
            "Drawing rectangle at {}, {} with {} width and {} height",
            self.x, self.y, self.width, self.height
        );
    }
}

struct Button<'label> {
    width: u32,
    height: u32,
    label: &'label str,
}

impl<'a> Draw for Button<'_> {
    fn draw(&self) {
        println!(
            "Drawing button \"{}\" with {} width and {} height",
            self.label, self.width, self.height
        );
    }
}

use blog::Post;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Rectangle {
                x: 0,
                y: 0,
                width: 4,
                height: 2,
            }),
            Box::new(Button {
                width: 20,
                height: 50,
                label: "Test Button",
            }),
        ],
    };
    screen.run();

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("The blog post content is \"{}\"", post.content());
}
