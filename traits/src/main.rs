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
}
