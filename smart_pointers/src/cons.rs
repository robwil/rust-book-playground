use std::ops::Deref;

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    pub fn print(&self) {
        match self {
            List::Cons(val, list) => {
                print!("{} ", val);
                list.print();
            },
            List::Nil => print!("fin\n")
        }
    }
}

#[derive(Debug)]
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}