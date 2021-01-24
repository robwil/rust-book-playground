use crate::cons::CustomSmartPointer;
use crate::cons::MyBox;
use crate::cons::List::Nil;
use crate::cons::List::Cons;

mod cons;

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
    list.print();

    let val = MyBox::new(5);
    assert_eq!(5, *val);
    println!("{:?}", val);
    println!("{:?}", *val);

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}